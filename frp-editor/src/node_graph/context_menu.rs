use crate::node_graph::{GraphOperation, NodeBuilder, ZoomPanState};
use egui::{pos2, Id, Pos2, Ui};
use std::sync::Arc;

pub enum ContextMenuResult {
    BuildNode(Arc<dyn NodeBuilder>),
}

pub enum ContextMenuItem {
    SubMenu {
        group: String,
        items: Vec<ContextMenuItem>,
    },
    AddNode {
        name: String,
        builder: Arc<dyn NodeBuilder>,
    },
}

impl ContextMenuItem {
    pub fn sub_menu<S: ToString>(group: S, items: Vec<ContextMenuItem>) -> Self {
        Self::SubMenu {
            group: group.to_string(),
            items,
        }
    }

    pub fn add_node<S, F>(name: S, node_builder: F) -> Self
    where
        F: NodeBuilder,
        S: ToString,
    {
        Self::AddNode {
            name: name.to_string(),
            builder: Arc::new(node_builder),
        }
    }
}

#[derive(Clone)]
pub(in crate::node_graph) struct ContextMenuState {
    filter: String,
    pub start_location: Pos2,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        ContextMenuState {
            filter: String::new(),
            start_location: pos2(0., 0.),
        }
    }
}

impl ContextMenuState {
    pub fn load(ui: &mut Ui, id: Id) -> Option<ContextMenuState> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    fn show_item(
        &self,
        item: &ContextMenuItem,
        ui: &mut Ui,
        filters: Option<&[&str]>,
        operations: &mut Vec<GraphOperation>,
    ) {
        match item {
            ContextMenuItem::SubMenu { group, items } => {
                if let Some(filters) = filters {
                    for item in items {
                        self.show_item(item, ui, Some(filters), operations);
                    }
                } else {
                    ui.menu_button(group, |ui| {
                        for item in items {
                            self.show_item(item, ui, None, operations);
                        }
                    });
                }
            }
            ContextMenuItem::AddNode { name, builder } => {
                let visible = filters
                    .map(|filter| filter.iter().any(|filter| name.starts_with(filter)))
                    .unwrap_or(true);
                if visible && ui.button(name).clicked() {
                    operations.push(GraphOperation::AddNode(self.start_location, builder.clone()));
                    ui.close_menu();
                }
            }
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        content: &[ContextMenuItem],
        operations: &mut Vec<GraphOperation>,
    ) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.filter).request_focus();
            if ui.button("X").clicked() {
                self.filter.clear()
            }
        });

        {
            let pointer = &ui.ctx().input().pointer;
            if pointer.secondary_down() {
                // context menu was just created
                let pos = pointer.press_origin().unwrap();
                self.start_location = zoom_pan.pos2_screen_to_area(pos);
                self.filter = String::new(); // remove this line to keep the filter
            }
        }

        let filters = self
            .filter
            .split(' ')
            .map(|f| f.trim())
            .filter(|f| !f.is_empty())
            .collect::<Vec<&str>>();
        let filters = if filters.is_empty() { None } else { Some(&filters[..]) };

        for item in content {
            self.show_item(item, ui, filters, operations);
        }
    }
}
