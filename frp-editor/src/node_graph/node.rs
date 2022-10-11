use crate::{
    node_graph::{
        Graph, GraphOperation, Input, InputId, InputOutputId, Output, OutputId, PortSelection, PortViewState,
        ZoomPanState,
    },
    utils::{FrameWithHeader, Scale},
};
use eframe::epaint::Shadow;
use egui::{pos2, vec2, Area, Frame, Id, Order, Painter, Pos2, Rect, Stroke, Ui, Vec2};
use slotmap::new_key_type;

new_key_type! { pub struct NodeId; }

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

pub struct Node {
    node_id: NodeId,
    caption: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    location: Pos2,
}

impl Node {
    pub fn new<S: ToString>(
        node_id: NodeId,
        caption: S,
        location: Pos2,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Self {
        Self {
            node_id,
            caption: caption.to_string(),
            inputs,
            outputs,
            location,
        }
    }

    pub fn caption(&self) -> &str {
        &self.caption
    }

    pub fn set_location(&mut self, location: Pos2) {
        self.location = location;
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_port(
        &self,
        painter: &Painter,
        zoom_pan: &ZoomPanState,
        port_visual: &mut PortViewState,
        graph: &Graph,
        port_id: InputOutputId,
        port_pos: Pos2,
        pointer_pos: Option<Pos2>,
        node_rect: &mut Rect,
    ) {
        port_visual.set_screen_pos(port_id, port_pos);

        let style = graph.get_type(port_id.type_id()).unwrap();
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
        &self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &mut PortViewState,
        graph: &Graph,
        operations: &mut Vec<GraphOperation>,
    ) -> NodeState {
        let id = zoom_pan.child_id(self.node_id);

        let mut node_state = NodeState::load(ui, id).unwrap_or_else(|| NodeState::new());
        let screen_location = zoom_pan.pos2_area_to_screen(self.location);

        let response = Area::new(id)
            .order(Order::Middle)
            .current_pos(screen_location)
            .enabled(port_visual.is_nodes_enabled())
            .movable(port_visual.is_nodes_enabled())
            .drag_bounds(Rect::EVERYTHING)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(zoom_pan.screen_rect);

                let mut node_rect = Rect::NOTHING;
                let margin = ui.style().spacing.window_margin.scaled(2.);

                FrameWithHeader::new(&self.caption)
                    .frame(Frame::window(ui.style()).shadow(Shadow::default()).inner_margin(margin))
                    .show(ui, |ui| {
                        let mut port_infos = Vec::<(InputOutputId, f32)>::new();
                        let port_top = ui.min_rect().bottom();
                        ui.horizontal(|ui| {
                            //inputs
                            ui.vertical(|ui| {
                                let mut height_before = port_top;
                                for (idx, input) in self.inputs.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        input.show(ui);
                                        let mut a = 2.;
                                        ui.add(egui::Slider::new(&mut a, 0.0..=2000.0).text("set me"));

                                        let height_after = ui.min_rect().bottom();
                                        let y = (height_after + height_before) / 2.;
                                        height_before = height_after;
                                        let id = InputId::new(self.node_id, input.type_id(), idx);
                                        port_infos.push((id.into(), y));
                                    });
                                }
                            });
                            // outputs
                            ui.vertical(|ui| {
                                let mut height_before = port_top;
                                for (idx, output) in self.outputs.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        output.show(ui);
                                        let mut a = 4.;
                                        ui.add(egui::Slider::new(&mut a, 0.0..=200.0).text("set \n\nme"));

                                        let height_after = ui.min_rect().bottom();
                                        let y = (height_after + height_before) / 2.;
                                        height_before = height_after;
                                        let id = OutputId::new(self.node_id, output.type_id(), idx);
                                        port_infos.push((id.into(), y));
                                    });
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
                            self.draw_port(
                                painter,
                                zoom_pan,
                                port_visual,
                                graph,
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
                    Stroke::new(1., egui::Color32::RED),
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
            let new_loc = self.location + zoom_pan.vec2_screen_to_area(response.drag_delta());
            operations.push(GraphOperation::SetNodeLocation(self.node_id, new_loc));
        }

        node_state.clone().store(ui, id);
        node_state
    }
}
