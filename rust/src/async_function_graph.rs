// Copyright 2024 Bewusstsein Labs

#![warn(clippy::type_complexity)]

pub mod function;
pub mod operation;

use std::{
    hash::Hash,
    collections::{ BTreeSet, VecDeque },
    fmt::Display,
    thread
};

use thiserror::Error;

use crate::{
    graph::{
        Error as GraphError,
        Graph,
        GraphAccess,
        GraphTraits,
        GraphType,
        traverser::{
            Traverser,
            TraverserAccess,
            AsyncTraverserTraits,
            Traversable
        }
    },
    function_graph::variable::{ Variable, Variables },
    async_function_graph::operation::{ AsyncOperation, Error as AsyncOperationError }
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: {0}")]
    GraphError( #[from] GraphError),
    #[error("Operation Error: {0}")]
    OperationError( #[from] AsyncOperationError )
}

#[derive( Debug )]
pub struct AsyncFn ();
impl GraphType for AsyncFn {}
pub type AsyncFnGraph<I, J> = Graph<AsyncFn, I, AsyncOperation<J>, bool>;
pub type FnTraverser<'a, I, J> = Traverser<'a, I, AsyncOperation<J>, bool, Graph<AsyncFn, I, AsyncOperation<J>, bool>>;

impl<'a, I, J> AsyncFnGraph<I, J>
where
    I: 'a + Clone + Ord + Display,
    J: 'static + Clone + Ord + Hash + Display
{
    pub fn generate_dot_to_file( &self, file_name: String ) {
        let mut dot = String::new();
        dot.push_str( "digraph G {\n" );
        for ( node_id, node_data ) in self.nodes().iter() {
            node_data.data().variables().iter().for_each( |( _, _ )|
                dot.push_str( &format!( " {} [label=\"{}\"];\n", node_id, node_id ) )
            );

            for ( adj_node_id, edge ) in node_data.adjacencies().iter() {
                if *edge {
                    dot.push_str( &format!( " {} -> {} [label=\"{}\" color=\"blue\"];\n", node_id, adj_node_id, edge ) );
                } else {
                    dot.push_str( &format!( " {} -> {} [label=\"{}\" color=\"red\"];\n", node_id, adj_node_id, edge ) );
                }
            }
        }
        dot.push_str( "}\n" );
        std::fs::write( file_name, dot ).unwrap();
    }

    pub fn add_operation<const N: usize, F, Fut>( &mut self, id: I, variables: [ ( J, Variable ); N ], function: F ) -> Result<(), Error>
    where
        F: 'static + Fn(&Variables<J>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.add_node( id, AsyncOperation::new(
            variables,
            function
        ))?;
        Ok( () )
    }
}

impl<'a, I, J> GraphTraits<'a, I, AsyncOperation<J>, bool> for AsyncFnGraph<I, J>
where
    I: 'a + Clone + Ord,
    J: 'static + Clone + Ord + Hash
{}

impl<'a, I, J> AsyncTraverserTraits<'a, AsyncFn, I, AsyncOperation<J>, bool, AsyncFnGraph<I, J>> for FnTraverser<'a, I, J>
where
    I: 'a + Clone + Ord,
    J: 'static + Clone + Ord + Hash,
    Self: TraverserAccess<'a, AsyncFn, I, AsyncOperation<J>, bool, AsyncFnGraph<I, J>>
{
    async fn bfs_step( &'a self, queue: &mut VecDeque<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        while let Some(current_id) = queue.pop_front() {
            if visited.insert( current_id.clone() ) {
                if let Some( current_node ) = self.graph().data().get( &current_id ) {
                    for ( next_id, edge ) in current_node.adjacencies().iter() {
                        if *edge && !visited.contains( next_id ) {
                            queue.push_back( next_id.clone() );
                        }
                    }
                }
                return Some( current_id );
            }
        }
        None
    }

    async fn dfs_step( &'a self, stack: &mut Vec<I>, visited: &mut BTreeSet<I> ) -> Option<I> {
        while let Some( current_id ) = stack.pop() {
            if visited.insert( current_id.clone() ) {
                if let Some( current_node ) = self.graph().data().get( &current_id ) {
                    for ( next_id, edge ) in current_node.adjacencies().iter() {
                        if *edge && !visited.contains( next_id ) {
                            stack.push( next_id.clone() );
                        }
                    }
                }
                return Some( current_id );
            }
        }
        None
    }

    async fn bfs( &'a self, start: I ) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back( start.clone() );
        while !queue.is_empty() {
            if let Some( current_id ) = self.bfs_step( &mut queue, &mut visited ).await {
                if let Some( operation ) = self.graph().data().get_node( current_id ) {
                    operation.execute().await.unwrap();
                }
            }
        }
    }

    async fn dfs( &'a self, start: I ) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push( start.clone() );
        while !stack.is_empty() {
            if let Some( current_id ) = self.dfs_step(&mut stack, &mut visited).await {
                if let Some( operation ) = self.graph().data().get_node( current_id ) {
                    operation.execute().await.unwrap();
                }
            }
        }
    }
}

impl<'a, I, J> Traversable<'a, AsyncFn, I, AsyncOperation<J>, bool> for AsyncFnGraph<I, J>
where
    I: 'a + Clone + Ord,
    J: 'static + Clone + Ord + Hash,
{}
