// Copyright 2024 Bewusstsein Labs

#ifndef GRAPHS_INTF_DIRECTED_GRAPH_HPP_
#define GRAPHS_INTF_DIRECTED_GRAPH_HPP_

#include <iostream>
#include <vector>
#include <set>
#include <map>

#include "intf/graph.hpp"

namespace bewusstsein {

    namespace graph {

        class DirectedGraph {

            protected:

            public:
                DirectedGraph() = default;

            public:
                virtual ~DirectedGraph() = default;

            public:

            private:
                friend  std::ostream&   operator<<( std::ostream& os, const DirectedGraph& graph );
                friend  DirectedGraph   operator+( DirectedGraph& lhs, DirectedGraph& rhs );
                friend  DirectedGraph   operator-( DirectedGraph& lhs, DirectedGraph& rhs );

        }; // class Graph

        std::ostream&   operator<<( std::ostream& os, const DirectedGraph& graph );
        DirectedGraph   operator+( Graph& lhs, DirectedGraph& rhs );
        DirectedGraph   operator-( Graph& lhs, DirectedGraph& rhs );

    } // namespace graph

} // namespace bewusstsein

#endif // GRAPHS_INTF_DIRECTED_GRAPH_HPP_