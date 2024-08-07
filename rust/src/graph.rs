pub mod error;
mod graph_data;
pub mod undirected_graph;
pub mod directed_graph;
pub mod function_graph;
mod test;

use std::{
    ops::{
        Add, AddAssign,
        Sub, SubAssign,
    },
    cmp::{
        Eq,
        Ord
    },
    fmt::Display
};

use self::error::Error;
pub use self::undirected_graph::UndirectedGraph;
pub use self::directed_graph::DirectedGraph;
pub use self::function_graph::FunctionGraph;

pub trait GraphTraits<I, N, E>
where
    I: Clone + Ord,
{
    fn new() -> Self;
    fn add_node(&mut self, node: I, data: N) -> Result<(), Error>;
    fn get_node(&self, node: I) -> Option<&N>;
    fn get_node_mut(&mut self, node: I) -> Option<&mut N>;
    fn contains_node(&self, node: I) -> bool;
    fn remove_node(&mut self, node: I) -> Result<N, Error>;
    fn delete_node(&mut self, node: I) -> Result<(), Error>;
    fn add_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error>;
    fn get_edge(&self, node1: I, node2: I) -> Option<&E>;
    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut E>;
    fn contains_edge(&self, node1: I, node2: I) -> bool;
    fn remove_edge(&mut self, node1: I, node2: I) -> Result<E, Error>;
    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error>;
    fn clear( &mut self );
    fn clear_edges( &mut self );
    fn bfs(&mut self, start: I);
    fn dfs(&mut self, start: I);
    fn is_complete( graph: &Self ) -> bool;
    fn is_empty( graph: &Self ) -> bool;
    fn is_trivial( graph: &Self ) -> bool;
    fn is_null( graph: &Self ) -> bool;
    fn is_child_node( graph: &Self, node_1: I ) -> bool;
    fn is_subgraph( graph: &Self, subgraph: &Self ) -> bool;
    fn is_proper_subgraph( graph: &Self, subgraph: &Self ) -> bool;
    fn is_improper_subgraph( graph: &Self, subgraph: &Self ) -> bool;
    fn is_spanning_subgraph( graph: &Self, subgraph: &Self ) -> bool;
    fn are_adjacent_nodes( graph: &Self, node_1: I, node_2: I ) -> bool;
    fn are_adjacent_edges( graph: &Self, node_1: I, node_2: I, node_3: I ) -> bool;
    fn order( graph: &Self ) -> usize;
    fn size( graph: &Self ) -> usize;
}