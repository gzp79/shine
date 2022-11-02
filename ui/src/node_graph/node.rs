use crate::{
    node_graph::{
        Input, InputId, InputOutputId, Output, OutputId, PortSelection, PortStyle, PortStyles, PortViewState,
        ZoomPanState,
    },
    utils::{FrameWithHeader, Scale},
};
use eframe::epaint::Shadow;
use egui::{pos2, vec2, Area, Frame, Id, Order, Painter, Pos2, Rect, Stroke, Ui, Vec2};
use shine_core::{
    downcast_rs::{impl_downcast, Downcast},
    slotmap::new_key_type,
    smallbox::{smallbox, space, SmallBox},
};

new_key_type! { pub struct NodeId; }

pub trait NodeData: 'static + Downcast + Send + Sync {
    fn show(&mut self, _ui: &mut Ui) {}
    fn on_moved(&mut self, _new_location: Pos2) {}
}
impl_downcast!(NodeData);

impl NodeData for () {}

type BoxedNodeData = SmallBox<dyn NodeData, space::S32>;

pub struct Node {
    id: NodeId,
    pub caption: String,
    pub location: Pos2,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    data: BoxedNodeData,
}

impl Node {
    pub fn new<S: ToString>(caption: S, location: Pos2, inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Self {
            id: NodeId::default(),
            caption: caption.to_string(),
            inputs,
            outputs,
            location,
            data: smallbox!(()),
        }
    }

