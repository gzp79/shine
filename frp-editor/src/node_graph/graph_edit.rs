use crate::node_graph::{
    ConnectionEditState, ContextMenuItem, ContextMenuState, Graph, GraphOperation, PortViewState, ZoomPanState,
};
use egui::{Id, Key, Response, Sense, Ui};
use shine_core::atomic_refcell::AtomicRefCell;
use std::{hash::Hash, sync::Arc};

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
    last_action: String,
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
pub struct GraphEdit {
    id: Id,
    graph: Arc<AtomicRefCell<Graph>>,
    context_menu: Arc<Vec<ContextMenuItem>>,
}

impl GraphEdit {
    pub fn new<I: Hash>(id: I, graph: Arc<AtomicRefCell<Graph>>, context_menu: Arc<Vec<ContextMenuItem>>) -> Self {
        Self {
            id: Id::new(id),
            graph,
            context_menu,
        }
    }
}

impl GraphEdit {
    fn show_graph(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        editor_state: &mut GraphEditState,
        port_visual: &mut PortViewState,
        operations: &mut Vec<GraphOperation>,
    ) -> Option<Response> {
        let graph = &*self.graph.borrow();

        // render nodes
        let mut response: Option<Response> = None;
        for node in graph.nodes() {
            let node_response = node.show(ui, zoom_pan, port_visual, graph, operations);
            response = match response {
                None => Some(node_response),
                Some(r) => Some(r.union(node_response)),
            };
        }

        if matches!(editor_state.mode, EditorMode::None | EditorMode::NodeInteract) {
            if response.as_ref().map(|r| r.dragged()).unwrap_or(false) {
                // port hover has a higher precedence. Start connection edit instead of node drag (unless we are already in drag mode)
                if !port_visual.has_hovered() {
                    editor_state.mode = EditorMode::NodeInteract;
                }
            } else {
                editor_state.mode = EditorMode::None;
            }
        }

        //render connections
        for connection in graph.connections() {
            connection.show(ui, zoom_pan, port_visual, graph)
        }

        response
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut editor_state = GraphEditState::load(ui, self.id).unwrap_or_default();
        let mut zoom_pan = ZoomPanState::load(ui, self.id).unwrap_or_else(|| ZoomPanState::new(self.id, ui));
        let mut port_visual = PortViewState::load(ui, self.id).unwrap_or_default();
        let mut connection_edit = ConnectionEditState::load(ui, self.id).unwrap_or_default();
        let mut context_menu = ContextMenuState::load(ui, self.id).unwrap_or_default();

        let mut operations = Vec::new();

        zoom_pan.prepare(ui.style());
        zoom_pan.screen_rect = ui.available_rect_before_wrap();
        port_visual.clear();
        port_visual.set_nodes_enabled(matches!(editor_state.mode, EditorMode::None | EditorMode::NodeInteract));
        port_visual.set_ports_enabled(matches!(
            editor_state.mode,
            EditorMode::None | EditorMode::EditConnection
        ));
        connection_edit.prepare(&mut port_visual);

        let nodes_response = zoom_pan.show_zoomed(ui, |ui| {
            self.show_graph(ui, &zoom_pan, &mut editor_state, &mut port_visual, &mut operations)
        });

        let mut response = ui.interact(zoom_pan.screen_rect, self.id.with("graph"), Sense::drag());

        // connection edit
        if matches!(editor_state.mode, EditorMode::None | EditorMode::EditConnection) {
            let (mode, operation) = connection_edit.update(
                ui,
                &zoom_pan,
                &port_visual,
                nodes_response.as_ref(),
                &self.graph.borrow(),
                |_, _| true,
            );
            if let Some(mode) = mode {
                editor_state.mode = mode;
            }
            if let Some(operation) = operation {
                operations.push(operation);
            }
        }

        // context menu
        if matches!(editor_state.mode, EditorMode::None | EditorMode::ContextMenu) {
            editor_state.mode = EditorMode::None;
            response = response.context_menu(|ui| {
                editor_state.mode = EditorMode::ContextMenu;
                context_menu.show(ui, &zoom_pan, &self.context_menu[..], &mut operations)
            });
        }

        //handle pan
        if matches!(editor_state.mode, EditorMode::None) {
            zoom_pan.drag(response.drag_delta());
        }
        //handle zoom
        if matches!(editor_state.mode, EditorMode::None | EditorMode::EditConnection) {
            if let Some(pos) = ui.input().pointer.hover_pos() {
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

        // apply actions
        if !operations.is_empty() {
            editor_state.last_action = String::new();
            let graph = &mut *self.graph.borrow_mut();
            for action in operations.drain(..) {
                editor_state.last_action = format!("{} {:?}", editor_state.last_action, action);
                graph.apply_action(action);
            }
        }

        // some debug stuff
        egui::Window::new("debug")
            .id(self.id.with("debug"))
            .show(ui.ctx(), |ui| {
                ui.label(format!("mode: {:?}", editor_state.mode));
                ui.label(format!("action: {:?}", editor_state.last_action));
                ui.label(format!("clip: {:?}", zoom_pan.screen_rect));
                ui.label(format!("start_location: {:?}", context_menu.start_location));
                let ap = zoom_pan.pos2_screen_to_area(ui.input().pointer.hover_pos().unwrap_or_default());
                ui.label(format!(
                    "pos : {:?}",
                    ui.input().pointer.hover_pos().unwrap_or_default()
                ));
                ui.label(format!("area pos: {:?}", ap));
            });

        editor_state.store(ui, self.id);
        zoom_pan.store(ui, self.id);
        port_visual.store(ui, self.id);
        connection_edit.store(ui, self.id);
        context_menu.store(ui, self.id);
    }
}
