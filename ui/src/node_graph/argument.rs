use crate::node_graph::GraphOperation;
use egui::Ui;

pub trait Argument {
    fn show(&self, ui: &mut Ui, _operations: &mut Vec<GraphOperation>);
}
