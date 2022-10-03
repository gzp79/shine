use crate::node_graph::{Connection, ConnectionId, InputId, Node, NodeBuilder, NodeId, OutputId, PortType, PortTypeId};
use egui::Pos2;
use slotmap::SlotMap;
use std::sync::Arc;

/// Update actions on the graph
pub enum GraphOperation {
    AddNode(Pos2, Arc<dyn NodeBuilder>),
    SetNodeLocation(NodeId, Pos2),
    Connect(InputId, OutputId),
}

impl std::fmt::Debug for GraphOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddNode(pos, _arg1) => f.debug_tuple("AddNode").field(pos).finish(),
            Self::SetNodeLocation(node_id, pos) => f.debug_tuple("SetNodeLocation").field(node_id).field(pos).finish(),
            Self::Connect(input_id, output_id) => f.debug_tuple("Connect").field(input_id).field(output_id).finish(),
        }
    }
}

/// The node graph.
#[derive(Default)]
pub struct Graph {
    type_infos: SlotMap<PortTypeId, PortType>,
    nodes: SlotMap<NodeId, Node>,
    //node_datas: SecondaryMap<NodeId, NodeData>,
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

    pub fn apply_action(&mut self, operation: GraphOperation) {
        match operation {
            GraphOperation::AddNode(pos, builder) => {
                let _ = self.add_node(|node_id| builder.build(node_id, pos));
            }
            GraphOperation::Connect(input_id, output_id) => {
                let _ = self.add_connection(|connection_id| Connection::new(connection_id, input_id, output_id));
            }
            GraphOperation::SetNodeLocation(node_id, pos) => {
                if let Some(node) = self.node_mut(node_id) {
                    node.set_location(pos);
                }
            }
        }
    }
}
