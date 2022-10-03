use crate::node_graph::{
    utils::draw_connection, EditorMode, Graph, GraphOperation, InputId, InputOutputId, OutputId, PortSelection,
    ZoomPanState,
};
use egui::{Id, Pos2, Response, Stroke, Ui};

use super::PortViewState;

/// Edit connection between ports
#[derive(Default, Clone, Debug)]
pub(in crate::node_graph) struct ConnectionEditState {
    start: Option<InputOutputId>,
    start_pos: Option<Pos2>,
    end: Option<InputOutputId>,
    end_pos: Option<Pos2>,
    valid: bool,
}

impl ConnectionEditState {
    pub fn load(ui: &mut Ui, id: Id) -> Option<ConnectionEditState> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    pub fn prepare(&self, port_visual: &mut PortViewState) {
        let selection = if self.valid {
            PortSelection::Hover
        } else {
            PortSelection::Error
        };

        if let Some(start) = self.start {
            port_visual.set_selection(start, selection);
        }

        if let Some(end) = self.end {
            port_visual.set_selection(end, selection);
        }
    }

    fn draw(&self, ui: &mut Ui, zoom_pan: &ZoomPanState, graph: &Graph) {
        zoom_pan.show_clipped(ui, |ui| {
            if let (Some(start), Some(start_pos), Some(end_pos)) = (&self.start, self.start_pos, self.end_pos) {
                let (start_pos, end_pos) = if start.is_input() {
                    (start_pos, end_pos)
                } else {
                    (end_pos, start_pos)
                };

                let style = graph.get_type(start.type_id()).unwrap();
                let color = if self.valid {
                    style.hover_color
                } else {
                    style.error_color
                };
                draw_connection(
                    ui.painter(),
                    start_pos,
                    end_pos,
                    Stroke {
                        color,
                        width: style.connection_width * zoom_pan.zoom,
                    },
                );
            }
        })
    }

    pub fn update<F>(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        dragged_node: Option<&Response>,
        graph: &Graph,
        validate: F,
    ) -> (Option<EditorMode>, Option<GraphOperation>)
    where
        F: FnOnce(InputId, OutputId) -> bool,
    {
        let pointer_pos = ui.input().pointer.hover_pos().unwrap_or(Pos2::ZERO);
        let pointer_down = ui.input().pointer.any_down();
        let drag_start = dragged_node.map(|r| r.drag_started()).unwrap_or(false);

        if self.start.is_none() && port_visual.has_hovered() && drag_start {
            // start a new connection
            let port = port_visual.get_hovered().unwrap();
            let pos = port_visual.get_hovered_pos().unwrap();
            self.start = Some(port);
            self.start_pos = Some(pos);
            self.end = None;
            self.end_pos = None;
            self.valid = false;

            (Some(EditorMode::EditConnection), None)
        } else if self.start.is_some() {
            // update a pending connection
            self.start_pos = port_visual.get_screen_pos(self.start.unwrap());

            if let Some(port) = port_visual.get_hovered() {
                let pos = port_visual.get_screen_pos(port).unwrap();
                let update_validity = self.end != Some(port); // port was changed

                self.end = Some(port);
                self.end_pos = Some(pos);

                if update_validity {
                    self.valid = match (self.start.unwrap(), self.end.unwrap()) {
                        (InputOutputId::Input(input_id), InputOutputId::Output(output_id)) => {
                            input_id.type_id() == output_id.type_id() && (validate)(input_id, output_id)
                        }
                        (InputOutputId::Output(output_id), InputOutputId::Input(input_id)) => {
                            input_id.type_id() == output_id.type_id() && (validate)(input_id, output_id)
                        }
                        _ => false,
                    };
                }
            } else {
                self.end = None;
                self.end_pos = Some(pointer_pos);
                self.valid = false;
            }

            self.draw(ui, zoom_pan, graph);

            if !pointer_down {
                let operation = if self.valid {
                    match (self.start.unwrap(), self.end.unwrap()) {
                        (InputOutputId::Input(input_id), InputOutputId::Output(output_id)) => {
                            Some(GraphOperation::Connect(input_id, output_id))
                        }
                        (InputOutputId::Output(output_id), InputOutputId::Input(input_id)) => {
                            Some(GraphOperation::Connect(input_id, output_id))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    None
                };

                self.cancel();
                (Some(EditorMode::None), operation)
            } else {
                (Some(EditorMode::EditConnection), None)
            }
        } else {
            (None, None)
        }
    }

    pub fn cancel(&mut self) {
        *self = Self::default();
    }
}
