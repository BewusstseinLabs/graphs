// Copyright 2024 Bewusstsein Labs

#![warn(private_bounds)]

//: Standard
use std::{
    cmp::{ Eq, Ord, PartialEq },
    collections::{ BTreeMap, BTreeSet, VecDeque },
    marker::PhantomData,
    ops::{ Deref, DerefMut }
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: Node Not Found")]
    NodeNotFound,
    #[error("Graph Error: Node Already Exists")]
    NodeAlreadyExists,
    #[error("Graph Error: Edge Not Found")]
    EdgeNotFound,
    #[error("Graph Error: Edge Already Exists")]
    EdgeAlreadyExists,
}

pub type AdjacencyData<I, E> = BTreeMap<I, E>;

#[derive( PartialEq, Eq )]
pub struct NodeData<I, N, E> {
    pub data: N,
    pub adjacencies: AdjacencyData<I, E>
}

impl<I, N, E> NodeData<I, N, E> {
    pub fn new( data: N ) -> Self {
        Self {
            data,
            adjacencies: BTreeMap::new()
        }
    }
}

#[derive( PartialEq, Eq )]
pub struct GraphData<I, N, E>( BTreeMap<I, NodeData<I, N, E>> );

impl<I, N, E> Deref for GraphData<I, N, E> {
    type Target = BTreeMap<I, NodeData<I, N, E>>;
    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<I, N, E> DerefMut for GraphData<I, N, E> {
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I, N, E> GraphData<I, N, E>
where
    I: Clone + PartialEq + Ord,
    N: Clone + PartialEq,
    E: Clone + PartialEq,
    Self: Deref<Target=BTreeMap<I, NodeData<I, N, E>>> + DerefMut<Target=BTreeMap<I, NodeData<I, N, E>>>
{
    pub fn add_node( &mut self, id: I, data: N ) -> Result<(), Error> {
        if self.contains_key( &id ) {
            return Err( Error::NodeAlreadyExists );
        }
        self.insert( id, NodeData::new( data ) );
        Ok( () )
    }

    pub fn get_node( &self, id: I ) -> Option<&N> {
        self.get( &id ).map( |node| &node.data )
    }

    pub fn get_node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.get_mut( &id ).map( |node| &mut node.data )
    }

    pub fn get_nodes( &self ) -> &BTreeMap<I, NodeData<I, N, E>> {
        self
    }

    pub fn get_nodes_mut( &mut self ) -> &mut BTreeMap<I, NodeData<I, N, E>> {
        self
    }

    pub fn contains_node( &self, id: I ) -> bool {
        self.contains_key( &id )
    }

    pub fn remove_node( &mut self, id: I ) -> Result<N, Error> {
        self.remove(&id).map(|node| node.data).ok_or(Error::NodeNotFound)
    }

    pub fn delete_node( &mut self, id: I ) -> Result<(), Error> {
        self.remove(&id).ok_or(Error::NodeNotFound)?;
        Ok(())
    }

    pub fn add_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error> {
        match self.get_mut(&node1) {
            Some(node1) if node1.adjacencies.contains_key(&node2) => Err(Error::EdgeAlreadyExists),
            Some(node1) => {
                node1.adjacencies.insert(node2, data);
                Ok(())
            }
            None => Err(Error::NodeNotFound),
        }
    }

    pub fn get_edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.get( &id1 ).and_then( |node| node.adjacencies.get( &id2 ) )
    }

    pub fn get_edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.get_mut( &id1 ).and_then( |node| node.adjacencies.get_mut( &id2 ) )
    }

    pub fn get_edges( &self, id: I ) -> Option<&AdjacencyData<I, E>> {
        self.get( &id ).map( |node| &node.adjacencies )
    }

    pub fn get_edges_mut( &mut self, id: I ) -> Option<&mut AdjacencyData<I, E>> {
        self.get_mut( &id ).map( |node| &mut node.adjacencies )
    }

    pub fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.get( &id1 ).map_or( false, |node| node.adjacencies.contains_key( &id2 ) )
    }

    pub fn remove_edge( &mut self, id1: I, id2: I ) -> Result<E, Error> {
        self.get_mut(&id1)
            .ok_or(Error::NodeNotFound)
            .and_then(|node| node.adjacencies.remove(&id2).ok_or(Error::EdgeNotFound))
    }

    pub fn delete_edge( &mut self, node1: I, node2: I ) -> Result<(), Error> {
        match self.get_mut(&node1) {
            Some(node) => {
                if node.adjacencies.remove(&node2).is_some() {
                    Ok(())
                } else {
                    Err(Error::EdgeNotFound)
                }
            }
            None => Err(Error::NodeNotFound),
        }
    }

    pub fn clear_edges( &mut self ) {
        for node in self.values_mut() {
            node.adjacencies.clear();
        }
    }
}

