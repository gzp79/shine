use crate::node_graph::{Argument, GraphOperation};
use egui::Ui;

pub struct HelloWorld;

impl Argument for HelloWorld {
    fn show(&self, ui: &mut Ui, _operations: &mut Vec<GraphOperation>) {
        ui.label("Hello, world!");
    }
}
