pub mod error;
mod graph_data;
pub mod undirected_graph;
pub mod directed_graph;
pub mod function_graph;
mod test;

use self::error::Error;

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
    fn bfs(&mut self, start: I);
    fn dfs(&mut self, start: I);
}