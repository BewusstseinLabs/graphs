use crate::graph::{
    error::Error,
    GraphTraits,
    undirected_graph::UndirectedGraph,
    directed_graph::DirectedGraph,
    function_graph::FunctionGraph
};

#[test]
fn test_directional_graph() -> Result<(), Error> {
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
fn test_undirectional_graph() -> Result<(), Error> {
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
    graph.dfs( 0 );
    println!("Breadth First Search:");
    graph.bfs( 0 );
    Ok( () )
}

#[test]
fn test_functional_graph() -> Result<(), Error> {
    let mut graph = FunctionGraph::<i32, f32, fn( a: &f32, b: &f32) -> f32>::new();
    graph.add_node( 0, 0.1 )?;
    graph.add_node( 1, 0.2 )?;
    graph.add_node( 2, 0.2 )?;
    graph.add_node( 3, 0.0 )?;
    graph.add_edge( 0, 1, | input: &f32, output: &f32 | -> f32 { *input + *output } )?;
    graph.add_edge( 0, 2, | input: &f32, output: &f32 | -> f32 { *input + *output } )?;
    graph.add_edge( 1, 3, | input: &f32, output: &f32 | -> f32 { *input + *output } )?;
    graph.add_edge( 2, 3, | input: &f32, output: &f32 | -> f32 { *input + *output } )?;

    println!("Before:");
    println!("Node 0: {}", graph.get_node( 0 ).unwrap());
    println!("Node 1: {}", graph.get_node( 1 ).unwrap());
    println!("Node 2: {}", graph.get_node( 2 ).unwrap());
    println!("Node 3: {}", graph.get_node( 3 ).unwrap());

    graph.generate_dot_to_file("function_graph_before.dot".to_string());

    graph.bfs( 0 );

    graph.generate_dot_to_file("function_graph_after.dot".to_string());

    println!("Before:");
    println!("Node 0: {}", graph.get_node( 0 ).unwrap());
    println!("Node 1: {}", graph.get_node( 1 ).unwrap());
    println!("Node 2: {}", graph.get_node( 2 ).unwrap());
    println!("Node 3: {}", graph.get_node( 3 ).unwrap());

    Ok( () )
}