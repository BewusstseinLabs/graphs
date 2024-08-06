use crate::graph::{DirectedGraph, GraphTraits, Error};

#[test]
fn test_graph() -> Result<(), Error> {
    let mut graph = DirectedGraph::new();
    graph.add_node( 1, "Node 1" )?;
    graph.add_node( 2, "Node 2" )?;
    graph.add_node( 3, "Node 3" )?;
    graph.add_node( 4, "Node 4" )?;
    graph.add_adjacency( 1, 2, "Edge 1-2" )?;
    graph.add_adjacency( 1, 3, "Edge 1-3" )?;
    graph.add_adjacency( 2, 4, "Edge 2-4" )?;
    graph.add_adjacency( 3, 4, "Edge 3-4" )?;
    println!("Depth First Search:");
    graph.dfs( 1 );
    println!("Breadth First Search:");
    graph.bfs( 1 );
    Ok( () )
}