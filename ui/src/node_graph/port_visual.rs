use crate::node_graph::InputOutputId;
use egui::{Id, Pos2, Ui};
use shine_core::atomic_refcell::AtomicRefCell;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub(in crate::node_graph) enum PortSelection {
    Normal,
    Error,
    Hover,
}

#[derive(Default)]
struct Inner {
    selected_ports: HashMap<InputOutputId, PortSelection>,
    screen_location: HashMap<InputOutputId, Pos2>,
}

/// Visual information of the ports of a graph asociated to a view of it.
#[derive(Clone)]
pub(in crate::node_graph) struct PortViewState {
    hovered: Option<InputOutputId>,
    /// Arc-ed larger data structurd to make the `Clone` cheaper.
    inner: Arc<AtomicRefCell<Inner>>,

    nodes_enabled: bool,
    ports_enabled: bool,
}

impl Default for PortViewState {
    fn default() -> Self {
        Self {
            hovered: None,
            inner: Arc::new(AtomicRefCell::new(Inner::default())),
            nodes_enabled: true,
            ports_enabled: true,
        }
    }
}

impl PortViewState {
    pub fn load(ui: &mut Ui, id: Id) -> Option<PortViewState> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    pub fn clear(&mut self) {
        let inner = &mut *self.inner.borrow_mut();
        self.hovered = None;
        inner.selected_ports.clear();
        inner.screen_location.clear();
    }

    pub fn is_nodes_enabled(&mut self) -> bool {
        self.nodes_enabled
    }

    pub fn set_nodes_enabled(&mut self, enabled: bool) {
        self.nodes_enabled = enabled;
    }

    pub fn is_ports_enabled(&mut self) -> bool {
        self.ports_enabled
    }

    pub fn set_ports_enabled(&mut self, enabled: bool) {
        self.ports_enabled = enabled;
    }

    pub fn has_hovered(&self) -> bool {
        self.hovered.is_some()
    }

    pub fn get_hovered(&self) -> Option<InputOutputId> {
        self.hovered
    }

    pub fn get_hovered_pos(&self) -> Option<Pos2> {
        self.hovered.and_then(|port_id| self.get_screen_pos(port_id))
    }

    pub fn set_hovered(&mut self, port_id: InputOutputId) {
        self.hovered = Some(port_id)
    }

    pub fn get_screen_pos(&self, port_id: InputOutputId) -> Option<Pos2> {
        let inner = self.inner.borrow();
        inner.screen_location.get(&port_id).cloned()
    }

    pub fn set_screen_pos(&mut self, port_id: InputOutputId, pos: Pos2) {
        let inner = &mut *self.inner.borrow_mut();
        let _ = inner.screen_location.insert(port_id, pos);
    }

    pub fn get_selection(&self, port_id: InputOutputId) -> PortSelection {
        let inner = self.inner.borrow();
        inner
            .selected_ports
            .get(&port_id)
            .cloned()
            .unwrap_or(PortSelection::Normal)
    }

    pub fn set_selection(&mut self, port_id: InputOutputId, selection: PortSelection) {
        let inner = &mut *self.inner.borrow_mut();
        let _ = inner.selected_ports.insert(port_id, selection);
    }
}
