use crate::graph::error::Error;

//: Standard
use std::collections::{ BTreeSet, BTreeMap, VecDeque };

pub trait GraphDataTraits<I, N, E>
where
    I: Clone + Ord,
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
    fn add_edge(&mut self, node1: I, node2: I, data: E) -> Result<(), Error>;
    fn get_edge(&self, node1: I, node2: I) -> Option<&E>;
    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut E>;
    fn get_edges(&self, node: I) -> Option<&AdjacencyData<I, E>>;
    fn get_edges_mut(&mut self, node: I) -> Option<&mut AdjacencyData<I, E>>;
    fn contains_edge(&self, node1: I, node2: I) -> bool;
    fn remove_edge(&mut self, node1: I, node2: I) -> Result<E, Error>;
    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error>;
    fn bfs_step(&mut self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I>) -> (Option<I>, Option<I>);
    fn dfs_step(&mut self, stack: &mut Vec<I>, visited: &mut BTreeSet<I>) -> (Option<I>, Option<I>);
}

pub type AdjacencyData<I, E> = BTreeMap<I, E>;
pub type NodeData<I, N, E> = ( AdjacencyData<I, E>, N );
pub struct GraphData<I, N, E> {
    data: BTreeMap<I, NodeData<I, N, E>>
}

impl<I, N, E> GraphDataTraits<I, N, E> for GraphData<I, N, E>
where
    I: Clone + Ord,
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

    fn add_edge(&mut self, id1: I, id2: I, data: E) -> Result<(), Error> {
        if let Some(id) = self.data.get_mut(&id1) {
            if id.0.contains_key(&id2) {
                return Err(Error::EdgeAlreadyExists);
            }
            id.0.insert(id2, data);
            Ok(())
        } else {
            Err(Error::NodeNotFound)
        }
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

    fn bfs_step(&mut self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I>) -> (Option<I>, Option<I>) {
        let mut edge: (Option<I>, Option<I>) = (None, None);
        if let Some( current_id ) = queue.pop_front() {
            edge.0 = Some( current_id.clone() );
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                if let Some( ( current_adjacencies, _ ) ) = self.data.get( &current_id ) {
                    for ( next_id, _ ) in current_adjacencies {
                        if !visited.contains( next_id ) {
                            queue.push_back( next_id.clone() );
                        }
                    }
                }
            }
        } else {
            edge = (None, None);
        }
        edge.1 = queue.front().cloned();
        edge
    }

    fn dfs_step(&mut self, stack: &mut Vec<I>, visited: &mut BTreeSet<I>) -> (Option<I>, Option<I>) {
        let mut edge: (Option<I>, Option<I>) = (None, None);
        if let Some( current_id ) = stack.pop() {
            edge.0 = Some( current_id.clone() );
            if !visited.contains( &current_id ) {
                visited.insert( current_id.clone() );
                if let Some( ( current_adjacencies, _ ) ) = self.data.get_mut( &current_id ) {
                    for ( next_id, _ ) in current_adjacencies.iter_mut() {
                        if !visited.contains( next_id ) {
                            stack.push( next_id.clone() );
                        }
                    }
                }
            }
        } else {
            edge = (None, None);
        }
        edge.1 = stack.last().cloned();
        edge

    }
}