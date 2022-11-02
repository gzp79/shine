use crate::node_graph::{Connection, ConnectionId, Input, InputId, Node, NodeId, Output, OutputId, PortStyles};
use shine_core::{
    downcast_rs::{impl_downcast, Downcast},
    slotmap::SlotMap,
    smallbox::{smallbox, space, SmallBox},
};
use std::{collections::HashMap, sync::Arc};

pub trait Validator: 'static + Downcast + Send + Sync {
    fn try_create_connection(&self, graph: &Graph, input_id: InputId, output_id: OutputId) -> Option<Connection>;
}
impl_downcast!(Validator);

/// The default validator that
pub struct DefaultValidator;
impl Validator for DefaultValidator {
    fn try_create_connection(&self, _graph: &Graph, input_id: InputId, output_id: OutputId) -> Option<Connection> {
        if input_id.port_type_id() == output_id.port_type_id() {
            Some(Connection::new(input_id, output_id, ()))
        } else {
            None
        }
    }
}

pub trait GraphData: 'static + Downcast + Send + Sync {}
impl_downcast!(GraphData);

impl GraphData for () {}

type BoxedGraphData = SmallBox<dyn GraphData, space::S32>;

/// The node graph.
pub struct Graph {
    styles: Arc<PortStyles>,
    nodes: SlotMap<NodeId, Node>,
    connections: SlotMap<ConnectionId, Connection>,
    connection_map: HashMap<(InputId, OutputId), ConnectionId>,
    data: BoxedGraphData,
    validator: Box<dyn Validator>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            styles: Arc::new(PortStyles::default()),
            nodes: SlotMap::default(),
            connections: SlotMap::default(),
            connection_map: HashMap::new(),
            data: smallbox!(()),
            validator: Box::new(DefaultValidator),
        }
    }
}

impl Graph {
    pub fn with<G: GraphData>(self, data: G) -> Self {
        Self {
            data: smallbox!(data),
            ..self
        }
    }

    pub fn get_port_styles(&self) -> &Arc<PortStyles> {
        &self.styles
    }

    pub fn set_port_styles<S: Into<Arc<PortStyles>>>(&mut self, style: S) {
        self.styles = style.into()
    }

    pub fn set_validator<V: Validator>(&mut self, validator: V) {
        self.validator = Box::new(validator);
    }

    pub fn validator(&self) -> &dyn Validator {
        &*self.validator
    }

    /// Clear the graph, but keeps the allocated memory.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.connections.clear();
    }

    /// Add a new node to the graph with the given builder.
    pub fn add_node(&mut self, node: Node) -> NodeId {
        self.nodes.insert_with_key(|node_id| node.with_id(node_id))
    }

    /// Remove a node with its connections from the graph.
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove(node_id);

        self.connections.retain(|_, connection| {
            if connection.input_node_id() == node_id && connection.output_node_id() == node_id {
                let key = (connection.input_id(), connection.output_id());
                self.connection_map.remove(&key);
                false
            } else {
                true
            }
        })
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }

    pub fn node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    /// Add a new connection to the graph with the given builder.
    /// # Panics
    /// This function will panic if there is a connection between these two ports.
    pub fn add_connection(&mut self, connection: Connection) -> ConnectionId {
        let key = (connection.input_id(), connection.output_id());
        assert!(!self.connection_map.contains_key(&key));
        let connection_id = self
            .connections
            .insert_with_key(|connection_id| connection.with_id(connection_id));
        self.connection_map.insert(key, connection_id);
        connection_id
    }

    pub fn remove_connection(&mut self, connection_id: ConnectionId) {
        self.connections.remove(connection_id);
    }

    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values()
    }

    pub fn connections_mut(&mut self) -> impl Iterator<Item = &mut Connection> {
        self.connections.values_mut()
    }

    pub fn connection(&self, connection_id: ConnectionId) -> Option<&Connection> {
        self.connections.get(connection_id)
    }

    pub fn connection_mut(&mut self, connection_id: ConnectionId) -> Option<&mut Connection> {
        self.connections.get_mut(connection_id)
    }

    pub fn find_connections(&self, input_id: InputId, output_id: OutputId) -> Option<ConnectionId> {
        self.connection_map.get(&(input_id, output_id)).cloned()
    }

    pub fn get_input(&self, input_id: InputId) -> Option<&Input> {
        self.nodes
            .get(input_id.node_id())
            .and_then(|node| node.inputs.get(input_id.port_id()))
    }

    pub fn get_output(&self, output_id: OutputId) -> Option<&Output> {
        self.nodes
            .get(output_id.node_id())
            .and_then(|node| node.outputs.get(output_id.port_id()))
    }

    pub fn data(&self) -> &dyn GraphData {
        &*self.data
    }

    pub fn data_as<T: GraphData>(&self) -> &T {
        let data = &*self.data;
        data.downcast_ref::<T>().unwrap()
    }

    pub fn data_mut_as<T: GraphData>(&mut self) -> &mut T {
        let data = &mut *self.data;
        data.downcast_mut::<T>().unwrap()
    }
}
