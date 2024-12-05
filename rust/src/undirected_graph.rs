// Copyright 2024 Bewusstsein Labs

//: Standard
use std::fmt::Display;

use crate::graph::{
    Error,
    Graph,
    GraphAccess,
    GraphType,
    GraphTraits,
    traverser::{
        Traverser,
        TraverserAccess,
        TraverserTraits,
        Traversable
    }
};

pub struct Undirected();
impl GraphType for Undirected {}
pub type UnGraph<I, N, E> = Graph<Undirected, I, N, E>;
pub type UnTraverser<'a, I, N, E> = Traverser<'a, I, N, E, UnGraph<I, N, E>>;

impl<'a, I, N, E> UnGraph<I, N, E>
where
    I: 'a + Clone + Ord + Display,
    N: 'a + Clone + PartialEq + Display,
    E: 'a + Clone + PartialEq + Display
{
    pub fn generate_dot_to_file( &self, file_name: String ) {
        let mut dot = String::new();
        dot.push_str( "digraph G {\n" );
        for ( node1, node1_data ) in self.nodes().iter() {
            dot.push_str( &format!( " {} [label=\"{}\"];\n", node1, node1_data.data() ) );
            for ( node2, data ) in node1_data.adjacencies().iter() {
                dot.push_str( &format!( " {} -> {} [label=\"{}\" dir=none];\n", node1, node2, data ) );
            }
        }
        dot.push_str( "}\n" );
        std::fs::write( file_name, dot ).unwrap();
    }
}

impl<'a, I, N, E> GraphTraits<'a, I, N, E> for UnGraph<I, N, E>
where
I: 'a + Clone + Ord,
N: 'a + Clone + PartialEq,
E: 'a + Clone + PartialEq
{
    fn add_edge( &mut self, id1: I, id2: I, data: E ) -> Result<(), Error> {
        self.data_mut().add_edge( id1.clone(), id2.clone(), data.clone() )?;
        self.data_mut().add_edge( id2, id1, data )?;
        Ok( () )
    }
}

impl<'a, I, N, E> TraverserTraits<'a, Undirected, I, N, E, UnGraph<I, N, E>> for UnTraverser<'a, I, N, E>
where
    I: 'a + Clone + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    Self: TraverserAccess<'a, Undirected, I, N, E, UnGraph<I, N, E>>
{}

impl<'a, I, N, E> Traversable<'a, Undirected, I, N, E> for UnGraph<I, N, E>
where
    I: 'a + Clone + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq
{}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{
            Graph,
            GraphTraits,
            traverser::{
                TraverserTraits,
                Traversable
            }
        },
        undirected_graph::UnGraph
    };

    #[test]
    fn test_create_graph() {
        let _ = UnGraph::<usize, (), ()>::new();
    }

    #[test]
    fn test_add_node() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        assert!( graph.add_node( 1, () ).is_ok() );
        assert!( graph.contains_node( 1 ) );
    }

    #[test]
    fn test_get_node() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        assert!( graph.get_node( 1 ).is_some() );
        assert!( graph.get_node( 4 ).is_none() );
    }

    #[test]
    fn test_get_node_mut() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        assert!( graph.get_node_mut( 1 ).is_some() );
        assert!( graph.get_node_mut( 4 ).is_none() );
    }

    #[test]
    fn test_contains_node() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        assert!( graph.contains_node( 1 ) );
        assert!( !graph.contains_node( 4 ) );
    }

    #[test]
    fn test_remove_node() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        assert!( graph.remove_node( 1 ).is_ok() );
        assert!( !graph.contains_node( 1 ) );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 3, () ).unwrap();
        assert!( graph.add_edge( 1, 3, () ).is_ok() );
        assert!( graph.contains_edge( 1, 3 ) );
    }

    #[test]
    fn test_get_edge() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        assert!( graph.get_edge( 1, 2 ).is_some() );
        assert!( graph.get_edge( 1, 3 ).is_none() );
    }

    #[test]
    fn test_contains_edge() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        assert!( graph.contains_edge( 1, 2 ) );
        assert!( !graph.contains_edge( 1, 3 ) );
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        assert!( graph.remove_edge( 1, 2 ).is_ok() );
        assert!( !graph.contains_edge( 1, 2 ) );
    }

    #[test]
    fn test_bfs() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.traverser().bfs( 1 );
    }

    #[test]
    fn test_dfs() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.traverser().dfs( 1 );
    }

    #[test]
    fn test_is_complete() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        assert!( graph.is_complete() );
    }

    #[test]
    fn test_is_empty() {
        let graph = UnGraph::<usize, (), ()>::new();
        assert!( graph.is_empty() );
    }

    #[test]
    fn test_is_trivial() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        assert!( graph.is_trivial() );
    }

    #[test]
    fn test_is_null() {
        let graph = UnGraph::<usize, (), ()>::new();
        assert!( graph.is_null() );
    }

    #[test]
    fn test_order() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_node( 3, () ).unwrap();
        assert_eq!( graph.order(), 3 );
    }

    #[test]
    fn test_size() {
        let mut graph = UnGraph::<usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        assert_eq!( graph.size(), 1 );
    }
}
