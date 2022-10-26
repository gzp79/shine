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
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}
impl_downcast!(InputPort);

pub type BoxedInput = SmallBox<dyn InputPort, space::S32>;

pub trait InputData : 'static + Send +Sync {
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}

impl InputData for () {
    fn show(&mut self, _ui: &mut Ui, _style: &PortStyle) {}
}

/// Input port
pub struct Input<T, I = ()> where T: Any, I: InputData {
    pub name: String,
    pub data: I,
    _ph: PhantomData<T>,
}

impl<T, I> Input<T, I> where T: Any, I: InputData {
    pub fn new<S: ToString>(name: S, data: I) -> Self {
        Self {
            name: name.to_string(),
            data,
            _ph: PhantomData,
        }
    }
}

impl<T, I> InputPort for Input<T, I> where T: Any, I: InputData {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn port_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn show(&mut self, ui: &mut Ui, style: &PortStyle) {
        ui.label(&self.name);
        self.data.show(ui, style);
    }
}

impl<T, I> From<Input<T, I>> for BoxedInput
where
    T: Any,
    I: InputData
{
    fn from(input: Input<T, I>) -> Self {
        smallbox!(input)
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
    pub(in crate::node_graph) fn new(node_id: NodeId, type_id: TypeId, port_id: usize) -> Self {
        Self(node_id, type_id, port_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn port_type_id(&self) -> TypeId {
        self.1
    }

    pub fn port_id(&self) -> usize {
        self.2
    }
}

pub trait OutputPort: 'static + Downcast {
    fn name(&self) -> &str;
    fn port_type_id(&self) -> TypeId;
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}

impl_downcast!(OutputPort);

pub type BoxedOutput = SmallBox<dyn OutputPort, space::S32>;

pub trait OutputData : 'static + Send + Sync {
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}

impl OutputData for () {
    fn show(&mut self, _ui: &mut Ui, _style: &PortStyle) {}
}

/// Output port
pub struct Output<T, O = ()>
where
    T: Any,
    O: OutputData,
{
    pub name: String,
    pub data: O,
    _ph: PhantomData<T>,
}

impl<T, O> Output<T, O>
where
    T: Any, O: OutputData
{
    pub fn new<S: ToString>(name: S, data: O) -> Self {
        Self {
            name: name.to_string(),
            data,
            _ph: PhantomData,
        }
    }
}

impl<T, O> OutputPort for Output<T, O>
where
    T: Any,
    O: OutputData
{
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn port_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn show(&mut self, ui: &mut Ui, style: &PortStyle) {
        ui.label(&self.name);
        self.data.show(ui, style);
    }
}

impl<T, O> From<Output<T, O>> for BoxedOutput
where
    T: Any,
    O: OutputData,
{
    fn from(output: Output<T, O>) -> Self {
        smallbox!(output)
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
    pub(in crate::node_graph) fn new(node_id: NodeId, type_id: TypeId, port_id: usize) -> Self {
        Self(node_id, type_id, port_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }

    pub fn port_type_id(&self) -> TypeId {
        self.1
    }

    pub fn port_id(&self) -> usize {
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
