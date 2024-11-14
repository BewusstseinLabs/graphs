// Copyright 2024 Bewusstsein Labs

//: Standard
use std::fmt::Display;

use crate::graph::{
    Graph,
    GraphType,
    GraphTraits
};

pub struct Directed();
impl GraphType for Directed {}

impl<'a, I, N, E> Graph<Directed, I, N, E>
where
    I: 'a + Clone + PartialEq + Display + Ord,
    N: 'a + Clone + PartialEq + Display,
    E: 'a + Clone + PartialEq + Display
{
    pub fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node1, node1_data) in self.get_nodes().iter() {
            dot.push_str(&format!(" {} [label=\"{}\"];\n", node1, node1_data.data));
            for (node2, node2_data) in node1_data.adjacencies.iter() {
                dot.push_str(&format!(" {} -> {} [label=\"{}\"];\n", node1, node2, node2_data));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}

impl<'a, I, N, E> GraphTraits<'a, I, N, E> for Graph<Directed, I, N, E>
where
I: 'a + Clone + PartialEq + Ord,
N: 'a + Clone + PartialEq,
E: 'a + Clone + PartialEq
{
    fn size( &'a self ) -> usize {
        self.data.values().map(|neighbors| neighbors.adjacencies.len()).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{ Graph, GraphTraits },
        directed_graph::Directed
    };

    #[test]
    fn test_create_graph() {
        let _ = Graph::<Directed, usize, (), ()>::new();
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        assert!(graph.add_node(1, ()).is_ok());
        assert!(graph.contains_node(1));
    }

    #[test]
    fn test_get_node() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        assert!(graph.get_node(1).is_some());
        assert!(graph.get_node(4).is_none());
    }

    #[test]
    fn test_get_node_mut() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        assert!(graph.get_node_mut(1).is_some());
        assert!(graph.get_node_mut(4).is_none());
    }

    #[test]
    fn test_contains_node() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        assert!(graph.contains_node(1));
        assert!(!graph.contains_node(4));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        assert!(graph.remove_node(1).is_ok());
        assert!(!graph.contains_node(1));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(3, ()).unwrap();
        assert!(graph.add_edge(1, 3, ()).is_ok());
        assert!(graph.contains_edge(1, 3));
    }

    #[test]
    fn test_get_edge() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_edge(1, 2, ()).unwrap();
        assert!(graph.get_edge(1, 2).is_some());
        assert!(graph.get_edge(1, 3).is_none());
    }

    #[test]
    fn test_contains_edge() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_edge(1, 2, ()).unwrap();
        assert!(graph.contains_edge(1, 2));
        assert!(!graph.contains_edge(1, 3));
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_edge(1, 2, ()).unwrap();
        assert!(graph.remove_edge(1, 2).is_ok());
        assert!(!graph.contains_edge(1, 2));
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.bfs(1);
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.dfs(1);
    }

    #[test]
    fn test_is_complete() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_edge(1, 2, ()).unwrap();
        graph.add_edge(2, 1, ()).unwrap();
        assert!(graph.is_complete());
    }

    #[test]
    fn test_is_empty() {
        let graph = Graph::<Directed, usize, (), ()>::new();
        assert!(graph.is_empty());
    }

    #[test]
    fn test_is_trivial() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        assert!(graph.is_trivial());
    }

    #[test]
    fn test_is_null() {
        let graph = Graph::<Directed, usize, (), ()>::new();
        assert!(graph.is_null());
    }

    #[test]
    fn test_order() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_node(3, ()).unwrap();
        assert_eq!(graph.order(), 3);
    }

    #[test]
    fn test_size() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node(1, ()).unwrap();
        graph.add_node(2, ()).unwrap();
        graph.add_edge(1, 2, ()).unwrap();
        assert_eq!(graph.size(), 1);
    }
}
