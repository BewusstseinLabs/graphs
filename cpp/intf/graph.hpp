// Copyright 2024 Bewusstsein Labs

#ifndef GRAPHS_INTF_GRAPH_HPP_
#define GRAPHS_INTF_GRAPH_HPP_

#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <fstream>

namespace bewusstsein {

    namespace graph {

        using Id = int;

        enum class Error {
            NONE,
            NODE_NOT_FOUND,
            EDGE_NOT_FOUND,
            NODE_ALREADY_EXISTS,
            EDGE_ALREADY_EXISTS,
            NODES_NOT_ADJACENT,
            NODES_ALREADY_ADJACENT
        };

        std::ostream& operator<<( std::ostream& os, const Error& error );

        template <typename T>
        class Graph {

            protected:
                std::map<T, std::set<T>> adjacencies;

            public:
                Graph() = default;

            public:
                virtual ~Graph() = default;

            public:
                Error   add_node( const T node );
                Error   remove_node( const T node );
                Error   add_edge( const T node_1, const T node_2 );
                Error   remove_edge( const T node_1, const T node_2 );
                void    clear();
                void    clear_edges();

            private:
                template<typename U> friend void            bfs( const Graph<U>& graph, const U start_node );
                template<typename U> friend void            dfs( const Graph<U>& graph, const U start_node );
                template<typename U> friend std::ostream&   operator<<( std::ostream& os, const Graph<U>& graph );
                template<typename U> friend bool            operator==( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            operator!=( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            operator<=( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            operator>=( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            operator<( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            operator>( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend Graph<U>        operator+( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend Graph<U>        operator-( const Graph<U>& lhs, const Graph<U>& rhs );
                template<typename U> friend bool            is_complete( const Graph<U>& graph );
                template<typename U> friend bool            is_empty( const Graph<U>& graph );
                template<typename U> friend bool            is_trivial( const Graph<U>& graph );
                template<typename U> friend bool            is_null( const Graph<U>& graph );
                template<typename U> friend bool            is_child_node( const Graph<U>& graph, const U node_1 );
                template<typename U> friend bool            is_subgraph( const Graph<U>& graph, const Graph<U>& subgraph );
                template<typename U> friend bool            is_proper_subgraph( const Graph<U>& graph, const Graph<U>& subgraph );
                template<typename U> friend bool            is_improper_subgraph( const Graph<U>& graph, const Graph<U>& subgraph );
                template<typename U> friend bool            is_spanning_subgraph( const Graph<U>& graph, const Graph<U>& subgraph );
                template<typename U> friend bool            are_adjacent_nodes( const Graph<U>& graph, const U node_1, const U node_2 );
                template<typename U> friend bool            are_adjacent_edges( const Graph<U>& graph, const U node_1, const U node_2, const U node_3 );
                template<typename U> friend size_t          order( const Graph<U>& graph );
                template<typename U> friend size_t          size( const Graph<U>& graph );
                template<typename U> friend void            generate_dot_graph( const Graph<U>& graph, const std::string& filename );

        }; // class Graph

        template<typename T> void           bfs( const Graph<T>& graph, const T start_node );
        template<typename T> void           dfs( const Graph<T>& graph, const T start_node );
        template<typename T> std::ostream&  operator<<( std::ostream& os, const Graph<T>& graph );
        template<typename T> bool           operator==( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           operator!=( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           operator<=( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           operator>=( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           operator<( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           operator>( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> Graph<T>       operator+( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> Graph<T>       operator-( const Graph<T>& lhs, const Graph<T>& rhs );
        template<typename T> bool           is_complete( const Graph<T>& graph );
        template<typename T> bool           is_empty( const Graph<T>& graph );
        template<typename T> bool           is_trivial( const Graph<T>& graph );
        template<typename T> bool           is_null( const Graph<T>& graph );
        template<typename T> bool           is_child_node( const Graph<T>& graph, const T node_1 );
        template<typename T> bool           is_subgraph( const Graph<T>& graph, const Graph<T>& subgraph );
        template<typename T> bool           is_proper_subgraph( const Graph<T>& graph, const Graph<T>& subgraph );
        template<typename T> bool           is_improper_subgraph( const Graph<T>& graph, const Graph<T>& subgraph );
        template<typename T> bool           is_spanning_subgraph( const Graph<T>& graph, const Graph<T>& subgraph );
        template<typename T> bool           are_adjacent_nodes( const Graph<T>& graph, const T node_1, const T node_2 );
        template<typename T> bool           are_adjacent_edges( const Graph<T>& graph, const T node_1, const T node_2, const T node_3 );
        template<typename T> size_t         order( const Graph<T>& graph );
        template<typename T> size_t         size( const Graph<T>& graph );
        template<typename T> void           generate_dot_graph( const Graph<T>& graph, const std::string& filename );

    } // namespace graph

} // namespace bewusstsein

#endif // GRAPHS_INTF_GRAPH_HPP_