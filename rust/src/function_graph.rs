// Copyright 2024 Bewusstsein Labs

#![warn(clippy::type_complexity)]

pub mod variable;
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
            TraverserTraits,
            Traversable
        }
    },
    function_graph::{
        variable::{ Variable, Variables },
        operation::{ Operation, Error as OperationError }
    }
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: {0}")]
    GraphError( #[from] GraphError),
    #[error("Operation Error: {0}")]
    OperationError( #[from] OperationError )
}

pub struct Functional ();
impl GraphType for Functional {}
pub type FnGraph<I, J> = Graph<Functional, I, Operation<J>, ()>;
pub type FnTraverser<'a, I, J> = Traverser<'a, Functional, I, Operation<J>, ()>;

impl<I, J> FnGraph<I, J>
where
    I: Clone + PartialEq + Ord + Display + 'static,
    J: Clone + PartialEq + Ord + Hash + Display + Send + 'static
{
    pub fn generate_dot_to_file( &self, file_name: String ) {
        let mut dot = String::new();
        dot.push_str( "digraph G {\n" );
        for ( node_id, node_data ) in self.nodes().iter() {
            node_data.data().variables().iter().for_each( |( _, _ )|
                dot.push_str( "" )
            );

            for adj_node_id in node_data.adjacencies().keys() {
                dot.push_str( &format!( " {} -> {};\n", node_id, adj_node_id ) );
            }
        }
        dot.push_str( "}\n" );
        std::fs::write( file_name, dot ).unwrap();
    }

    #[allow(dead_code)]
    fn variable_to_string( variable: &Variable ) -> String {
        if let Some( val ) = variable.read().downcast_ref::<i32>() {
            val.to_string()
        } else if let Some( val ) = variable.read().downcast_ref::<f64>() {
            val.to_string()
        } else if let Some( val ) = variable.read().downcast_ref::<String>() {
            val.clone()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn add_operation<const N: usize, F>( &mut self, id: I, variables: [ ( J, Variable ); N ], function: F ) -> Result<(), Error>
    where
        F: Fn( &Variables<J> ) + Send + Sync + 'static
    {
        self.add_node( id, Operation::new(
            variables,
            function
        ))?;
        Ok( () )
    }
}

impl<'a, I, J> GraphTraits<'a, I, Operation<J>, ()> for FnGraph<I, J>
where
    I: 'static + Clone + PartialEq + Ord,
    J: 'static + Clone + PartialEq + Ord + Hash
{}

impl<'a, I, J> TraverserTraits<'a, Functional, I, Operation<J>, (), FnGraph<I, J>> for Traverser<'a, I, Operation<J>, (), FnGraph<I, J>>
where
    I: 'static + Clone + PartialEq + Ord,
    J: 'static + Clone + PartialEq + Ord + Hash,
    Self: TraverserAccess<'a, Functional, I, Operation<J>, (), FnGraph<I, J>>
{
    fn bfs( &'a self, start: I ) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back( start.clone() );
        while !queue.is_empty() {
            if let Some( current_id ) = self.bfs_step( &mut queue, &mut visited ) {
                if let Some( operation ) = self.graph().data().get_node( current_id ) {
                    operation.execute().unwrap();
                }
            }
        }
    }

    fn dfs( &'a self, start: I ) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push( start.clone() );
        while !stack.is_empty() {
            if let Some( current_id ) = self.dfs_step(&mut stack, &mut visited) {
                if let Some( operation ) = self.graph().data().get_node( current_id ) {
                    operation.execute().unwrap();
                }
            }
        }
    }
}

impl<'a, I, J> Traversable<'a, Functional, I, Operation<J>, ()> for FnGraph<I, J>
where
    I: 'static + Clone + PartialEq + Ord,
    J: 'static + Clone + PartialEq + Ord + Hash,
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
        function_graph::{
            FnGraph,
            Variable
        }
    };

    #[test]
    fn test() {
        let a = Variable::new( 0 );
        let b = Variable::new( 0 );
        let c = Variable::new( 0 );
        let d = Variable::new( 0 );
        let e = Variable::new( "hello".to_string() );
        let f = Variable::new( "world".to_string() );

        //graph.generate_dot_to_file( "graphs/function_graph_before.dot".to_string() );
        let start = std::time::Instant::now();
        {
            if let ( Some( a ), Some( b ) ) = (
                a.read().downcast_ref::<i32>(),
                b.write().downcast_mut::<i32>()
            ) {
                *b = *a + 2;
                println!( "{} = {} + 2", *b, *a );
            }
            if let ( Some( b ), Some( c ) ) = (
                b.read().downcast_ref::<i32>(),
                c.write().downcast_mut::<i32>()
            ) {
                *c = *b * 4;
                println!( "{} = {} * 4", *c, *b );
            }
            if let ( Some( c ), Some( d ) ) = (
                c.read().downcast_ref::<i32>(),
                d.write().downcast_mut::<i32>()
            ) {
                *d = *c - 1;
                println!( "{} = {} - 1", *d, *c );
            }
            if let Some( e ) = e.read().downcast_ref::<String>() {
                println!( "{}", e );
            }
            if let Some( f ) = f.read().downcast_ref::<String>() {
                println!( "{}", f );
            }
        }
        let duration = start.elapsed();
        println!("Time taken to traverse the graph: {:?}", duration);
        //graph.generate_dot_to_file( "graphs/function_graph_after.dot".to_string() );
    }

    #[test]
    fn test_function_graph() {
        let mut graph = FnGraph::<char, char>::new();

        let a = Variable::new( 0 );
        let b = Variable::new( 0 );
        let c = Variable::new( 0 );
        let d = Variable::new( 0 );
        let e = Variable::new( "hello".to_string() );
        let f = Variable::new( "world".to_string() );

        graph.add_operation( 'a',
            [
                ( 'a', a.clone() ),
                ( 'b', b.clone() )
            ],
            |variables| {
                if let ( Some( a ), Some( b ) ) = (
                    variables.read( &'a' ).downcast_ref::<i32>(),
                    variables.write( &'b' ).downcast_mut::<i32>()
                ) {
                    *b = *a + 2;
                    println!( "{} = {} + 2", *b, *a );
                }
            }
        ).unwrap();
        graph.add_operation( 'b',
            [
                ( 'b', b.clone() ),
                ( 'c', c.clone() )
            ],
            |variables| {
                if let ( Some( b ), Some( c ) ) = (
                    variables.read( &'b' ).downcast_ref::<i32>(),
                    variables.write( &'c' ).downcast_mut::<i32>()
                ) {
                    *c = *b * 4;
                    println!( "{} = {} * 4", *c, *b );
                }
            }
        ).unwrap();
        graph.add_operation( 'c',
            [
                ( 'c', c.clone() ),
                ( 'd', d.clone() )
            ],
            |variables| {
                if let ( Some( c ), Some( d ) ) = (
                    variables.read( &'c' ).downcast_ref::<i32>(),
                    variables.write( &'d' ).downcast_mut::<i32>()
                ) {
                    *d = *c - 1;
                    println!( "{} = {} - 1", *d, *c );
                }
            }
        ).unwrap();
        graph.add_operation( 'd',
            [ ( 'e', e.clone() ) ],
            |variables| {
                if let Some( e ) = variables.read( &'e' ).downcast_ref::<String>() {
                    println!( "{}", e );
                }
            }
        ).unwrap();
        graph.add_operation( 'e',
            [ ( 'f', f.clone() ) ],
            |variables| {
                if let Some( f ) = variables.read( &'f' ).downcast_ref::<String>() {
                    println!( "{}", f );
                }
            }
        ).unwrap();
        graph.add_operation( 'f', [], |_| println!( "Done!" ) ).unwrap();

        graph.add_edge( 'a', 'b', () ).unwrap();
        graph.add_edge( 'b', 'c', () ).unwrap();
        graph.add_edge( 'c', 'd', () ).unwrap();
        graph.add_edge( 'd', 'e', () ).unwrap();
        graph.add_edge( 'e', 'f', () ).unwrap();

        //graph.generate_dot_to_file( "graphs/function_graph_before.dot".to_string() );
        let start = std::time::Instant::now();
        graph.traverser().bfs( 'a' );
        let duration = start.elapsed();
        println!("Time taken to traverse the graph: {:?}", duration);
        //graph.generate_dot_to_file( "graphs/function_graph_after.dot".to_string() );
    }

    #[test]
    fn test_string_equation_graph() {
        let mut graph = FnGraph::<char, char>::new();

        // Define variables as strings
        let a = Variable::new( "2".to_string() );
        let b = Variable::new( "+".to_string() );
        let c = Variable::new( "3".to_string() );
        let d = Variable::new( "=".to_string() );
        let e = Variable::new( "5".to_string() );

        // Add nodes to the graph
        graph.add_operation( 'a',
            [ ( 'a', a.clone() ) ],
            |variables| {
                if let Some( a ) = variables.read( &'a' ).downcast_ref::<String>() {
                    print!( "{} ", a );
                }
            }
        ).unwrap();

        graph.add_operation( 'b',
            [ ( 'b', b.clone() ) ],
            |variables| {
                if let Some( b ) = variables.read( &'b' ).downcast_ref::<String>() {
                    print!( "{} ", b );
                }
            }
        ).unwrap();

        graph.add_operation( 'c',
            [ ( 'c', c.clone() ) ],
            |variables| {
                if let Some( c ) = variables.read( &'c' ).downcast_ref::<String>() {
                    print!( "{} ", c );
                }
            }
        ).unwrap();

        graph.add_operation( 'd',
            [ ( 'd', d.clone() ) ],
            |variables| {
                if let Some( d ) = variables.read( &'d' ).downcast_ref::<String>() {
                    print!( "{} ", d );
                }
            }
        ).unwrap();

        graph.add_operation( 'e',
            [ ( 'e', e.clone() ) ],
            |variables| {
                if let Some( e ) = variables.read( &'e' ).downcast_ref::<String>() {
                    println!( "{}", e );
                }
            }
        ).unwrap();

        graph.add_edge( 'a', 'b', () ).unwrap();
        graph.add_edge( 'b', 'c', () ).unwrap();
        graph.add_edge( 'c', 'd', () ).unwrap();
        graph.add_edge( 'd', 'e', () ).unwrap();

        //graph.generate_dot_to_file( "graphs/string_equation_graph_before.dot".to_string() );
        let start = std::time::Instant::now();
        graph.traverser().bfs( 'a' );
        let duration = start.elapsed();
        println!( "Time taken to traverse the graph: {:?}", duration );
        //graph.generate_dot_to_file( "graphs/string_equation_graph_after.dot".to_string() );
    }

    #[test]
    fn test_function_graph_with_multiple_branches() {
        let mut graph = FnGraph::<char, char>::new();
        let a = Variable::new( 0 );
        let b = Variable::new( 0 );
        let c = Variable::new( 0 );
        let d = Variable::new( 0 );
        let e = Variable::new( 0 );
        let f = Variable::new( 0 );
        let g = Variable::new( 0 );
        let h = Variable::new( 0 );
        let i = Variable::new( 0 );

        // Node 0: Add 2
        graph.add_operation( 'a',
            [
                ( 'a', a.clone() ),
                ( 'b', b.clone() )
            ],
            |variables| {
                if let ( Some( a ), Some( b ) ) = (
                    variables.read( &'a' ).downcast_ref::<i32>(),
                    variables.write( &'b' ).downcast_mut::<i32>()
                ) {
                    *b = *a + 2;
                    println!( "{} = {} + 2", *b, *a );
                }
            }
        ).unwrap();

        // Node 1: Multiply by 4
        graph.add_operation( 'b',
            [
                ( 'b', b.clone() ),
                ( 'c', c.clone() )
            ],
            |variables| {
                if let ( Some( b ), Some( c ) ) = (
                    variables.read( &'b' ).downcast_ref::<i32>(),
                    variables.write( &'c' ).downcast_mut::<i32>()
                ) {
                    *c = *b * 4;
                    println!( "{} = {} * 4", *c, *b );
                }
            }
        ).unwrap();

        // Node 2: Check if divisible by 3
        graph.add_operation( 'c',
            [
                ( 'c', c.clone() ),
                ( 'd', d.clone() )
            ],
            |variables| {
                if let ( Some( c ), Some( d ) ) = (
                    variables.read( &'c' ).downcast_ref::<i32>(),
                    variables.write( &'d' ).downcast_mut::<i32>()
                ) {
                    if c % 3 == 0 {
                        *d = 1; // Go to divisible by 3 branch
                        println!( "{} is divisible by 3", *c );
                    } else {
                        *d = 0; // Go to not divisible by 3 branch
                        println!( "{} is not divisible by 3", *c );
                    }
                }
            }
        ).unwrap();

        // Node 3: Divisible by 3 branch - Add 5
        graph.add_operation( 'd',
            [
                ( 'c', c.clone() ),
                ( 'e', e.clone() )
            ],
            |variables| {
                if let ( Some( c ), Some( e ) ) = (
                    variables.read( &'c' ).downcast_ref::<i32>(),
                    variables.write( &'e' ).downcast_mut::<i32>()
                ) {
                    *e = *c + 5;
                    println!( "{} = {} + 5 (divisible by 3 branch)", *e, *c );
                }
            }
        ).unwrap();

        // Node 4: Not divisible by 3 branch - Subtract 2
        graph.add_operation( 'e',
            [
                ( 'c', c.clone() ),
                ( 'f', f.clone() )
            ],
            |variables| {
                if let ( Some( c ), Some( f ) ) = (
                    variables.read( &'z' ).downcast_ref::<i32>(),
                    variables.write( &'f' ).downcast_mut::<i32>()
                ) {
                    *f = *c - 2;
                    println!( "{} = {} - 2 (not divisible by 3 branch)", *f, *c );
                }
            }
        ).unwrap();

        // Node 5: Further branch from divisible by 3 - Multiply by 2
        graph.add_operation( 'f',
            [
                ( 'e', e.clone() ),
                ( 'g', g.clone() )
            ],
            |variables| {
                if let ( Some( e ), Some( g ) ) = (
                    variables.read( &'e' ).downcast_ref::<i32>(),
                    variables.write( &'g' ).downcast_mut::<i32>()
                ) {
                    *g = *e * 2;
                    println!( "{} = {} * 2 (further divisible by 3 branch)", *g, *e );
                }
            }
        ).unwrap();

        // Node 6: Further branch from not divisible by 3 - Add 7
        graph.add_operation( 'g',
            [
                ( 'f', f.clone() ),
                ( 'h', h.clone() )
            ],
            |variables| {
                if let ( Some( f ), Some( h ) ) = (
                    variables.read( &'f' ).downcast_ref::<i32>(),
                    variables.write( &'h' ).downcast_mut::<i32>()
                ) {
                    *h = *f + 7;
                    println!( "{} = {} + 7 (further not divisible by 3 branch)", *h, *f );
                }
            }
        ).unwrap();

        // Node 7: Converge both branches - Subtract 1
        graph.add_operation( 'h',
            [
                ( 'g', g.clone() ),
                ( 'i', i.clone() )
            ],
            |variables| {
                if let ( Some( g ), Some( i ) ) = (
                    variables.read(  &'g' ).downcast_ref::<i32>(),
                    variables.write( &'i' ).downcast_mut::<i32>()
                ) {
                    *i = *g - 1;
                    println!( "{} = {} - 1 (converged branch)", *i, *g );
                }
            }
        ).unwrap();

        // Edges
        graph.add_edge( 'a', 'b', () ).unwrap();
        graph.add_edge( 'b', 'c', () ).unwrap();
        graph.add_edge( 'c', 'd', () ).unwrap(); // Divisible by 3 branch
        graph.add_edge( 'c', 'e', () ).unwrap(); // Not divisible by 3 branch
        graph.add_edge( 'd', 'f', () ).unwrap(); // Further divisible by 3 branch
        graph.add_edge( 'e', 'g', () ).unwrap(); // Further not divisible by 3 branch
        graph.add_edge( 'f', 'h', () ).unwrap(); // Converge branch
        graph.add_edge( 'g', 'h', () ).unwrap(); // Converge branch

        //graph.generate_dot_to_file( "graphs/function_graph_with_multiple_branches_before.dot".to_string() );
        let start = std::time::Instant::now();
        graph.traverser().bfs( 'a' );
        let duration = start.elapsed();
        println!( "Time taken to traverse the graph: {:?}", duration );
        //graph.generate_dot_to_file( "graphs/function_graph_with_multiple_branches_after.dot".to_string() );
    }

    #[test]
    fn test_function_subgraph() {
        let mut graph = FnGraph::<char, char>::new();
        let mut sub_graph = FnGraph::<char, char>::new();

        let a = Variable::new( 'a' );
        let b = Variable::new( 'b' );
        let c = Variable::new( 'c' );
        let d = Variable::new( 'd' );

        sub_graph.add_operation( 'a',
            [ ( 'a', a.clone() ) ],
            |variables| {
                if let Some( a ) = variables.read( &'a' ).downcast_ref::<char>() {
                    println!( "{}", a );
                }
            }
        ).unwrap();

        sub_graph.add_operation( 'b',
            [ ( 'b', b.clone() ) ],
            |variables| {
                if let Some( b ) = variables.read( &'b' ).downcast_ref::<char>() {
                    println!( "{}", b );
                }
            }
        ).unwrap();

        sub_graph.add_operation( 'c',
            [ ( 'c', c.clone() ) ],
            |variables| {
                if let Some( c ) = variables.read( &'c' ).downcast_ref::<char>() {
                    println!( "{}", c );
                }
            }
        ).unwrap();

        sub_graph.add_operation( 'd',
            [ ( 'd', d.clone() ) ],
            |variables| {
                if let Some( d ) = variables.read( &'d' ).downcast_ref::<char>() {
                    println!( "{}", d );
                }
            }
        ).unwrap();

        sub_graph.add_edge( 'a', 'b', () ).unwrap();
        sub_graph.add_edge( 'b', 'c', () ).unwrap();
        sub_graph.add_edge( 'c', 'd', () ).unwrap();

        let e = Variable::new( sub_graph );

        graph.add_operation( 'a',
            [ ( 'e', e.clone() ) ],
            |variables| {
                if let Some( e ) = variables.read( &'e' ).downcast_ref::<FnGraph<char, char>>() {
                    e.traverser().bfs( 'a' );
                }
            }
        ).unwrap();

        let start = std::time::Instant::now();
        graph.traverser().bfs( 'a' );

        let duration = start.elapsed();
        println!( "Time taken to traverse the graph: {:?}", duration );
    }

    #[test]
    fn test_mpsc_graph() {
        use crossbeam::channel::{ bounded, Sender, Receiver };

        let mut graph = FnGraph::<&'static str, &'static str>::new();

        let a = Variable::new( 4 );
        let c = Variable::new( 0 );

        let ( a_sender, b_receiver ) = bounded::<i32>( 1 );
        let ( b_sender, c_receiver ) = bounded::<i32>( 1 );

        graph.add_operation( "a",
            [
                ( "a", a.clone() ),
                ( "a_sender", Variable::new( a_sender ) )
            ],
            |variables| {
                if let ( Some( a ), Some( a_sender ) ) = (
                    variables.read( &"a" ).downcast_ref::<i32>(),
                    variables.read( &"a_sender" ).downcast_ref::<Sender<i32>>()
                ) {
                    a_sender.send( *a * 2 ).unwrap();
                }
            }
        ).unwrap();

        graph.add_operation( "b",
            [
                ( "b_receiver", Variable::new( b_receiver ) ),
                ( "b_sender", Variable::new( b_sender ) )
            ],
            |variables| {
                if let ( Some( b_receiver ), Some( b_sender ) ) = (
                    variables.read( &"b_receiver" ).downcast_ref::<Receiver<i32>>(),
                    variables.read( &"b_sender" ).downcast_ref::<Sender<i32>>()
                ) {
                    if let Ok( b ) = b_receiver.try_recv() {
                        b_sender.send( b * 3 ).unwrap();
                    }
                }
            }
        ).unwrap();

        graph.add_operation( "c",
            [
                ( "c_receiver", Variable::new( c_receiver ) ),
                ( "c", c.clone() )
            ],
            |variables| {
                if let ( Some( c_receiver ), Some( c ) ) = (
                    variables.read( &"c_receiver" ).downcast_ref::<Receiver<i32>>(),
                    variables.write( &"c" ).downcast_mut::<i32>()
                ) {
                    if let Ok( recv ) = c_receiver.try_recv() {
                        *c = recv + 1;
                    }
                }
            }
        ).unwrap();

        graph.add_edge( "a", "b", () ).unwrap();
        graph.add_edge( "b", "c", () ).unwrap();

        let start = std::time::Instant::now();
        graph.traverser().bfs( "a" );
        let duration = start.elapsed();
        println!( "Time taken to traverse the graph: {:?}", duration );

        println!( "a: {}", a.read().downcast_ref::<i32>().unwrap() );
        println!( "c: {}", c.read().downcast_ref::<i32>().unwrap() );
    }
}
