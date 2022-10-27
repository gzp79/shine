use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use egui::Color32;

#[derive(Clone, Debug)]
pub struct PortStyle {
    pub name: String,
    pub port_size: f32,
    pub connection_width: f32,
    pub color: Color32,
    pub hover_color: Color32,
    pub error_color: Color32,
}

impl PortStyle {
    pub fn new<S: ToString>(name: S) -> Self {
        PortStyle {
            name: name.to_string(),
            port_size: 5.,
            connection_width: 3.,
            color: Color32::WHITE,
            hover_color: Color32::BLUE,
            error_color: Color32::RED,
        }
    }

    pub fn with_port_size(self, port_size: f32) -> Self {
        Self { port_size, ..self }
    }

    pub fn with_color(self, color: Color32) -> Self {
        Self { color, ..self }
    }

    pub fn with_hover_color(self, hover_color: Color32) -> Self {
        Self { hover_color, ..self }
    }

    pub fn with_error_color(self, error_color: Color32) -> Self {
        Self { error_color, ..self }
    }
}

#[derive(Default)]
pub struct PortStyles {
    styles: HashMap<TypeId, PortStyle>,
}

impl PortStyles {
    pub fn set<T: Any>(&mut self, port: PortStyle) {
        let ty = TypeId::of::<T>();
        self.styles.insert(ty, port);
    }

    pub fn remove<T: Any>(&mut self) {
        let ty = TypeId::of::<T>();
        self.styles.remove(&ty);
    }

    pub fn get<T: Any>(&self) -> Option<&PortStyle> {
        let ty = TypeId::of::<T>();
        self.styles.get(&ty)
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut PortStyle> {
        let ty = TypeId::of::<T>();
        self.styles.get_mut(&ty)
    }

    pub fn find(&self, ty: TypeId) -> Option<&PortStyle> {
        self.styles.get(&ty)
    }
}