trait GraphAccess<'a, I, N, E>
where
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq
{
    fn data( &'a self ) -> &'a GraphData<I, N, E>;
    fn data_mut( &'a mut self ) -> &'a mut GraphData<I, N, E>;
}

pub trait GraphTraits<'a, I, N, E>
where
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    Self: GraphAccess<'a, I, N, E>
{
    fn add_node( &'a mut self, id: I, data: N ) -> Result<(), Error> {
        self.data_mut().add_node( id, data )
    }

    fn get_node( &'a self, id: I ) -> Option<&'a N> {
        self.data().get_node( id )
    }

    fn get_node_mut( &'a mut self, id: I ) -> Option<&'a mut N> {
        self.data_mut().get_node_mut( id )
    }

    fn get_nodes( &'a self ) -> &'a BTreeMap<I, NodeData<I, N, E>> {
        self.data().get_nodes()
    }

    fn get_nodes_mut( &'a mut self ) -> &'a mut BTreeMap<I, NodeData<I, N, E>> {
        self.data_mut().get_nodes_mut()
    }

    fn contains_node( &'a self, id: I ) -> bool {
        self.data().contains_node( id )
    }

    fn remove_node( &'a mut self, id: I ) -> Result<N, Error> {
        self.data_mut().remove_node( id )
    }

    fn delete_node( &'a mut self, id: I ) -> Result<(), Error> {
        self.data_mut().delete_node( id )
    }

    fn add_edge(&'a mut self, node1: I, node2: I, data: E) -> Result<(), Error> {
        self.data_mut().add_edge( node1, node2, data )
    }

    fn get_edge( &'a self, id1: I, id2: I ) -> Option<&'a E> {
        self.data().get_edge( id1, id2 )
    }

    fn get_edge_mut( &'a mut self, id1: I, id2: I ) -> Option<&'a mut E> {
        self.data_mut().get_edge_mut( id1, id2 )
    }

    fn get_edges( &'a self, id: I ) -> Option<&'a AdjacencyData<I, E>> {
        self.data().get_edges( id )
    }

    fn get_edges_mut( &'a mut self, id: I ) -> Option<&'a mut AdjacencyData<I, E>> {
        self.data_mut().get_edges_mut( id )
    }

    fn contains_edge( &'a self, id1: I, id2: I ) -> bool {
        self.data().contains_edge( id1, id2 )
    }

    fn remove_edge( &'a mut self, id1: I, id2: I ) -> Result<E, Error> {
        self.data_mut().remove_edge( id1, id2 )
    }

    fn delete_edge( &'a mut self, node1: I, node2: I ) -> Result<(), Error> {
        self.data_mut().delete_edge( node1, node2 )
    }

    fn clear( &'a mut self ) {
        self.data_mut().clear();
    }

    fn clear_edges( &'a mut self ) {
        self.data_mut().clear_edges();
    }

    fn bfs_step( &'a self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        let mut current: Option<I> = None;
        if let Some( current_id ) = queue.pop_front() {
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                current = Some( current_id.clone() );
                if let Some( current_node ) = self.data().get( &current_id ) {
                    for next_id in current_node.adjacencies.keys() {
                        if !visited.contains( next_id ) {
                            queue.push_back( next_id.clone() );
                        }
                    }
                }
            }
        }
        current
    }

    fn dfs_step( &'a self, stack: &mut Vec<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        let mut current: Option<I> = None;
        if let Some( current_id ) = stack.pop() {
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                current = Some( current_id.clone() );
                if let Some( current_node ) = self.data().get( &current_id ) {
                    for next_id in current_node.adjacencies.keys() {
                        if !visited.contains( next_id ) {
                            stack.push( next_id.clone() );
                        }
                    }
                }
            }
        }
        current
    }

    fn bfs(&'a self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while !queue.is_empty() {
            self.bfs_step(&mut queue, &mut visited);
        }
    }

    fn dfs(&'a self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while !stack.is_empty() {
            self.dfs_step(&mut stack, &mut visited);
        }
    }

    fn bfs_mut(&'a mut self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while !queue.is_empty() {
            self.bfs_step(&mut queue, &mut visited);
        }
    }

    fn dfs_mut(&'a mut self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while !stack.is_empty() {
            self.dfs_step(&mut stack, &mut visited);
        }
    }

    fn is_complete( &'a self ) -> bool {
        for ( node, neighbors ) in self.data().iter() {
            if neighbors.adjacencies.len() != self.data().len() - 1 {
                return false;
            }
            for neighbor in neighbors.adjacencies.keys() {
                if !self.data().get(neighbor).map_or(false, |n| n.adjacencies.contains_key(node)) {
                    return false;
                }
            }
        }
        true
    }

    fn is_empty( &'a self ) -> bool {
        self.data().values().all(|neighbors| neighbors.adjacencies.is_empty())
    }

    fn is_trivial( &'a self ) -> bool {
        self.data().len() == 1 && self.data().values().next().map_or(false, |neighbors| neighbors.adjacencies.is_empty())
    }

    fn is_null( &'a self ) -> bool {
        self.data().is_empty()
    }

    fn is_child_node( &'a self, node_1: I ) -> bool {
        self.data().contains_key( &node_1 )
    }

    fn is_subgraph(&'a self, subgraph: &'a Self) -> bool {
        subgraph.data().iter().all(|(node, neighbors)| {
            self.data().get(node).map_or(false, |graph_node| {
                neighbors.adjacencies.keys().all(|key| graph_node.adjacencies.contains_key(key))
            })
        })
    }

    fn is_proper_subgraph( &'a self, subgraph: &'a Self ) -> bool {
        self.data() != subgraph.data() && self.is_subgraph(subgraph)
    }

    fn is_improper_subgraph( &'a self, subgraph: &'a Self ) -> bool {
        self.data() == subgraph.data()
    }

    fn is_spanning_subgraph( &'a self, subgraph: &'a Self ) -> bool {
        self.data().len() == subgraph.data().len() && self.is_subgraph(subgraph)
    }

    fn are_adjacent_nodes( &'a self, node_1: I, node_2: I ) -> bool {
        self.is_child_node(node_1.clone())
            && self.is_child_node(node_2.clone())
            && self.data().get(&node_1).unwrap().adjacencies.contains_key(&node_2)
    }

    fn are_adjacent_edges( &'a self, node_1: I, node_2: I, node_3: I ) -> bool {
        self.are_adjacent_nodes(node_1, node_2.clone())
            && self.are_adjacent_nodes(node_2, node_3)
    }

    fn order( &'a self ) -> usize {
        self.data().len()
    }

    fn size( &'a self ) -> usize {
        self.data().values().map(|neighbors| neighbors.adjacencies.len()).sum::<usize>() / 2
    }
}

pub trait GraphType {}

pub struct Graph<T, I, N, E>
where
    T: GraphType,
    I: Clone + PartialEq + Ord,
    N: Clone + PartialEq,
    E: Clone + PartialEq
{
    pub(crate) data: GraphData<I, N, E>,
    t: PhantomData<T>
}

impl<T, I, N, E> Graph<T, I, N, E>
where
    T: GraphType,
    I: Clone + PartialEq + Ord,
    N: Clone + PartialEq,
    E: Clone + PartialEq
{
    pub fn new() -> Self {
        Self {
            data: GraphData( BTreeMap::new() ),
            t: PhantomData
        }
    }
}

impl<T, I, N, E> Default for Graph<T, I, N, E>
where
    T: GraphType,
    I: Clone + PartialEq + Ord,
    N: Clone + PartialEq,
    E: Clone + PartialEq
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, I, N, E> GraphAccess<'a, I, N, E> for Graph<T, I, N, E>
where
    T: GraphType,
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq
{
    #[inline]
    fn data( &'a self ) -> &'a GraphData<I, N, E> {
        &self.data
    }

    #[inline]
    fn data_mut( &'a mut self ) -> &'a mut GraphData<I, N, E> {
        &mut self.data
    }
}
