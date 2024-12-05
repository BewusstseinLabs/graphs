// Copyright 2024 Bewusstsein Labs

use std::{
    cmp::{ Ord, PartialEq },
    collections::{ BTreeSet, BinaryHeap, HashMap, VecDeque },
    marker::PhantomData
};

use crate::graph::{
    GraphType,
    GraphTraits
};

pub(crate) trait TraverserAccess<'a, T, I, N, E, G>
where
    T: GraphType,
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    fn graph( &'a self ) -> &'a G;
    fn graph_mut( &'a mut self ) -> &'a mut G;
}

pub trait TraverserTraits<'a, T, I, N, E, G>
where
    T: 'static + GraphType,
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: 'a + GraphTraits<'a, I, N, E>,
    Self: TraverserAccess<'a, T, I, N, E, G>,
{
    fn bfs_step( &'a self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        while let Some(current_id) = queue.pop_front() {
            if visited.insert( current_id.clone() ) {
                if let Some( current_node ) = self.graph().data().get( &current_id ) {
                    for next_id in current_node.adjacencies().keys() {
                        if !visited.contains( next_id ) {
                            queue.push_back( next_id.clone() );
                        }
                    }
                }
                return Some( current_id );
            }
        }
        None
    }

    fn dfs_step( &'a self, stack: &mut Vec<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        while let Some( current_id ) = stack.pop() {
            if visited.insert( current_id.clone() ) {
                if let Some( current_node ) = self.graph().data().get( &current_id ) {
                    for next_id in current_node.adjacencies().keys() {
                        if !visited.contains( next_id ) {
                            stack.push( next_id.clone() );
                        }
                    }
                }
                return Some( current_id );
            }
        }
        None
    }

    fn bfs( &'a self, start: I ) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back( start.clone() );
        while !queue.is_empty() {
            let _ = self.bfs_step( &mut queue, &mut visited );
        }
    }

    fn dfs( &'a self, start: I ) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push( start.clone() );
        while !stack.is_empty() {
            let _ = self.dfs_step( &mut stack, &mut visited );
        }
    }

    fn dijkstra( &'a self, start: I, end: I ) -> Option<Vec<I>>
    where
        I: std::hash::Hash
    {
        let mut dist: HashMap<I, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut predecessors: HashMap<I, I> = HashMap::new();

        // Initialize distances
        dist.insert( start.clone(), 0 );
        heap.push( ( 0, start.clone() ) );

        while let Some( ( cost, position ) ) = heap.pop() {
            if position == end {
                // Reconstruct the path from end to start
                let mut path = VecDeque::new();
                let mut current = end.clone();
                while let Some( predecessor ) = predecessors.get( &current ) {
                    path.push_front( current.clone() );
                    current = predecessor.clone();
                }
                path.push_front( start );
                return Some( path.into_iter().collect() );
            }

            if cost > *dist.get( &position ).unwrap_or( &usize::MAX ) {
                continue;
            }

            if let Some( current_node) = self.graph().data().get( &position ) {
                for next_id in current_node.adjacencies().keys() {
                    let next_cost = cost + 1;
                    if next_cost < *dist.get( next_id ).unwrap_or( &usize::MAX ) {
                        dist.insert( next_id.clone(), next_cost );
                        predecessors.insert( next_id.clone(), position.clone() );
                        heap.push( ( next_cost, next_id.clone() ) );
                    }
                }
            }
        }

        None // Return None if no path is found
    }
}

pub trait Traversable<'a, T, I, N, E>
where
    T: GraphType,
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    Self: Sized,
    Self: GraphTraits<'a, I, N, E>
{
    fn traverser( &'a self ) -> Traverser<'a, I, N, E, Self> {
        Traverser::new( self )
    }

    fn traverser_mut( &'a mut self ) -> TraverserMut<'a, I, N, E, Self> {
        TraverserMut::new( self )
    }
}

#[derive( Debug, Clone )]
pub struct Traverser<'a, I, N, E, G>
where
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    graph: &'a G,
    i: PhantomData<I>,
    n: PhantomData<N>,
    e: PhantomData<E>
}

#[derive( Debug )]
pub struct TraverserMut<'a, I, N, E, G>
where
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    graph: &'a mut G,
    i: PhantomData<I>,
    n: PhantomData<N>,
    e: PhantomData<E>
}

