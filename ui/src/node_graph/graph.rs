use crate::node_graph::{
    Connection, ConnectionData, ConnectionId, InputId, Node, NodeData, NodeId, OutputId, PortType, PortTypeId,
};
use slotmap::SlotMap;

pub trait GraphData: Clone + Send + Sync + 'static {
    type NodeData: NodeData;
    type ConnectionData: ConnectionData;

    fn create_connection_data(&mut self, input: InputId, output: OutputId) -> Option<Self::ConnectionData>;
}

/// The node graph.
pub struct Graph<G>
where
    G: GraphData,
{
    pub types: SlotMap<PortTypeId, PortType>,
    pub nodes: SlotMap<NodeId, Node<G::NodeData>>,
    pub connections: SlotMap<ConnectionId, Connection<G::ConnectionData>>,
    pub data: G,
}

impl<G> Default for Graph<G>
where
    G: Default + GraphData,
{
    fn default() -> Self {
        Self {
            types: SlotMap::default(),
            nodes: SlotMap::default(),
            connections: SlotMap::default(),
            data: G::default(),
        }
    }
}

impl<G> Graph<G>
where
    G: GraphData,
{
    /// Create a new graph with the given user data
    pub fn new_with_data(data: G) -> Self {
        Self {
            types: SlotMap::default(),
            nodes: SlotMap::default(),
            connections: SlotMap::default(),
            data,
        }
    }

    /// Create a new port-type.
    pub fn add_type(&mut self, port: PortType) -> PortTypeId {
        self.types.insert(port)
    }

    /// Get the type corresponding to the id or `PortType::unknown()` in case it is not found
    pub fn get_type(&self, port_id: PortTypeId) -> PortType {
        self.types.get(port_id).cloned().unwrap_or_else(PortType::unknown)
    }

    /// Add a new node to the graph with the given builder.
    pub fn add_node<F>(&mut self, node: F) -> NodeId
    where
        F: FnOnce(NodeId) -> Node<G::NodeData>,
    {
        self.nodes.insert_with_key(node)
    }

    /// Remove a node with its connections from the graph
    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.remove(node_id);
        // also remove connections
        self.connections.retain(|_, connection| {
            connection.input_id.node_id() != node_id && connection.output_id.node_id() != node_id
        })
    }

    /// Add a new connection to the graph with the given builder
    pub fn add_connection<F>(&mut self, connection: F) -> ConnectionId
    where
        F: FnOnce(ConnectionId) -> Connection<G::ConnectionData>,
    {
        self.connections.insert_with_key(connection)
    }

    /// Clear the graph, but keeps the allocated memory.
    pub fn clear(&mut self) {
        self.types.clear();
        self.nodes.clear();
        self.connections.clear();
    }
}
