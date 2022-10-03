use crate::node_graph::{utils::draw_connection, Graph, InputId, OutputId, PortTypeId, PortViewState, ZoomPanState};
use egui::{Stroke, Ui};
use slotmap::new_key_type;

new_key_type! { pub struct ConnectionId; }

pub struct Connection {
    pub connection_id: ConnectionId,
    pub input_id: InputId,
    pub output_id: OutputId,
    pub type_id: PortTypeId,
}

impl Connection {
    pub fn new(connection_id: ConnectionId, input_id: InputId, output_id: OutputId) -> Self {
        assert!(input_id.type_id() == output_id.type_id());

        Self {
            connection_id,
            input_id,
            output_id,
            type_id: input_id.type_id(),
        }
    }

    pub(in crate::node_graph) fn show(
        &self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        graph: &Graph,
    ) {
        let start = port_visual.get_screen_pos(self.input_id.into());
        let end = port_visual.get_screen_pos(self.output_id.into());

        if let (Some(start), Some(end)) = (start, end) {
            let style = graph.get_type(self.type_id).unwrap();
            draw_connection(
                ui.painter(),
                start,
                end,
                Stroke {
                    color: style.color,
                    width: style.connection_width * zoom_pan.zoom,
                },
            );
        }
    }
}