    pub fn with_id(self, node_id: NodeId) -> Self {
        Self { id: node_id, ..self }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn with_data<N: NodeData>(self, data: N) -> Self {
        Self {
            data: smallbox!(data),
            ..self
        }
    }

    pub fn data(&self) -> &dyn NodeData {
        &*self.data
    }

    pub fn data_as<T: NodeData>(&self) -> &T {
        let data = &*self.data;
        data.downcast_ref::<T>().unwrap()
    }

    pub fn data_mut_as<T: NodeData>(&mut self) -> &mut T {
        let data = &mut *self.data;
        data.downcast_mut::<T>().unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_port(
        &self,
        painter: &Painter,
        zoom_pan: &ZoomPanState,
        port_visual: &mut PortViewState,
        style: &PortStyle,
        port_id: InputOutputId,
        port_pos: Pos2,
        pointer_pos: Option<Pos2>,
        node_rect: &mut Rect,
    ) {
        port_visual.set_screen_pos(port_id, port_pos);

        let r = style.port_size * zoom_pan.zoom;
        let dist = pointer_pos.map(|p| port_pos.distance_sq(p)).unwrap_or(f32::MAX);
        let is_hovered = dist < r * r * 1.3 && port_visual.is_ports_enabled() && !port_visual.has_hovered();

        if is_hovered {
            port_visual.set_hovered(port_id);
        }

        let color = match port_visual.get_selection(port_id) {
            PortSelection::Normal => {
                if is_hovered {
                    style.hover_color
                } else {
                    style.color
                }
            }
            PortSelection::Error => style.error_color,
            PortSelection::Hover => style.hover_color,
        };

        painter.circle(port_pos, r, color, Stroke::none());
        *node_rect = node_rect.union(Rect::from_center_size(port_pos, vec2(r * 2., r * 2.)));
    }

    pub(in crate::node_graph) fn show(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &mut PortViewState,
        port_styles: &PortStyles,
    ) -> NodeState {
        let id = zoom_pan.child_id(self.id);

        let mut node_state = NodeState::load(ui, id).unwrap_or_else(NodeState::new);
        let screen_location = zoom_pan.pos2_area_to_screen(self.location);

        let response = Area::new(id)
            .order(Order::Middle)
            .current_pos(screen_location)
            .enabled(port_visual.is_nodes_enabled())
            .movable(port_visual.is_nodes_enabled())
            .drag_bounds(Rect::EVERYTHING)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(zoom_pan.screen_rect);
                ui.set_max_size(Vec2::INFINITY);

                let mut node_rect = Rect::NOTHING;
                let margin = ui.style().spacing.window_margin.scaled(2.);

                FrameWithHeader::new(&self.caption)
                    .frame(Frame::window(ui.style()).shadow(Shadow::default()).inner_margin(margin))
                    .show(ui, |ui| {
                        self.data.show(ui);

                        let mut port_infos = Vec::<(InputOutputId, f32)>::new();
                        let port_top = ui.min_rect().bottom();
                        ui.horizontal(|ui| {
                            //inputs
                            ui.vertical(|ui| {
                                let mut height_before = port_top;
                                for (idx, input) in self.inputs.iter_mut().enumerate() {
                                    let type_id = input.port_type_id();
                                    if let Some(style) = port_styles.find(type_id) {
                                        input.show(ui, style);
                                        let height_after = ui.min_rect().bottom();
                                        let y = (height_after + height_before) / 2.;
                                        height_before = height_after;
                                        let id = InputId::new(self.id, type_id, idx);
                                        port_infos.push((id.into(), y));
                                    } else {
                                        log::warn!("Skipping input port, style for {:?} not found", type_id);
                                    }
                                }
                            });
                            // outputs
                            ui.vertical(|ui| {
                                let mut height_before = port_top;
                                for (idx, output) in self.outputs.iter_mut().enumerate() {
                                    let type_id = output.port_type_id();
                                    if let Some(style) = port_styles.find(type_id) {
                                        output.show(ui, style);
                                        let height_after = ui.min_rect().bottom();
                                        let y = (height_after + height_before) / 2.;
                                        height_before = height_after;
                                        let id = OutputId::new(self.id, type_id, idx);
                                        port_infos.push((id.into(), y));
                                    } else {
                                        log::warn!("Skipping output port, style for {:?} not found", type_id);
                                    }
                                }
                            });
                        });

                        let port_rect = Rect::from_min_max(
                            ui.min_rect().min - margin.left_top(),
                            ui.min_rect().max + margin.right_bottom(),
                        );
                        node_rect = node_rect.union(port_rect);

                        // render port after the frame on a background layer
                        let painter = ui.painter();
                        let pointer_pos = ui.ctx().pointer_latest_pos();
                        for (port_id, y) in port_infos {
                            let port_pos = match &port_id {
                                InputOutputId::Input(_) => pos2(port_rect.left(), y),
                                InputOutputId::Output(_) => pos2(port_rect.right(), y),
                            };
                            let style = port_styles
                                .find(port_id.port_type_id())
                                .expect("Port shall be drown only with known types");
                            self.draw_port(
                                painter,
                                zoom_pan,
                                port_visual,
                                style,
                                port_id,
                                port_pos,
                                pointer_pos,
                                &mut node_rect,
                            );
                        }
                    });

                // increment the node to include the ports
                /*ui.painter().rect(
                    node_rect,
                    0.,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(1., egui::Color32::YELLOW),
                );*/
                ui.expand_to_include_rect(node_rect);
            })
            .response;

        node_state.drag_started = false;
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            if response.drag_started() && zoom_pan.screen_rect.contains(pos) {
                node_state.drag_started = true;
                node_state.dragged = true;
            }
        }
        if !response.dragged() {
            node_state.dragged = false;
        }

        if node_state.dragged && response.drag_delta() != Vec2::ZERO {
            let new_location = self.location + zoom_pan.vec2_screen_to_area(response.drag_delta());
            self.data.on_moved(new_location);
            self.location = new_location;
        }

        node_state.clone().store(ui, id);
        node_state
    }
}

#[derive(Clone)]
pub(in crate::node_graph) struct NodeState {
    /// node drag was started in this frame
    pub drag_started: bool,
    /// this node is dragged
    pub dragged: bool,
}

impl NodeState {
    fn load(ui: &mut Ui, id: Id) -> Option<NodeState> {
        ui.data().get_temp(id)
    }

    fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    fn new() -> Self {
        Self {
            drag_started: false,
            dragged: false,
        }
    }
}
