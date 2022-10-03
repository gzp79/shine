use shine_core::graph::{DotAttribute, Edge, Graph, Node};
use shine_test::test;

struct NodeData(usize);
impl DotAttribute for NodeData {}

struct EdgeData;
impl DotAttribute for EdgeData {}

fn n(i: usize) -> Node<NodeData> {
    Node { data: NodeData(i) }
}

fn e(f: usize, t: usize) -> Edge<EdgeData> {
    Edge {
        from: f,
        to: t,
        data: EdgeData,
    }
}

#[test]
fn case_1() {
    let nodes = vec![n(0), n(1), n(2), n(3)];
    let edges = vec![e(1, 0), e(2, 0), e(3, 0)];
    let graph = Graph { nodes, edges };

    log::trace!("{}", graph.dot_graph());
    assert_eq!(graph.get_topology_order(), Some(vec![3, 2, 1, 0]));
}

#[test]
fn case_2() {
    let nodes = vec![n(0), n(1), n(2), n(3), n(4), n(5)];
    let edges = vec![e(5, 0), e(4, 0), e(4, 1), e(1, 3), e(2, 3), e(5, 2)];
    let graph = Graph { nodes, edges };

    log::trace!("{}", graph.dot_graph());
    assert_eq!(graph.get_topology_order(), Some(vec![5, 2, 4, 1, 3, 0]));
}

#[test]
fn case_3() {
    let nodes = vec![n(0), n(1), n(2), n(3)];
    let edges = vec![e(0, 1), e(1, 2), e(2, 3), e(3, 1)];
    let graph = Graph { nodes, edges };

    log::trace!("{}", graph.dot_graph());
    assert_eq!(graph.get_topology_order(), None);
}
