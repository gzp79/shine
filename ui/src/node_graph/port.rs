use crate::node_graph::{NodeId, PortTypeId};
use egui::Ui;

/// Input port
pub struct Input {
    pub name: String,
    pub type_id: PortTypeId,
}

impl Input {
    pub fn new<S: ToString>(name: S, type_id: PortTypeId) -> Self {
        Self {
            name: name.to_string(),
            type_id,
        }
    }

    pub(in crate::node_graph) fn show(&self, ui: &mut Ui) {
        ui.label(&self.name);
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InputId(NodeId, PortTypeId, usize);

impl InputId {
    pub fn new(node_id: NodeId, type_id: PortTypeId, signal_id: usize) -> Self {
        Self(node_id, type_id, signal_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn type_id(&self) -> PortTypeId {
        self.1
    }

    pub fn signal_id(&self) -> usize {
        self.2
    }
}

/// Output port
pub struct Output {
    pub name: String,
    pub type_id: PortTypeId,
}

impl Output {
    pub fn new<S: ToString>(name: S, type_id: PortTypeId) -> Self {
        Self {
            name: name.to_string(),
            type_id,
        }
    }

    pub(in crate::node_graph) fn show(&self, ui: &mut Ui) -> f32 {
        let height_before = ui.min_rect().bottom();
        ui.label(&self.name);
        let height_after = ui.min_rect().bottom();
        (height_before + height_after) / 2.
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OutputId(NodeId, PortTypeId, usize);

impl OutputId {
    pub fn new(node_id: NodeId, type_id: PortTypeId, signal_id: usize) -> Self {
        Self(node_id, type_id, signal_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn type_id(&self) -> PortTypeId {
        self.1
    }

    pub fn signal_id(&self) -> usize {
        self.2
    }
}

/// An id to an input or output port.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InputOutputId {
    Input(InputId),
    Output(OutputId),
}

impl InputOutputId {
    pub fn is_input(&self) -> bool {
        matches!(self, InputOutputId::Input(_))
    }

    pub fn is_output(&self) -> bool {
        matches!(self, InputOutputId::Output(_))
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            InputOutputId::Input(id) => id.node_id(),
            InputOutputId::Output(id) => id.node_id(),
        }
    }

    pub fn type_id(&self) -> PortTypeId {
        match self {
            InputOutputId::Input(id) => id.type_id(),
            InputOutputId::Output(id) => id.type_id(),
        }
    }
}

impl From<InputId> for InputOutputId {
    fn from(input_id: InputId) -> Self {
        InputOutputId::Input(input_id)
    }
}

impl From<OutputId> for InputOutputId {
    fn from(output_id: OutputId) -> Self {
        InputOutputId::Output(output_id)
    }
}
