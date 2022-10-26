use crate::node_graph::{
    utils::draw_connection, Graph, GraphData, InputId, InputOutputId, OutputId, PortSelection, PortViewState,
    ZoomPanState,
};
use egui::{Id, Pos2, Stroke, Ui};

use super::ConnectionData;

pub(in crate::node_graph) enum ConnectionResult<C>
where
    C: ConnectionData,
{
    Pending,
    Completed(Option<(InputId, OutputId, C)>),
}

/// Edit connection between ports
#[derive(Clone, Debug)]
pub(in crate::node_graph) struct ConnectionEditState<C>
where
    C: ConnectionData,
{
    start: Option<InputOutputId>,
    start_pos: Option<Pos2>,
    end: Option<InputOutputId>,
    end_pos: Option<Pos2>,
    connection_data: Option<C>,
}

impl<C> Default for ConnectionEditState<C>
where
    C: ConnectionData,
{
    fn default() -> Self {
        Self {
            start: None,
            start_pos: None,
            end: None,
            end_pos: None,
            connection_data: None,
        }
    }
}

impl<C> ConnectionEditState<C>
where
    C: ConnectionData,
{
    pub fn load(ui: &mut Ui, id: Id) -> Option<Self> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    pub fn prepare(&self, port_visual: &mut PortViewState) {
        let selection = if self.connection_data.is_some() {
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

    fn draw<G: GraphData>(&self, ui: &mut Ui, zoom_pan: &ZoomPanState, graph: &Graph<G>) {
        zoom_pan.show_clipped(ui, |ui| {
            if let (Some(start), Some(start_pos), Some(end_pos)) = (&self.start, self.start_pos, self.end_pos) {
                let (start_pos, end_pos) = if start.is_input() {
                    (start_pos, end_pos)
                } else {
                    (end_pos, start_pos)
                };

                let type_id = start.port_type_id();
                let style = graph
                    .type_styles
                    .get(&type_id)
                    .expect("Connection shall be drown only with known types");
                let color = if self.connection_data.is_some() {
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

    pub fn update<G>(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &PortViewState,
        graph: &mut Graph<G>,
    ) -> ConnectionResult<G::ConnectionData>
    where
        G: GraphData<ConnectionData = C>,
    {
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
            self.connection_data = None;

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
                    self.connection_data = match (self.start.unwrap(), self.end.unwrap()) {
                        (InputOutputId::Input(input_id), InputOutputId::Output(output_id)) => {
                            <G::ConnectionData as ConnectionData>::try_connect(graph, input_id, output_id)
                        }
                        (InputOutputId::Output(output_id), InputOutputId::Input(input_id)) => {
                            <G::ConnectionData as ConnectionData>::try_connect(graph, input_id, output_id)
                        }
                        _ => None,
                    };
                }
            } else {
                self.end = None;
                self.end_pos = Some(pointer_pos);
                self.connection_data = None;
            }

            self.draw(ui, zoom_pan, graph);

            if !pointer_down {
                let operation = if let Some(connection_data) = self.connection_data.take() {
                    match (self.start.unwrap(), self.end.unwrap()) {
                        (InputOutputId::Input(input_id), InputOutputId::Output(output_id)) => {
                            Some((input_id, output_id, connection_data))
                        }
                        (InputOutputId::Output(output_id), InputOutputId::Input(input_id)) => {
                            Some((input_id, output_id, connection_data))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    None
                };

                self.cancel();
                ConnectionResult::Completed(operation)
            } else {
                ConnectionResult::Pending
            }
        }
    }

    pub fn cancel(&mut self) {
        *self = Self::default();
    }
}
