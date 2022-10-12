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
    pub types: SlotMap<PortTypeId, PortType>,
    pub nodes: SlotMap<NodeId, Node>,
    pub connections: SlotMap<ConnectionId, Connection>,
}

impl Graph {
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
        F: FnOnce(NodeId) -> Node,
    {
        self.nodes.insert_with_key(node)
    }

    /// Add a new connection to the graph with the given builder
    pub fn add_connection<F>(&mut self, connection: F) -> ConnectionId
    where
        F: FnOnce(ConnectionId) -> Connection,
    {
        self.connections.insert_with_key(connection)
    }
}
