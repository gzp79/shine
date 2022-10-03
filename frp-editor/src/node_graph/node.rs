use crate::node_graph::{Graph, GraphOperation, Input, Output, PortViewState, PrimitiveNode, ZoomPanState};
use egui::{Pos2, Response, Ui};
use slotmap::new_key_type;

new_key_type! { pub struct NodeId; }

pub trait NodeBuilder: 'static {
    fn build(&self, node_id: NodeId, location: Pos2) -> Node;
}

impl<F> NodeBuilder for F
where
    F: Fn(NodeId, Pos2) -> Node + 'static,
{
    fn build(&self, node_id: NodeId, location: Pos2) -> Node {
        (self)(node_id, location)
    }
}

pub enum Node {
    Primitive(PrimitiveNode),
    //Api(ApiNode),  io from group into the internal,
    //Group(GroupNode)
}

impl Node {
    pub fn primitive<S: ToString>(
        node_id: NodeId,
        caption: S,
        location: Pos2,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Self {
        Node::Primitive(PrimitiveNode::new(
            node_id,
            caption.to_string(),
            inputs,
            outputs,
            location,
        ))
    }

    pub fn set_location(&mut self, location: Pos2) {
        match self {
            Node::Primitive(node) => node.set_location(location),
        }
    }

    pub(in crate::node_graph) fn show(
        &self,
        ui: &mut Ui,
        zoom_pan: &ZoomPanState,
        port_visual: &mut PortViewState,
        graph: &Graph,
        operations: &mut Vec<GraphOperation>,
    ) -> Response {
        match self {
            Node::Primitive(node) => node.show(ui, zoom_pan, port_visual, graph, operations),
        }
    }
}
