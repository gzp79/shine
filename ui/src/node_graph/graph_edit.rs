use crate::node_graph::{
    ConnectionEditState, ConnectionResult, ContextMenu, ContextMenuState, Graph, NodeCommand, PortViewState,
    ZoomPanState,
};
use egui::{Id, Key, Sense, Ui};

/// Current editor mode
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::node_graph) enum EditorMode {
    #[default]
    None,
    NodeInteract,
    EditConnection,
    ContextMenu,
}

#[derive(Default, Clone)]
struct GraphEditState {
    mode: EditorMode,
}

impl GraphEditState {
    fn load(ui: &mut Ui, id: Id) -> Option<GraphEditState> {
        ui.data().get_temp(id)
    }

    fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }
}

/// The graph editor widget
pub struct GraphEdit<'a> {
    id: Id,
    graph: &'a mut Graph,
    context_menu: &'a ContextMenu,
}

impl<'a> GraphEdit<'a> {
    pub fn new<I: Into<Id>>(id: I, graph: &'a mut Graph, context_menu: &'a ContextMenu) -> Self {
        Self {
            id: id.into(),
            graph,
            context_menu,
        }
    }

    fn show_graph(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        editor_state: &mut GraphEditState,
        port_visual: &mut PortViewState,
        command_queue: &mut Vec<NodeCommand>,
    ) {
        // render nodes
        let mut dragged_node = None;

        let style = self.graph.get_port_styles().clone();
        for node in self.graph.nodes_mut() {
            let node_state = node.show(ui, zoom_pan, port_visual, &style, command_queue);
            if node_state.dragged {
                dragged_node = Some(node_state);
            }
        }

        if matches!(editor_state.mode, EditorMode::None | EditorMode::NodeInteract) {
            if let Some(dragged_node) = dragged_node {
                // port hover has a higher precedence. Start connection edit instead of node drag (unless we are already in drag mode)
                if dragged_node.drag_started && port_visual.has_hovered() {
                    editor_state.mode = EditorMode::EditConnection;
                } else {
                    editor_state.mode = EditorMode::NodeInteract;
                }
            } else {
                editor_state.mode = EditorMode::None;
            }
        }

        //render connections
        for connection in self.graph.connections_mut() {
            connection.show(ui, zoom_pan, port_visual, &style)
        }
    }

    pub fn show(&mut self, ui: &mut Ui, command_queue: &mut Vec<NodeCommand>) {
        let mut editor_state = GraphEditState::load(ui, self.id).unwrap_or_default();
        let mut zoom_pan = ZoomPanState::load(ui, self.id).unwrap_or_else(|| ZoomPanState::new(self.id, ui));
        let mut port_visual = PortViewState::load(ui, self.id).unwrap_or_default();
        let mut connection_edit = ConnectionEditState::load(ui, self.id).unwrap_or_default();
        let mut context_menu = ContextMenuState::load(ui, self.id).unwrap_or_default();

        zoom_pan.prepare(ui.style());
        zoom_pan.screen_rect = ui.available_rect_before_wrap();
        port_visual.clear();
        port_visual.set_nodes_enabled(matches!(editor_state.mode, EditorMode::None | EditorMode::NodeInteract));
        port_visual.set_ports_enabled(matches!(
            editor_state.mode,
            EditorMode::None | EditorMode::EditConnection
        ));
        connection_edit.prepare(&mut port_visual);

        let mut response = ui.interact(zoom_pan.screen_rect, self.id.with("graph"), Sense::drag());

        zoom_pan.show_zoomed(ui, |ui| {
            self.show_graph(ui, &zoom_pan, &mut editor_state, &mut port_visual, command_queue);
        });

        // connection edit
        if matches!(editor_state.mode, EditorMode::EditConnection) {
            editor_state.mode = match connection_edit.update(ui, &zoom_pan, &port_visual, self.graph) {
                ConnectionResult::Pending => EditorMode::EditConnection,
                ConnectionResult::Completed(connection) => {
                    if let Some(connection) = connection {
                        self.graph.add_connection(connection);
                    }
                    EditorMode::None
                }
            };
        }

        // context menu
        if matches!(editor_state.mode, EditorMode::None | EditorMode::ContextMenu) {
            editor_state.mode = EditorMode::None;
            response = response.context_menu(|ui| {
                editor_state.mode = EditorMode::ContextMenu;
                context_menu.show(ui, &zoom_pan, self.context_menu, self.graph)
            });
        }

        //handle pan
        if matches!(editor_state.mode, EditorMode::None) {
            zoom_pan.drag(response.drag_delta());
        }
        //handle zoom
        if matches!(editor_state.mode, EditorMode::None | EditorMode::EditConnection) {
            if let Some(pos) = ui.ctx().pointer_latest_pos() {
                let zoom = ui.input().scroll_delta.y;
                if zoom != 0. && zoom_pan.screen_rect.contains(pos) {
                    let zoom = (zoom * 0.002).exp();
                    zoom_pan.zoom_to_screen(pos, zoom);
                }
            }
        }

        if ui.input().key_pressed(Key::Escape) {
            // reset editor sate
            ui.close_menu();
            connection_edit.cancel();
            editor_state.mode = EditorMode::None;
        }

        // some debug stuff
        /*
        egui::Window::new("debug")
            .id(self.id.with("debug"))
            .drag_bounds(zoom_pan.screen_rect)
            .show(ui.ctx(), |ui| {
                ui.label(format!("mode: {:?}", editor_state.mode));
                ui.label(format!("clip: {:?}", zoom_pan.screen_rect));
            });*/

        editor_state.store(ui, self.id);
        zoom_pan.store(ui, self.id);
        port_visual.store(ui, self.id);
        connection_edit.store(ui, self.id);
        context_menu.store(ui, self.id);
    }
}
