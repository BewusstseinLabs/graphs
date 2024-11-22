// Copyright 2024 Bewusstsein Labs

use std::{
    cmp::{ Ord, PartialEq },
    collections::{ BTreeSet, VecDeque },
    marker::PhantomData
};

use crate::graph::{
    GraphType,
    GraphTraits
};

pub(crate) trait TraverserAccess<'a, T, I, N, E, G>
where
    T: GraphType,
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    fn graph( &'a self ) -> &'a G;
}

pub trait TraverserTraits<'a, T, I, N, E, G>
where
    T: 'static + GraphType,
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
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


}

pub trait Traversable<'a, T, I, N, E>
where
    T: GraphType,
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    Self: Sized,
    Self: GraphTraits<'a, I, N, E>
{
    fn traverser( &'a self ) -> Traverser<'a, I, N, E, Self> {
        Traverser::new( self )
    }
}

pub struct Traverser<'a, I, N, E, G>
where
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    graph: &'a G,
    i: PhantomData<I>,
    n: PhantomData<N>,
    e: PhantomData<E>
}

impl<'a, T, I, N, E, G> TraverserAccess<'a, T, I, N, E, G> for Traverser<'a, I, N, E, G>
where
    T: GraphType,
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
    G: GraphTraits<'a, I, N, E>
{
    fn graph( &'a self ) -> &'a G {
        &self.graph
    }
}

impl<'a, I, N, E, G> Traverser<'a, I, N, E, G>
where
    I: 'a + Clone + PartialEq + Ord,
    N: 'a + Clone + PartialEq,
    E: 'a + Clone + PartialEq,
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
