use crate::node_graph::{GraphOperation, ZoomPanState};
use egui::{pos2, Id, Pos2, Ui};
use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct ContextMenuId; }

pub struct ContextMenuItem {
    pub name: String,
}

impl From<&str> for ContextMenuItem {
    fn from(name: &str) -> Self {
        ContextMenuItem { name: name.into() }
    }
}

enum ContextMenuKind {
    SubMenu { name: String, items: Vec<ContextMenuKind> },
    LeafItem(ContextMenuId),
}

pub struct ContextMenu {
    items: SlotMap<ContextMenuId, ContextMenuItem>,
    root: ContextMenuKind,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            items: SlotMap::default(),
            root: ContextMenuKind::SubMenu {
                name: "root".into(),
                items: Vec::new(),
            },
        }
    }
}

impl ContextMenu {
    pub fn builder(&mut self) -> ConextSubMenuBuilder<'_> {
        ConextSubMenuBuilder {
            menu_items: &mut self.items,
            corrent: &mut self.root,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        if let ContextMenuKind::SubMenu { items, .. } = &mut self.root {
            items.clear();
        }
    }
}

pub struct ConextSubMenuBuilder<'m> {
    menu_items: &'m mut SlotMap<ContextMenuId, ContextMenuItem>,
    corrent: &'m mut ContextMenuKind,
}

impl<'m> ConextSubMenuBuilder<'m> {
    pub fn add_item_with<I: Into<ContextMenuItem>, F: FnOnce(ContextMenuId)>(&mut self, item: I, with: F) -> &mut Self {
        let id = self.menu_items.insert(item.into());
        if let ContextMenuKind::SubMenu { items, .. } = &mut self.corrent {
            items.push(ContextMenuKind::LeafItem(id));
            (with)(id);
            self
        } else {
            unreachable!()
        }
    }

    pub fn add_item<I: Into<ContextMenuItem>>(&mut self, item: I) -> &mut Self {
        self.add_item_with(item, |_| {})
    }

    pub fn add_group<'n, S: ToString>(&'n mut self, name: S) -> ConextSubMenuBuilder<'n>
    where
        'm: 'n,
    {
        if let ContextMenuKind::SubMenu { items, .. } = &mut self.corrent {
            items.push(ContextMenuKind::SubMenu {
                name: name.to_string(),
                items: Vec::new(),
            });
            let corrent = items.last_mut().unwrap();
            ConextSubMenuBuilder {
                menu_items: self.menu_items,
                corrent,
            }
        } else {
            unreachable!()
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

    fn show_recursive(
        &self,
        menu_items: &SlotMap<ContextMenuId, ContextMenuItem>,
        current: &ContextMenuKind,
        ui: &mut Ui,
        operations: &mut Vec<GraphOperation>,
    ) {
        match current {
            ContextMenuKind::SubMenu { name, items } => {
                ui.menu_button(name, |ui| {
                    for sub_item in items {
                        self.show_recursive(menu_items, sub_item, ui, operations);
                    }
                });
            }
            ContextMenuKind::LeafItem(menu_id) => {
                let item = menu_items.get(*menu_id).unwrap();
                if ui.button(&item.name).clicked() {
                    operations.push(GraphOperation::ContextMenu(self.start_location, *menu_id));
                    ui.close_menu();
                }
            }
        }
    }

    fn show_filtered(
        &self,
        menu_items: &SlotMap<ContextMenuId, ContextMenuItem>,
        filter: &[&str],
        ui: &mut Ui,
        operations: &mut Vec<GraphOperation>,
    ) {
        for (id, item) in menu_items {
            if filter.iter().any(|filter| item.name.starts_with(filter)) && ui.button(&item.name).clicked() {
                operations.push(GraphOperation::ContextMenu(self.start_location, id));
                ui.close_menu();
            }
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        content: &ContextMenu,
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

        //todo: store in state, thus no "heavy" calculation in each frame
        let filters = self
            .filter
            .split(' ')
            .map(|f| f.trim())
            .filter(|f| !f.is_empty())
            .collect::<Vec<&str>>();

        if filters.is_empty() {
            if let ContextMenuKind::SubMenu { items, .. } = &content.root {
                for sub_item in items {
                    self.show_recursive(&content.items, sub_item, ui, operations);
                }
            } else {
                unreachable!()
            }
        } else {
            self.show_filtered(&content.items, &filters, ui, operations);
        }
    }
}
