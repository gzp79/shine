use crate::node_graph::{
    Connection, ConnectionId, ContextMenuId, InputId, Node, NodeId, OutputId, PortType, PortTypeId,
};
use egui::Pos2;
use slotmap::SlotMap;

/// Update actions on the graph
#[derive(Clone, Debug)]
pub enum GraphOperation {
    ContextMenu(Pos2, ContextMenuId),
    SetNodeLocation(NodeId, Pos2),
    Connect(InputId, OutputId),
}

/// The node graph.
#[derive(Default)]
pub struct Graph {
    type_infos: SlotMap<PortTypeId, PortType>,
    nodes: SlotMap<NodeId, Node>,
    connections: SlotMap<ConnectionId, Connection>,
}

impl Graph {
    /// Create a new port-type.
    pub fn add_type(&mut self, port: PortType) -> PortTypeId {
        self.type_infos.insert(port)
    }

    /// Get type info by the id.
    pub fn get_type(&self, type_id: PortTypeId) -> Option<&PortType> {
        self.type_infos.get(type_id)
    }

    /// Add a new node to the graph with the given builder.
    pub fn add_node<F>(&mut self, node: F) -> NodeId
    where
        F: FnOnce(NodeId) -> Node,
    {
        self.nodes.insert_with_key(node)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    /// Add a new connection to the graph with the given builder
    pub fn add_connection<F>(&mut self, connection: F) -> ConnectionId
    where
        F: FnOnce(ConnectionId) -> Connection,
    {
        self.connections.insert_with_key(connection)
    }

    /// Get all the connections of the graph
    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.values()
    }

    pub fn connection(&self, connection_id: ConnectionId) -> Option<&Connection> {
        self.connections.get(connection_id)
    }

    pub fn connection_mut(&mut self, connection_id: ConnectionId) -> Option<&mut Connection> {
        self.connections.get_mut(connection_id)
    }
}
