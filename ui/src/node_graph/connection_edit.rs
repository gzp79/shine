use crate::node_graph::{
    utils::draw_connection, Connection, Graph, InputOutputId, PortSelection, PortViewState, ZoomPanState,
};
use egui::{Id, Pos2, Stroke, Ui};
use shine_core::atomic_refcell::AtomicRefCell;
use std::sync::Arc;

#[allow(clippy::large_enum_variant)]
pub(in crate::node_graph) enum ConnectionResult {
    Pending,
    Completed(Option<Connection>),
}

/// Edit connection between ports
#[derive(Clone)]
pub(in crate::node_graph) struct ConnectionEditState {
    start: Option<InputOutputId>,
    start_pos: Option<Pos2>,
    end: Option<InputOutputId>,
    end_pos: Option<Pos2>,
    connection: Arc<AtomicRefCell<Option<Connection>>>,
}

impl Default for ConnectionEditState {
    fn default() -> Self {
        Self {
            start: None,
            start_pos: None,
            end: None,
            end_pos: None,
            connection: Arc::new(AtomicRefCell::new(None)),
        }
    }
}

impl ConnectionEditState {
    pub fn load(ui: &mut Ui, id: Id) -> Option<Self> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    pub fn prepare(&self, port_visual: &mut PortViewState) {
        let selection = if self.connection.borrow().is_some() {
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

                let type_id = start.port_type_id();
                let style = graph
                    .get_port_styles()
                    .find(type_id)
                    .expect("Connection shall be drown only with known types");
                let color = if self.connection.borrow().is_some() {
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

    pub fn update(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        graph: &mut Graph,
    ) -> ConnectionResult {
        let pointer_pos = ui.ctx().pointer_latest_pos().unwrap_or(Pos2::ZERO);
        let pointer_down = ui.input().pointer.any_down();

        if self.start.is_none() {
            // start a new connection
            let port = port_visual.get_hovered().unwrap();
            let pos = port_visual.get_hovered_pos().unwrap();
            self.start = Some(port);
            self.start_pos = Some(pos);
            self.end = None;
            self.end_pos = None;
            *self.connection.borrow_mut() = None;

            ConnectionResult::Pending
        } else {
            // update a pending connection
            self.start_pos = port_visual.get_screen_pos(self.start.unwrap());

            if let Some(port) = port_visual.get_hovered() {
                let pos = port_visual.get_screen_pos(port).unwrap();
                let update_validity = self.end != Some(port); // port was changed

                self.end = Some(port);
                self.end_pos = Some(pos);

                if update_validity {
                    let connection = match (self.start.unwrap(), self.end.unwrap()) {
                        (InputOutputId::Input(input_id), InputOutputId::Output(output_id)) => {
                            if graph.find_connections(input_id, output_id).is_none() {
                                graph.validator().try_create_connection(graph, input_id, output_id)
                            } else {
                                None
                            }
                        }
                        (InputOutputId::Output(output_id), InputOutputId::Input(input_id)) => {
                            if graph.find_connections(input_id, output_id).is_none() {
                                graph.validator().try_create_connection(graph, input_id, output_id)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    *self.connection.borrow_mut() = connection;
                }
            } else {
                self.end = None;
                self.end_pos = Some(pointer_pos);
                *self.connection.borrow_mut() = None;
            }

            self.draw(ui, zoom_pan, graph);

            if !pointer_down {
                let connection = self.connection.borrow_mut().take();
                self.cancel();
                ConnectionResult::Completed(connection)
            } else {
                ConnectionResult::Pending
            }
        }
    }

    pub fn cancel(&mut self) {
        *self = Self::default();
    }
}
