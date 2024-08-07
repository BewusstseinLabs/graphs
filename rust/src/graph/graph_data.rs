use crate::graph::error::Error;

//: Standard
use std::{
    collections::{ BTreeSet, BTreeMap, VecDeque },
    ops::{
        Add, AddAssign,
        Sub, SubAssign,
    },
    cmp::{
        Eq, PartialEq,
        Ord
    },
    fmt::Display
};

pub trait GraphDataTraits<I, N, E>
where
    I: Clone + Ord + PartialEq,
    N: Clone + PartialEq,
    E: Clone + PartialEq,
{
    fn new() -> Self;
    fn add_node(&mut self, node: I, data: N) -> Result<(), Error>;
    fn get_node(&self, node: I) -> Option<&N>;
    fn get_node_mut(&mut self, node: I) -> Option<&mut N>;
    fn get_nodes(&self) -> &BTreeMap<I, NodeData<I, N, E>>;
    fn get_nodes_mut(&mut self) -> &mut BTreeMap<I, NodeData<I, N, E>>;
    fn contains_node(&self, node: I) -> bool;
    fn remove_node(&mut self, node: I) -> Result<N, Error>;
    fn delete_node(&mut self, node: I) -> Result<(), Error>;
    fn add_undirected_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error>;
    fn add_directed_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error>;
    fn get_edge(&self, node1: I, node2: I) -> Option<&E>;
    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut E>;
    fn get_edges(&self, node: I) -> Option<&AdjacencyData<I, E>>;
    fn get_edges_mut(&mut self, node: I) -> Option<&mut AdjacencyData<I, E>>;
    fn contains_edge(&self, node1: I, node2: I) -> bool;
    fn remove_edge(&mut self, node1: I, node2: I) -> Result<E, Error>;
    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error>;
    fn clear( &mut self );
    fn clear_edges( &mut self );
    fn bfs_step(&mut self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I>) -> Option<I>;
    fn dfs_step(&mut self, stack: &mut Vec<I>, visited: &mut BTreeSet<I>) -> Option<I>;
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

pub type AdjacencyData<I, E> = BTreeMap<I, E>;
pub type NodeData<I, N, E> = ( AdjacencyData<I, E>, N );
pub struct GraphData<I, N, E> {
    data: BTreeMap<I, NodeData<I, N, E>>
}

impl<I, N, E> GraphDataTraits<I, N, E> for GraphData<I, N, E>
where
    I: Clone + Ord + PartialEq,
    N: Clone + PartialEq,
    E: Clone + PartialEq,
{
    fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    fn add_node(&mut self, id: I, data: N) -> Result<(), Error> {
        if self.data.contains_key(&id) {
            return Err(Error::NodeAlreadyExists);
        }
        self.data.insert(id, (BTreeMap::new(), data));
        Ok(())
    }

    fn get_node(&self, id: I) -> Option<&N> {
        self.data.get(&id).map(|node| &node.1)
    }

    fn get_node_mut(&mut self, id: I) -> Option<&mut N> {
        self.data.get_mut(&id).map(|node| &mut node.1)
    }

    fn get_nodes(&self) -> &BTreeMap<I, NodeData<I, N, E>> {
        &self.data
    }

    fn get_nodes_mut(&mut self) -> &mut BTreeMap<I, NodeData<I, N, E>> {
        &mut self.data
    }

    fn contains_node(&self, id: I) -> bool {
        self.data.contains_key(&id)
    }

    fn remove_node(&mut self, id: I) -> Result<N, Error> {
        if let Some(node) = self.data.remove(&id) {
            Ok(node.1)
        } else {
            Err(Error::NodeNotFound)
        }
    }

    fn delete_node(&mut self, id: I) -> Result<(), Error> {
        if self.data.remove(&id).is_none() {
            return Err(Error::NodeNotFound);
        }
        Ok(())
    }

    fn add_undirected_edge(&mut self, id1: I, id2: I, data: E) -> Result<(), Error> {
        if let Some(node1) = self.data.get_mut(&id1) {
            if node1.0.contains_key(&id2) {
                return Err(Error::EdgeAlreadyExists);
            }
            node1.0.insert(id2.clone(), data.clone());
        } else {
            return Err(Error::NodeNotFound);
        }
        if let Some(node2) = self.data.get_mut(&id2) {
            if node2.0.contains_key(&id1) {
                return Err(Error::EdgeAlreadyExists);
            }
            node2.0.insert(id1, data);
        } else {
            return Err(Error::NodeNotFound);
        }
        Ok(())
    }

    fn add_directed_edge(&mut self, id1: I, id2: I, data: E) -> Result<(), Error> {
        if let Some(node1) = self.data.get_mut(&id1) {
            if node1.0.contains_key(&id2) {
                return Err(Error::EdgeAlreadyExists);
            }
            node1.0.insert(id2.clone(), data.clone());
            return Ok(());
        }
        Err(Error::NodeNotFound)
    }

    fn get_edge(&self, id1: I, id2: I) -> Option<&E> {
        self.data.get(&id1).and_then(|node| node.0.get(&id2))
    }

    fn get_edge_mut(&mut self, id1: I, id2: I) -> Option<&mut E> {
        self.data.get_mut(&id1).and_then(|node| node.0.get_mut(&id2))
    }

    fn get_edges(&self, id: I) -> Option<&AdjacencyData<I, E>> {
        self.data.get(&id).map(|node| &node.0)
    }

    fn get_edges_mut(&mut self, id: I) -> Option<&mut AdjacencyData<I, E>> {
        self.data.get_mut(&id).map(|node| &mut node.0)
    }

    fn contains_edge(&self, id1: I, id2: I) -> bool {
        self.data.get(&id1).map_or(false, |node| node.0.contains_key(&id2))
    }

    fn remove_edge(&mut self, id1: I, id2: I) -> Result<E, Error> {
        if let Some(node) = self.data.get_mut(&id1) {
            if let Some(data) = node.0.remove(&id2) {
                Ok(data)
            } else {
                Err(Error::EdgeNotFound)
            }
        } else {
            Err(Error::NodeNotFound)
        }
    }

    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error> {
        if let Some(node) = self.data.get_mut(&node1) {
            if node.0.remove(&node2).is_none() {
                return Err(Error::EdgeNotFound);
            }
            Ok(())
        } else {
            Err(Error::NodeNotFound)
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn clear_edges(&mut self) {
        for (_, node) in self.data.iter_mut() {
            node.0.clear();
        }
    }

    fn bfs_step(&mut self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I>) -> Option<I> {
        let mut current: Option<I> = None;
        if let Some( current_id ) = queue.pop_front() {
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                current = Some( current_id.clone() );
                if let Some( ( current_adjacencies, _ ) ) = self.data.get( &current_id ) {
                    for ( next_id, _ ) in current_adjacencies {
                        if !visited.contains( next_id ) {
                            queue.push_back( next_id.clone() );
                        }
                    }
                }
            }
        }
        current
    }

    fn dfs_step(&mut self, stack: &mut Vec<I>, visited: &mut BTreeSet<I>) -> Option<I> {
        let mut current: Option<I> = None;
        if let Some( current_id ) = stack.pop() {
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                current = Some( current_id.clone() );
                if let Some( ( current_adjacencies, _ ) ) = self.data.get_mut( &current_id ) {
                    for ( next_id, _ ) in current_adjacencies.iter_mut() {
                        if !visited.contains( next_id ) {
                            stack.push( next_id.clone() );
                        }
                    }
                }
            }
        }
        current
    }

    fn is_complete( graph: &Self ) -> bool {
        for ( node, neighbors ) in &graph.data {
            if neighbors.0.len() != graph.data.len() - 1 {
                return false;
            }
            for neighbor in neighbors.0.keys() {
                if !graph.data.get( neighbor ).unwrap().0.contains_key( node ) {
                    return false;
                }
            }
        }
        true
    }

    fn is_empty( graph: &Self ) -> bool {
        if graph.data.is_empty() {
            return true;
        }
        for ( _, neighbors ) in &graph.data {
            if !neighbors.0.is_empty() {
                return false;
            }
        }
        true
    }

    fn is_trivial( graph: &Self ) -> bool {
        if graph.data.len() == 1 {
            for ( _, neighbors ) in &graph.data {
                if neighbors.0.is_empty() {
                    return true;
                }
            }
        }
        false
    }

    fn is_null( graph: &Self ) -> bool {
        if graph.data.is_empty() {
            return true;
        }
        false
    }

    fn is_child_node( graph: &Self, node_1: I ) -> bool {
        if graph.data.contains_key( &node_1 ) {
            return true;
        }
        false
    }

    fn is_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        for ( node, neighbors ) in &subgraph.data {
            if !graph.data.contains_key( node ) {
                return false;
            }
            else if !graph.data.get( node ).unwrap().0.keys().all( |key| neighbors.0.contains_key( key ) ) {
                return false;
            }
        }
        true
    }

    fn is_proper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        if graph.data != subgraph.data {
            if Self::is_subgraph( graph, subgraph ) {
                return true;
            }
        }
        false
    }

    fn is_improper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        if graph.data != subgraph.data {
            return false;
        }
        true
    }

    fn is_spanning_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        if graph.data.len() != subgraph.data.len() {
            return false;
        }
        else if Self::is_subgraph( graph, subgraph ) {
            return true;
        }
        false
    }

    fn are_adjacent_nodes( graph: &Self, node_1: I, node_2: I ) -> bool {
        if !Self::is_child_node( graph, node_1.clone() ) {
            return false;
        }
        if !Self::is_child_node( graph, node_2.clone() ) {
            return false;
        }
        if !graph.data.get( &node_1 ).unwrap().0.contains_key( &node_2 ) {
            return false;
        }
        true
    }

    fn are_adjacent_edges( graph: &Self, node_1: I, node_2: I, node_3: I ) -> bool {
        if !Self::are_adjacent_nodes( graph, node_1, node_2.clone() ) {
            return false;
        }
        if !Self::are_adjacent_nodes( graph, node_2, node_3 ) {
            return false;
        }
        true
    }

    fn order( graph: &Self ) -> usize {
        graph.data.len()
    }

    fn size( graph: &Self ) -> usize {
        let mut size = 0;
        for ( _, neighbors ) in &graph.data {
            size += neighbors.0.len();
        }
        size / 2
    }
}