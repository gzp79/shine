use std::{borrow::Cow, fmt};

pub struct Node<N> {
    pub data: N,
}

pub struct Edge<E> {
    pub data: E,
    pub from: usize,
    pub to: usize,
}

/// A very simple and minimalistic graph representation
pub struct Graph<N, E> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    /// Construct the topology order of the nodes. If graph contains a cycle, None is returned.
    pub fn get_topology_order(&self) -> Option<Vec<usize>> {
        let mut order = Vec::new();

        let mut active = Vec::new();
        let mut degrees = Vec::with_capacity(self.nodes.len());
        let mut edges = self.edges.iter().map(|edge| (edge.from, edge.to)).collect::<Vec<_>>();

        // find degree for each node and colleect roots
        for n in 0..self.nodes.len() {
            let degree = edges.iter().filter(|(_, t)| n == *t).count();
            degrees.push(degree);
            if degree == 0 {
                active.push(n);
            }
        }

        while let Some(n) = active.pop() {
            order.push(n);

            edges.retain(|(f, t)| {
                if n == *f {
                    degrees[*t] -= 1;
                    if degrees[*t] == 0 {
                        active.push(*t);
                    }
                    false
                } else {
                    true
                }
            });
        }

        if edges.is_empty() {
            Some(order)
        } else {
            None
        }
    }
}

/// Customize dot vizualization.
pub trait DotAttribute {
    fn label(&self) -> Option<Cow<'_, str>> {
        None
    }

    fn font_size(&self) -> Option<u32> {
        None
    }
}

impl<N, E> Graph<N, E>
where
    N: DotAttribute,
    E: DotAttribute,
{
    fn write_attributes<W: fmt::Write>(&self, f: &mut W, data: &dyn DotAttribute) -> Result<(), fmt::Error> {
        let mut sep = '[';
        if let Some(label) = data.label() {
            write!(f, "{}label=\"{}\"", sep, label)?;
            sep = ',';
        }

        if let Some(font_size) = data.font_size() {
            write!(f, "{}fontsize=\"{}pt\"", sep, font_size)?;
            sep = ',';
        }

        if sep == ',' {
            write!(f, "]")?;
        }

        Ok(())
    }

    /// Write a graphviz compatible graph.
    pub fn write_dot_graph<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        writeln!(f, "digraph G {{")?;
        for (n, d) in self.nodes.iter().enumerate() {
            write!(f, "N_{}", n)?;
            self.write_attributes(f, &d.data)?;
            writeln!(f)?;
        }
        write!(f, "")?;

        for edge in &self.edges {
            write!(f, "N_{} -> N_{}", edge.from, edge.to)?;
            self.write_attributes(f, &edge.data)?;
            writeln!(f)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }

    /// Return a graphviz compatible graph.
    /// For example http://www.webgraphviz.com/ can be used to visualize the returnd string.
    pub fn dot_graph(&self) -> String {
        let mut dot = String::new();
        self.write_dot_graph(&mut dot).unwrap();
        dot
    }
}
