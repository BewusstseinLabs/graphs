use crate::graph::{
    error::Error,
    GraphTraits,
    graph_data::{ GraphData, GraphDataTraits }
};

//: Standard
use std::collections::{ BTreeSet, VecDeque };
use std::fmt::Display;

pub struct DirectedGraph<I, N, E> {
    data: GraphData<I, N, E>,
}

impl<I, N, E> DirectedGraph<I, N, E>
where
    I: Clone + Ord + Display,
    N: Display,
    E: Display,
{
    pub fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node1, (adjacencies, data)) in self.data.get_nodes() {
            dot.push_str(&format!(" {} [label=\"{}\"];\n", node1, data));
            for (node2, data) in adjacencies {
                dot.push_str(&format!(" {} -> {} [label=\"{}\"];\n", node1, node2, data));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}

impl<I, N, E> GraphTraits<I, N, E> for DirectedGraph<I, N, E>
where
    I: Clone + Ord + Display,
    N: Display,
    E: Display,
{
    fn new() -> Self {
        Self {
            data: GraphData::new(),
        }
    }

    fn add_node(&mut self, node: I, data: N) -> Result<(), Error> {
        self.data.add_node(node, data)?;
        Ok( () )
    }

    fn get_node(&self, node: I) -> Option<&N> {
        self.data.get_node(node)
    }

    fn get_node_mut(&mut self, node: I) -> Option<&mut N> {
        self.data.get_node_mut(node)
    }

    fn contains_node(&self, node: I) -> bool {
        self.data.contains_node(node)
    }

    fn remove_node(&mut self, node: I) -> Result<N, Error> {
        self.data.remove_node(node)
    }

    fn delete_node(&mut self, node: I) -> Result<(), Error> {
        self.data.remove_node(node)?;
        Ok( () )
    }

    fn add_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error> {
        if self.data.contains_edge(node1.clone(), node2.clone()) {
            return Err(Error::EdgeAlreadyExists);
        }
        self.data.add_edge(node1, node2, data)?;
        Ok( () )
    }

    fn get_edge(&self, node1: I, node2: I) -> Option<&E> {
        self.data.get_edge(node1, node2)
    }

    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut E> {
        self.data.get_edge_mut(node1, node2)
    }

    fn contains_edge(&self, node1: I, node2: I) -> bool {
        self.data.contains_edge(node1, node2)
    }

    fn remove_edge(&mut self, node1: I, node2: I) -> Result<E, Error> {
        self.data.remove_edge(node1, node2)
    }

    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error> {
        self.data.remove_edge(node1, node2)?;
        Ok( () )
    }

    fn bfs(&mut self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while !queue.is_empty() {
            if let ( Some( current_id ), Some( next_id ) ) = self.data.bfs_step(&mut queue, &mut visited) {
                println!("Current: {} -> Next: {}", current_id, next_id);
            }
        }
    }

    fn dfs(&mut self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while !stack.is_empty() {
            if let ( Some( current_id ), Some( next_id ) ) = self.data.dfs_step(&mut stack, &mut visited) {
                println!("Current: {} -> Next: {}", current_id, next_id);
            }
        }
    }
}