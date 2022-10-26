use std::marker::PhantomData;

use crate::node_graph::{Graph, GraphData, ZoomPanState};
use egui::{pos2, Id, Pos2, Ui};
use shine_core::slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct ContextMenuId; }

pub trait ContextMenuData: Clone + Send + Sync + 'static {
    type GraphData: GraphData;

    fn on_select(&self, graph: &mut Graph<Self::GraphData>, location: Pos2);
}

pub struct ContextMenuItem<M: ContextMenuData> {
    id: ContextMenuId,
    pub name: String,
    pub data: M,
}

impl<M> ContextMenuItem<M>
where
    M: ContextMenuData,
{
    pub fn new(id: ContextMenuId, name: String, data: M) -> Self {
        Self { id, name, data }
    }

    pub fn id(&self) -> ContextMenuId {
        self.id
    }
}

enum ContextMenuKind {
    SubMenu { name: String, items: Vec<ContextMenuKind> },
    LeafItem(ContextMenuId),
}

pub struct ContextMenu<M>
where
    M: ContextMenuData,
{
    items: SlotMap<ContextMenuId, ContextMenuItem<M>>,
    root: ContextMenuKind,
}

impl<M> Default for ContextMenu<M>
where
    M: ContextMenuData,
{
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

impl<M: ContextMenuData> ContextMenu<M> {
    pub fn builder(&mut self) -> ConextSubMenuBuilder<'_, M> {
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

pub struct ConextSubMenuBuilder<'m, M>
where
    M: ContextMenuData,
{
    menu_items: &'m mut SlotMap<ContextMenuId, ContextMenuItem<M>>,
    corrent: &'m mut ContextMenuKind,
}

impl<'m, M> ConextSubMenuBuilder<'m, M>
where
    M: ContextMenuData,
{
    pub fn add_item<S: ToString>(&mut self, name: S, data: M) -> &mut Self {
        let id = self
            .menu_items
            .insert_with_key(|id| ContextMenuItem::new(id, name.to_string(), data));
        if let ContextMenuKind::SubMenu { items, .. } = &mut self.corrent {
            items.push(ContextMenuKind::LeafItem(id));
            self
        } else {
            unreachable!()
        }
    }

    pub fn add_group<'n, S: ToString>(&'n mut self, name: S) -> ConextSubMenuBuilder<'n, M>
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
pub(in crate::node_graph) struct ContextMenuState<M>
where
    M: ContextMenuData,
{
    filter: String,
    start_location: Pos2,
    _ph: PhantomData<M>,
}

impl<M> Default for ContextMenuState<M>
where
    M: ContextMenuData,
{
    fn default() -> Self {
        ContextMenuState {
            filter: String::new(),
            start_location: pos2(0., 0.),
            _ph: PhantomData,
        }
    }
}

impl<M> ContextMenuState<M>
where
    M: ContextMenuData,
{
    pub fn load(ui: &mut Ui, id: Id) -> Option<Self> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    fn show_recursive(
        &self,
        menu_items: &SlotMap<ContextMenuId, ContextMenuItem<M>>,
        current: &ContextMenuKind,
        ui: &mut Ui,
        graph: &mut Graph<M::GraphData>,
    ) {
        match current {
            ContextMenuKind::SubMenu { name, items } => {
                ui.menu_button(name, |ui| {
                    for sub_item in items {
                        self.show_recursive(menu_items, sub_item, ui, graph);
                    }
                });
            }
            ContextMenuKind::LeafItem(menu_id) => {
                let item = menu_items.get(*menu_id).unwrap();
                if ui.button(&item.name).clicked() {
                    item.data.on_select(graph, self.start_location);
                    ui.close_menu();
                }
            }
        }
    }

    fn show_filtered(
        &self,
        menu_items: &SlotMap<ContextMenuId, ContextMenuItem<M>>,
        filter: &[&str],
        ui: &mut Ui,
        graph: &mut Graph<M::GraphData>,
    ) {
        for item in menu_items.values() {
            if filter.iter().any(|filter| item.name.starts_with(filter)) && ui.button(&item.name).clicked() {
                item.data.on_select(graph, self.start_location);
                ui.close_menu();
            }
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        content: &ContextMenu<M>,
        graph: &mut Graph<M::GraphData>,
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
                    self.show_recursive(&content.items, sub_item, ui, graph);
                }
            } else {
                unreachable!()
            }
        } else {
            self.show_filtered(&content.items, &filters, ui, graph);
        }
    }
}
