use crate::node_graph::{NodeId, PortStyle};
use egui::Ui;
use shine_core::{
    downcast_rs::{impl_downcast, Downcast},
    smallbox::{smallbox, space, SmallBox},
};
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// Some dummy type for "null" input and output ids.
struct Void;

pub trait InputPort: 'static + Downcast {
    fn name(&self) -> &str;
    fn port_type_id(&self) -> TypeId;
    fn show(&self, ui: &mut Ui, style: &PortStyle);
}
impl_downcast!(InputPort);

pub type BoxedInput = SmallBox<dyn InputPort, space::S32>;

impl<T> From<Input<T>> for BoxedInput
where
    T: Any,
{
    fn from(input: Input<T>) -> Self {
        smallbox!(input)
    }
}

/// Input port
pub struct Input<T: Any> {
    pub name: String,
    _ph: PhantomData<T>,
}

impl<T: Any> Input<T> {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            _ph: PhantomData,
        }
    }
}

impl<T: Any> InputPort for Input<T> {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn port_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn show(&self, ui: &mut Ui, _style: &PortStyle) {
        ui.label(&self.name);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InputId(NodeId, TypeId, usize);

impl Default for InputId {
    fn default() -> Self {
        Self(NodeId::default(), TypeId::of::<Void>(), usize::MAX)
    }
}

impl InputId {
    pub(in crate::node_graph) fn new(node_id: NodeId, type_id: TypeId, signal_id: usize) -> Self {
        Self(node_id, type_id, signal_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn port_type_id(&self) -> TypeId {
        self.1
    }

    pub fn signal_id(&self) -> usize {
        self.2
    }
}

pub trait OutputPort: 'static + Downcast {
    fn name(&self) -> &str;
    fn port_type_id(&self) -> TypeId;
    fn show(&self, ui: &mut Ui, style: &PortStyle);
}

impl_downcast!(OutputPort);

pub type BoxedOutput = SmallBox<dyn OutputPort, space::S32>;

impl<T> From<Output<T>> for BoxedOutput
where
    T: Any,
{
    fn from(output: Output<T>) -> Self {
        smallbox!(output)
    }
}

/// Output port
pub struct Output<T: Any> {
    pub name: String,
    _ph: PhantomData<T>,
}

impl<T: Any> Output<T> {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            _ph: PhantomData,
        }
    }
}

impl<T: Any> OutputPort for Output<T> {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn port_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn show(&self, ui: &mut Ui, _style: &PortStyle) {
        ui.label(&self.name);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OutputId(NodeId, TypeId, usize);

impl Default for OutputId {
    fn default() -> Self {
        Self(NodeId::default(), TypeId::of::<Void>(), usize::MAX)
    }
}

impl OutputId {
    pub(in crate::node_graph) fn new(node_id: NodeId, type_id: TypeId, signal_id: usize) -> Self {
        Self(node_id, type_id, signal_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn port_type_id(&self) -> TypeId {
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

    pub fn port_type_id(&self) -> TypeId {
        match self {
            InputOutputId::Input(id) => id.port_type_id(),
            InputOutputId::Output(id) => id.port_type_id(),
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
