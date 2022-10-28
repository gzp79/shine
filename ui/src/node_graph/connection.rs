use crate::node_graph::{
    utils::draw_connection, InputId, NodeId, OutputId, PortStyle, PortStyles, PortViewState, ZoomPanState,
};
use egui::{Area, Order, Rect, Stroke, Ui};
use emath::Align2;
use shine_core::{
    downcast_rs::{impl_downcast, Downcast},
    slotmap::new_key_type,
    smallbox::{smallbox, space, SmallBox},
};

new_key_type! { pub struct ConnectionId; }

pub trait ConnectionData: 'static + Downcast + Send + Sync {
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}
impl_downcast!(ConnectionData);

impl ConnectionData for () {
    fn show(&mut self, _ui: &mut Ui, _style: &PortStyle) {}
}

type BoxedConnectionData = SmallBox<dyn ConnectionData, space::S32>;

pub struct Connection {
    id: ConnectionId,
    input_id: InputId,
    output_id: OutputId,
    data: BoxedConnectionData,
}

impl Connection {
    pub fn new<C: ConnectionData>(input_id: InputId, output_id: OutputId, data: C) -> Self {
        assert!(input_id.port_type_id() == output_id.port_type_id());

        Self {
            id: ConnectionId::default(),
            input_id,
            output_id,
            data: smallbox!(data),
        }
    }

    pub(in crate::node_graph) fn with_id(self, connection_id: ConnectionId) -> Self {
        Self {
            id: connection_id,
            ..self
        }
    }

    pub fn id(&self) -> ConnectionId {
        self.id
    }

    pub fn input_id(&self) -> InputId {
        self.input_id
    }

    pub fn input_node_id(&self) -> NodeId {
        self.input_id.node_id()
    }

    pub fn output_id(&self) -> OutputId {
        self.output_id
    }

    pub fn output_node_id(&self) -> NodeId {
        self.output_id.node_id()
    }

    pub fn data(&self) -> &dyn ConnectionData {
        &*self.data
    }

    pub fn data_as<T: ConnectionData>(&self) -> &T {
        let data = &*self.data;
        data.downcast_ref::<T>().unwrap()
    }

    pub fn data_mut_as<T: ConnectionData>(&mut self) -> &mut T {
        let data = &mut *self.data;
        data.downcast_mut::<T>().unwrap()
    }

    pub(in crate::node_graph) fn show(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        port_styles: &PortStyles,
    ) {
        let start = port_visual.get_screen_pos(self.input_id.into());
        let end = port_visual.get_screen_pos(self.output_id.into());

        if let (Some(start), Some(end)) = (start, end) {
            let type_id = self.input_id.port_type_id();
            if let Some(style) = port_styles.find(type_id) {
                draw_connection(
                    ui.painter(),
                    start,
                    end,
                    Stroke {
                        color: style.color,
                        width: style.connection_width * zoom_pan.zoom,
                    },
                );

                /*if self.data.is_visible() */
                {
                    let rect = Rect::from_points(&[start, end]);
                    /*ui.painter().rect(
                        rect,
                        0.,
                        egui::Color32::TRANSPARENT,
                        egui::Stroke::new(1., egui::Color32::YELLOW),
                    );*/
                    let screen_center = ui.ctx().available_rect().center();
                    let rect_center = rect.center();
                    let offset = rect_center - screen_center;
                    let id = zoom_pan.child_id(self.id);
                    Area::new(id)
                        .order(Order::Middle)
                        .anchor(Align2::CENTER_CENTER, offset)
                        .enabled(port_visual.is_nodes_enabled())
                        .movable(false)
                        .show(ui.ctx(), |ui| {
                            ui.set_max_size(rect.size());
                            ui.set_clip_rect(zoom_pan.screen_rect);
                            self.data.show(ui, style);
                        });
                }
            } else {
                log::warn!("Skipping connection {:?}, style for {:?} not found", self.id(), type_id);
            }
        }
    }
}