impl<'a, T, I, N, E, G> TraverserAccess<'a, T, I, N, E, G> for Traverser<'a, I, N, E, G>
where
    T: GraphType,
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    fn graph( &'a self ) -> &'a G {
        self.graph
    }

    fn graph_mut( &'a mut self ) -> &'a mut G {
        panic!( "Cannot access mutable graph from immutable traverser" )
    }
}

impl<'a, T, I, N, E, G> TraverserAccess<'a, T, I, N, E, G> for TraverserMut<'a, I, N, E, G>
where
    T: GraphType,
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    fn graph( &'a self ) -> &'a G {
        self.graph
    }

    fn graph_mut( &'a mut self ) -> &'a mut G {
        self.graph
    }
}

impl<'a, I, N, E, G> Traverser<'a, I, N, E, G>
where
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    pub fn new( graph: &'a G ) -> Self {
        Self {
            graph,
            i: PhantomData::<I>,
            n: PhantomData::<N>,
            e: PhantomData::<E>
        }
    }
}

impl<'a, I, N, E, G> TraverserMut<'a, I, N, E, G>
where
    I: 'a + Clone + Ord,
    N: 'a + PartialEq,
    E: 'a + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    pub fn new( graph: &'a mut G ) -> Self {
        Self {
            graph,
            i: PhantomData::<I>,
            n: PhantomData::<N>,
            e: PhantomData::<E>
        }
    }
}

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
        directed_graph::Directed
    };

    #[test]
    fn test_dfs() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_node( 3, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        graph.add_edge( 2, 3, () ).unwrap();
        graph.add_edge( 3, 1, () ).unwrap();
        graph.traverser().dfs( 1 );
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::<Directed, usize, (), ()>::new();
        graph.add_node( 1, () ).unwrap();
        graph.add_node( 2, () ).unwrap();
        graph.add_node( 3, () ).unwrap();
        graph.add_edge( 1, 2, () ).unwrap();
        graph.add_edge( 2, 3, () ).unwrap();
        graph.add_edge( 3, 1, () ).unwrap();
        graph.traverser().bfs( 1 );
    }

    #[test]
    fn test_dijkstra() {
        let mut graph = Graph::<Directed, &'static str, &'static str, &'static str>::new();

        graph.add_node( "a", "a" ).unwrap();
        graph.add_node( "b_a", "b_a" ).unwrap();
        graph.add_node( "b_b", "b_b" ).unwrap();
        graph.add_node( "b_c", "b_c" ).unwrap();
        graph.add_node( "c_a", "c_a" ).unwrap();
        graph.add_node( "c_b", "c_b" ).unwrap();
        graph.add_node( "d_a", "d_a" ).unwrap();
        graph.add_node( "d_b", "d_b" ).unwrap();
        graph.add_node( "d_c", "d_c" ).unwrap();
        graph.add_node( "e", "e" ).unwrap();

        graph.add_edge( "a", "b_a", "a -> b_a" ).unwrap();
        graph.add_edge( "b_a", "c_b", "b_a -> c_b" ).unwrap();

        graph.add_edge( "b_c", "c_b", "b_c -> c_b" ).unwrap();
        graph.add_edge( "c_a", "d_b", "c_a -> d_b" ).unwrap();
        graph.add_edge( "d_a", "e", "d_a -> e" ).unwrap();

        graph.add_edge( "a", "b_b", "a -> b_b" ).unwrap();
        graph.add_edge( "b_b", "c_a", "b_b -> c_a" ).unwrap();
        graph.add_edge( "c_a", "d_c", "c_a -> d_c" ).unwrap();
        graph.add_edge( "d_c", "e", "d_c -> e" ).unwrap();

        graph.generate_dot_to_file( "test.dot".into() );

        let start = std::time::Instant::now();
        let path = graph.traverser().dijkstra( "a", "e" );
        let duration = start.elapsed();
        println!("Time taken to traverse the graph: {:?}", duration);

        assert!( path.is_some() );
        let path = path.unwrap();
        println!( "path: {:?}", path );
    }
}
