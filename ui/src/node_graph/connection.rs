use crate::node_graph::{utils::draw_connection, Graph, GraphData, InputId, OutputId, PortViewState, ZoomPanState};
use egui::{Stroke, Ui};
use shine_core::slotmap::new_key_type;

new_key_type! { pub struct ConnectionId; }

pub trait ConnectionData: Clone + Send + Sync + 'static {}

pub struct Connection<C: ConnectionData> {
    id: ConnectionId,
    pub input_id: InputId,
    pub output_id: OutputId,
    pub data: C,
}

impl<C> Connection<C>
where
    C: ConnectionData,
{
    pub fn new(connection_id: ConnectionId, input_id: InputId, output_id: OutputId, data: C) -> Self {
        assert!(input_id.port_type_id() == output_id.port_type_id());

        Self {
            id: connection_id,
            input_id,
            output_id,
            data,
        }
    }

    pub fn id(&self) -> ConnectionId {
        self.id
    }

    pub(in crate::node_graph) fn show<G>(
        &self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        graph: &Graph<G>,
    ) where
        G: GraphData,
    {
        let start = port_visual.get_screen_pos(self.input_id.into());
        let end = port_visual.get_screen_pos(self.output_id.into());

        if let (Some(start), Some(end)) = (start, end) {
            let type_id = self.input_id.port_type_id();
            if let Some(style) = graph.type_styles.get(&type_id) {
                draw_connection(
                    ui.painter(),
                    start,
                    end,
                    Stroke {
                        color: style.color,
                        width: style.connection_width * zoom_pan.zoom,
                    },
                );
            } else {
                log::warn!("Skipping connection {:?}, style for {:?} not found", self.id(), type_id);
            }
        }
    }
}
