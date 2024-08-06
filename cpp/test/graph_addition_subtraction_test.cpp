// Copyright 2024 Shane W. Mulcahy

#include "graphs/intf/graph.hpp"

using namespace atom;

int main() {
    graph::Graph<std::string> graph_1;
    graph_1.add_node( "A" );
    graph_1.add_node( "B" );
    graph_1.add_node( "C" );
    graph_1.add_node( "D" );

    std::cout << graph_1.add_edge( "A", "B" ) << std::endl;
    std::cout << graph_1.add_edge( "B", "C" ) << std::endl;
    std::cout << graph_1.add_edge( "C", "D" ) << std::endl;
    std::cout << graph_1.add_edge( "D", "A" ) << std::endl << std::endl;

    graph::Graph<std::string> graph_2;
    graph_2.add_node( "A" );
    graph_2.add_node( "B" );
    graph_2.add_node( "C" );
    graph_2.add_node( "D" );

    std::cout << graph_2.add_edge( "A", "C" ) << std::endl;
    std::cout << graph_2.add_edge( "B", "D" ) << std::endl << std::endl;

    graph::Graph<std::string> graph_3 = graph_1 + graph_2;
    graph::Graph<std::string> graph_4 = graph_3 - graph_2;

    std::cout << "graph_1:" << std::endl << graph_1 << std::endl;
    std::cout << "graph_2:" << std::endl << graph_2 << std::endl;
    std::cout << "graph_3:" << std::endl << graph_3 << std::endl;
    std::cout << "graph_4:" << std::endl << graph_4 << std::endl;

    std::cout << "Is graph_2 a subgraph of graph_3: " << std::boolalpha << graph::is_subgraph( graph_3, graph_2 ) << std::endl;

    std::cout << "graph_2 order: " << order( graph_2 ) << std::endl;
    std::cout << "graph_2 size: " << size( graph_2 ) << std::endl;

    std::cout << "graph_3 order: " << order( graph_3 ) << std::endl;
    std::cout << "graph_3 size: " << size( graph_3 ) << std::endl;

    graph::generate_dot_graph( graph_3, "graph_3.dot" );

    return 0;
}