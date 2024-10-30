// Copyright 2024 Bewusstsein Labs

use std::sync::{Arc, Mutex};

use crate::graph::{
    self,
    graph_data::error::Error,
    GraphTraits,
    undirected_graph::UndirectedGraph,
    directed_graph::DirectedGraph,
    function_graph::{ FunctionGraph, FunctionNode, error::Error as FunctionGraphError }
};

#[test]
fn test_directed_graph() -> Result<(), Error> {
    let mut graph = DirectedGraph::new();
    graph.add_node( 0, "Node 0" )?;
    graph.add_node( 1, "Node 1" )?;
    graph.add_node( 2, "Node 2" )?;
    graph.add_node( 3, "Node 3" )?;
    graph.add_edge( 0, 1, "Edge 0-1" )?;
    graph.add_edge( 0, 2, "Edge 0-2" )?;
    graph.add_edge( 1, 3, "Edge 1-3" )?;
    graph.add_edge( 2, 3, "Edge 2-3" )?;
    graph.generate_dot_to_file("directed_graph.dot".to_string());
    println!("Depth First Search:");
    graph.dfs( 0 );
    println!("Breadth First Search:");
    graph.bfs( 0 );
    Ok( () )
}

#[test]
fn test_undirected_graph() -> Result<(), Error> {
    let mut graph = UndirectedGraph::new();
    graph.add_node( 0, "Node 0" )?;
    graph.add_node( 1, "Node 1" )?;
    graph.add_node( 2, "Node 2" )?;
    graph.add_node( 3, "Node 3" )?;
    graph.add_edge( 0, 1, "Edge 0-1" )?;
    graph.add_edge( 0, 2, "Edge 0-2" )?;
    graph.add_edge( 1, 3, "Edge 1-3" )?;
    graph.add_edge( 2, 3, "Edge 2-3" )?;
    graph.generate_dot_to_file("undirected_graph.dot".to_string());
    println!("Depth First Search:");
    graph.dfs( 3 );
    println!("Breadth First Search:");
    graph.bfs( 3 );
    Ok( () )
}

#[test]
fn test_functional_graph() -> Result<(), FunctionGraphError> {
    let mut graph = FunctionGraph::new();
    let var0 = Arc::new( Mutex::new( 1 ) );
    let var1 = Arc::new( Mutex::new( 2 ) );
    let var2 = Arc::new( Mutex::new( 2 ) );
    let var3 = Arc::new( Mutex::new( 0 ) );

    graph.add_node( 0, FunctionNode::new( var0.clone(), Box::new( | input: &i32, output: &mut i32 | *output += *input ), var1.clone() ) )?;
    graph.add_node( 1, FunctionNode::new( var0.clone(), Box::new( | input: &i32, output: &mut i32 | *output += *input ), var2.clone() ) )?;
    graph.add_node( 2, FunctionNode::new( var1.clone(), Box::new( | input: &i32, output: &mut i32 | *output += *input ), var3.clone() ) )?;
    graph.add_node( 3, FunctionNode::new( var2.clone(), Box::new( | input: &i32, output: &mut i32 | *output += *input ), var3.clone() ) )?;
    graph.add_edge( 0, 1, () )?;
    graph.add_edge( 0, 2, () )?;
    graph.add_edge( 1, 3, () )?;
    graph.add_edge( 2, 3, () )?;

    println!("Var 0: {}", var0.lock().unwrap() );
    println!("Var 1: {}", var1.lock().unwrap() );
    println!("Var 2: {}", var2.lock().unwrap() );
    println!("Var 3: {}", var3.lock().unwrap() );

    graph.bfs( 0 );

    println!("Var 0: {}", var0.lock().unwrap() );
    println!("Var 1: {}", var1.lock().unwrap() );
    println!("Var 2: {}", var2.lock().unwrap() );
    println!("Var 3: {}", var3.lock().unwrap() );

    Ok( () )
}