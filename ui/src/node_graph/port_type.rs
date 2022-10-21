use egui::Color32;
use slotmap::new_key_type;

new_key_type! { pub struct PortTypeId; }

#[derive(Clone, Debug)]
pub struct PortType {
    pub name: String,
    pub port_size: f32,
    pub connection_width: f32,
    pub color: Color32,
    pub hover_color: Color32,
    pub error_color: Color32,
}

impl PortType {
    pub fn new<S: ToString>(name: S) -> Self {
        PortType {
            name: name.to_string(),
            port_size: 7.,
            connection_width: 3.,
            color: Color32::WHITE,
            hover_color: Color32::BLUE,
            error_color: Color32::RED,
        }
    }

    pub fn unknown() -> Self {
        PortType {
            name: "unknown".to_string(),
            port_size: 7.,
            connection_width: 3.,
            color: Color32::RED,
            hover_color: Color32::RED,
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