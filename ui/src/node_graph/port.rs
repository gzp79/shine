use crate::node_graph::{NodeId, PortStyle};
use egui::Ui;
use shine_core::{
    downcast_rs::{impl_downcast, Downcast},
    smallbox::{smallbox, space, SmallBox},
};
use std::any::{Any, TypeId};

/// Some dummy type for "null" input and output ids.
struct Void;

pub trait InputPortData: 'static + Downcast + Send + Sync {
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}
impl_downcast!(InputPortData);

impl InputPortData for () {
    fn show(&mut self, _ui: &mut Ui, _style: &PortStyle) {}
}

type BoxedInputPortData = SmallBox<dyn InputPortData, space::S2>;

/// Input port
pub struct Input {
    pub name: String,
    port_type_id: TypeId,
    data: BoxedInputPortData,
}

impl Input {
    pub fn new<T: Any>(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            port_type_id: TypeId::of::<T>(),
            data: smallbox!(()),
        }
    }

    pub fn with<I>(self, data: I) -> Self
    where
        I: InputPortData,
    {
        Input {
            data: smallbox!(data),
            ..self
        }
    }

    pub fn port_type_id(&self) -> TypeId {
        self.port_type_id
    }

    pub fn data(&self) -> &dyn InputPortData {
        &*self.data
    }

    pub fn data_as<T: InputPortData>(&self) -> &T {
        let data = &*self.data;
        data.downcast_ref::<T>().unwrap()
    }

    pub fn data_mut_as<T: InputPortData>(&mut self) -> &mut T {
        let data = &mut *self.data;
        data.downcast_mut::<T>().unwrap()
    }

    pub fn show(&mut self, ui: &mut Ui, style: &PortStyle) {
        ui.label(&self.name);        
        self.data.show(ui, style);
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
pub trait OutputPortData: 'static + Downcast + Send + Sync {
    fn show(&mut self, ui: &mut Ui, style: &PortStyle);
}
impl_downcast!(OutputPortData);

impl OutputPortData for () {
    fn show(&mut self, _ui: &mut Ui, _style: &PortStyle) {}
}

type BoxedOutputPortData = SmallBox<dyn OutputPortData, space::S2>;

/// Output port
pub struct Output {
    pub name: String,
    port_type_id: TypeId,
    data: BoxedOutputPortData,
}

impl Output {
    pub fn new<T: Any>(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            port_type_id: TypeId::of::<T>(),
            data: smallbox!(()),
        }
    }

    pub fn with<I>(self, data: I) -> Self
    where
        I: OutputPortData,
    {
        Output {
            data: smallbox!(data),
            ..self
        }
    }

    pub fn port_type_id(&self) -> TypeId {
        self.port_type_id
    }

    pub fn data(&self) -> &dyn OutputPortData {
        &*self.data
    }

    pub fn data_as<T: OutputPortData>(&self) -> &T {
        let data = &*self.data;
        data.downcast_ref::<T>().unwrap()
    }

    pub fn data_mut_as<T: OutputPortData>(&mut self) -> &mut T {
        let data = &mut *self.data;
        data.downcast_mut::<T>().unwrap()
    }

    pub fn show(&mut self, ui: &mut Ui, style: &PortStyle) {
        ui.label(&self.name);
        self.data.show(ui, style);
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
