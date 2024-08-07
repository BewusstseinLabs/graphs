use crate::graph::{
    error::Error,
    GraphTraits,
    graph_data::{ GraphData, GraphDataTraits }
};

//: Standard
use std::collections::{ BTreeSet, VecDeque };
use std::fmt::Display;

pub struct FunctionGraph<I, N, F> {
    data: GraphData<I, N, F>,
}

impl<I, N, E> FunctionGraph<I, N, E>
where
    I: Clone + Ord + PartialEq + Display,
    N: PartialEq + Display,
    E: PartialEq,
{
    pub fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node1, (adjacencies, data)) in self.data.get_nodes() {
            dot.push_str(&format!(" {} [label=\"{}\"];\n", node1, data));
            for (node2, _) in adjacencies {
                dot.push_str(&format!(" {} -> {} [label=\"fn\"];\n", node1, node2));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}

impl<I, N, F> GraphTraits<I, N, F> for FunctionGraph<I, N, F>
where
    I: Clone + Ord + PartialEq + Display,
    N: PartialEq + Default,
    F: PartialEq + Fn(&N, &N) -> N,
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

    fn add_edge(&mut self, node1: I, node2: I, data: F) -> Result<(), Error> {
        if self.data.contains_edge(node1.clone(), node2.clone()) {
            return Err(Error::EdgeAlreadyExists);
        }
        self.data.add_edge(node1, node2, data)?;
        Ok( () )
    }

    fn get_edge(&self, node1: I, node2: I) -> Option<&F> {
        self.data.get_edge(node1, node2)
    }

    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut F> {
        self.data.get_edge_mut(node1, node2)
    }

    fn contains_edge(&self, node1: I, node2: I) -> bool {
        self.data.contains_edge(node1, node2)
    }

    fn remove_edge(&mut self, node1: I, node2: I) -> Result<F, Error> {
        self.data.remove_edge(node1, node2)
    }

    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error> {
        self.data.remove_edge(node1, node2)?;
        Ok( () )
    }

    fn clear( &mut self ) {
        self.data.clear();
    }
    fn clear_edges( &mut self ) {
        self.data.clear_edges();
    }

    fn bfs(&mut self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while !queue.is_empty() {
            if let ( Some( current_id ), Some( next_id ) ) = self.data.bfs_step(&mut queue, &mut visited) {
                let mut result = N::default();
                if let Some( edge_data ) = self.data.get_edge( current_id.clone(), next_id.clone() ) {
                    if let Some( current_data ) = self.data.get_node( current_id.clone() ) {
                        result = edge_data( current_data, &N::default() );
                        println!("Current: {} -> Next: {}", current_id, next_id);
                    }
                }
                if let Some( current_data ) = self.data.get_node_mut( current_id ) {
                    *current_data = result;
                }
            }
        }
    }

    fn dfs(&mut self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while !stack.is_empty() {
            if let ( Some( current_id ), Some( next_id ) ) = self.data.dfs_step(&mut stack, &mut visited) {
                let mut result = N::default();
                if let Some( edge_data ) = self.data.get_edge( current_id.clone(), next_id.clone() ) {
                    if let Some( current_data ) = self.data.get_node( current_id.clone() ) {
                        result = edge_data( current_data, &N::default() );
                        println!("Current: {} -> Next: {}", current_id, next_id);
                    }
                }
                if let Some( current_data ) = self.data.get_node_mut( current_id ) {
                    *current_data = result;
                }
            }
        }
    }

    fn is_complete( graph: &Self ) -> bool {
        GraphDataTraits::is_complete(&graph.data)
    }

    fn is_empty( graph: &Self ) -> bool {
        GraphDataTraits::is_empty(&graph.data)
    }

    fn is_trivial( graph: &Self ) -> bool {
        GraphDataTraits::is_trivial(&graph.data)
    }

    fn is_null( graph: &Self ) -> bool {
        GraphDataTraits::is_null(&graph.data)
    }

    fn is_child_node( graph: &Self, node_1: I ) -> bool {
        GraphDataTraits::is_child_node(&graph.data, node_1)
    }

    fn is_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_subgraph(&graph.data, &subgraph.data)
    }

    fn is_proper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_proper_subgraph(&graph.data, &subgraph.data)
    }

    fn is_improper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_improper_subgraph(&graph.data, &subgraph.data)
    }

    fn is_spanning_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_spanning_subgraph(&graph.data, &subgraph.data)
    }

    fn are_adjacent_nodes( graph: &Self, node_1: I, node_2: I ) -> bool {
        GraphDataTraits::are_adjacent_nodes(&graph.data, node_1, node_2)
    }

    fn are_adjacent_edges( graph: &Self, node_1: I, node_2: I, node_3: I ) -> bool {
        GraphDataTraits::are_adjacent_edges(&graph.data, node_1, node_2, node_3)
    }

    fn order( graph: &Self ) -> usize {
        GraphDataTraits::order(&graph.data)
    }

    fn size( graph: &Self ) -> usize {
        GraphDataTraits::size(&graph.data)
    }
}