// Copyright 2024 Shane W. Mulcahy

#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <fstream>

#include "intf/graph.hpp"

namespace atom {

    namespace graph {

        std::ostream& operator<<( std::ostream& os, const Error& error ) {
            switch ( error ) {
                case Error::NONE:
                    os << "No error";
                    break;
                case Error::NODE_NOT_FOUND:
                    os << "Node not found";
                    break;
                case Error::EDGE_NOT_FOUND:
                    os << "Edge not found";
                    break;
                case Error::NODE_ALREADY_EXISTS:
                    os << "Node already exists";
                    break;
                case Error::EDGE_ALREADY_EXISTS:
                    os << "Edge already exists";
                    break;
                case Error::NODES_NOT_ADJACENT:
                    os << "Nodes not adjacent";
                    break;
                case Error::NODES_ALREADY_ADJACENT:
                    os << "Nodes already adjacent";
                    break;
                default:
                    os << "Unknown error";
                    break;
            }
            return os;
        }

        template<typename T>
        void bfs( const Graph<T>& graph, const T start_node ) {
            std::set<T> visited;
            std::vector<T> queue;
            queue.push_back( start_node );
            visited.insert( start_node );
            while ( !queue.empty() ) {
                T node = queue.front();
                queue.erase( queue.begin() );
                std::cout << node << std::endl;
                for ( const T& neighbor : graph.adjacencies.at( node ) ) {
                    if ( visited.find( neighbor ) == visited.end() ) {
                        queue.push_back( neighbor );
                        visited.insert( neighbor );
                    }
                }
            }
        }

        template<typename T>
        void dfs( const Graph<T>& graph, const T start_node ) {
            std::set<T> visited;
            std::vector<T> stack;
            stack.push_back( start_node );
            visited.insert( start_node );
            while ( !stack.empty() ) {
                T node = stack.back();
                stack.pop_back();
                std::cout << node << std::endl;
                for ( const T& neighbor : graph.adjacencies.at( node ) ) {
                    if ( visited.find( neighbor ) == visited.end() ) {
                        stack.push_back( neighbor );
                        visited.insert( neighbor );
                    }
                }
            }
        }

        template<typename T>
        Error Graph<T>::add_node( T node ) {
            if ( this->adjacencies.find( node ) != this->adjacencies.end() ) {
                return Error::NODE_ALREADY_EXISTS;
            }
            this->adjacencies.insert( std::make_pair( node, std::set<T>() ) );
            return Error::NONE;
        }

        template<typename T>
        Error Graph<T>::remove_node( T node ) {
            if ( this->adjacencies.find( node ) == this->adjacencies.end() ) {
                return Error::NODE_NOT_FOUND;
            }
            this->adjacencies.erase( node );
            return Error::NONE;
        }

        template<typename T>
        Error Graph<T>::add_edge( T node_1, T node_2 ) {
            if ( this->adjacencies.find( node_1 ) == this->adjacencies.end() ) {
                return Error::NODE_NOT_FOUND;
            }
            if ( this->adjacencies.find( node_2 ) == this->adjacencies.end() ) {
                return Error::NODE_NOT_FOUND;
            }
            if ( this->adjacencies.at( node_1 ).find( node_2 ) != this->adjacencies.at( node_1 ).end() ) {
                return Error::NODES_ALREADY_ADJACENT;
            }
            this->adjacencies.at( node_1 ).insert( node_2 );
            this->adjacencies.at( node_2 ).insert( node_1 );
            return Error::NONE;
        }

        template<typename T>
        Error Graph<T>::remove_edge( T node_1, T node_2 ) {
            if ( this->adjacencies.find( node_1 ) == this->adjacencies.end() ) {
                return Error::NODE_NOT_FOUND;
            }
            if ( this->adjacencies.find( node_2 ) == this->adjacencies.end() ) {
                return Error::NODE_NOT_FOUND;
            }
            if ( this->adjacencies.at( node_1 ).find( node_2 ) == this->adjacencies.at( node_1 ).end() ) {
                return Error::NODES_NOT_ADJACENT;
            }
            this->adjacencies.at( node_1 ).erase( node_2 );
            this->adjacencies.at( node_2 ).erase( node_1 );
            return Error::NONE;
        }

        template<typename T>
        void Graph<T>::clear() {
            this->adjacencies.clear();
        }

        template<typename T>
        void Graph<T>::clear_edges() {
            for ( auto& adjacency : this->adjacencies ) {
                adjacency.second.clear();
            }
        }

        template<typename T>
        std::ostream& operator<<( std::ostream& os, const Graph<T>& graph ) {
            for ( const auto& adjacency : graph.adjacencies ) {
                os << "Node " << adjacency.first << " is connected to: " << std::endl;
                for ( const auto& node : adjacency.second ) {
                    os << "\t- Node " << node << std::endl;
                }
            }
            return os;
        }

        template<typename T> bool operator==( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return lhs.adjacencies == rhs.adjacencies;
        }

        template<typename T> bool operator!=( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return lhs.adjacencies != rhs.adjacencies;
        }

        template<typename T> bool operator<=( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return is_subgraph( rhs, lhs );
        }

        template<typename T> bool operator>=( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return is_subgraph( lhs, rhs );
        }

        template<typename T> bool operator<( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return is_proper_subgraph( rhs, lhs );
        }

        template<typename T> bool operator>( const Graph<T>& lhs, const Graph<T>& rhs ) {
            return is_proper_subgraph( lhs, rhs );
        }

        template<typename T>
        Graph<T> operator+( const Graph<T>& lhs, const Graph<T>& rhs ) {
            Graph<T> result;
            for ( const auto& [ node, neighbors ] : lhs.adjacencies ) {
                result.adjacencies[ node ] = neighbors;
            }
            for ( const auto& [ node, neighbors ] : rhs.adjacencies ) {
                if ( result.adjacencies.find( node ) == result.adjacencies.end() ) {
                    result.adjacencies[ node ] = neighbors;
                } else {
                    result.adjacencies[ node ].insert( neighbors.begin(), neighbors.end() );
                }
            }
            return result;
        }

        template<typename T>
        Graph<T> operator-( const Graph<T>& lhs, const Graph<T>& rhs ) {
            Graph<T> result = lhs;
            for ( const auto& [ node, neighbors ] : rhs.adjacencies ) {
                if ( result.adjacencies.find( node ) != result.adjacencies.end() ) {
                    for ( const auto& neighbor : neighbors ) {
                        result.adjacencies.at( node ).erase( neighbor );
                        if ( result.adjacencies.at( neighbor ).find( node ) != result.adjacencies.at( neighbor ).end() ) {
                            result.adjacencies.at( neighbor ).erase( node );
                        }
                    }
                    if ( result.adjacencies.at( node ).empty() ) {
                        result.adjacencies.erase( node );
                    }
                }
            }
            return result;
        }

        template<typename T>
        bool is_complete( const Graph<T>& graph ) {
            size_t n = graph.adjacencies.size();
            for ( const auto& [ node, neighbors ] : graph.adjacencies ) {
                if ( neighbors.size() != n - 1 ) {
                    return false;
                }
                for ( const T& neighbor : neighbors ) {
                    // The neighbor should also have the current node in its adjacency list
                    if (graph.adjacencies.at( neighbor ).find( node ) == graph.adjacencies.at( neighbor ).end() ) {
                        return false;
                    }
                }
            }
            return true;
        }

        template<typename T>
        bool is_empty( const Graph<T>& graph ) {
            if ( graph.adjacencies.size() == 0 ) {
                return true;
            }
            else {
                for ( const auto& [ node, neighbors ] : graph.adjacencies ) {
                    if ( neighbors.size() != 0 ) {
                        return false;
                    }
                }
            }
            return true;
        }

        template<typename T>
        bool is_trivial( const Graph<T>& graph ) {
            if ( graph.adjacencies.size() == 1 ) {
                for ( const auto& [ node, neighbors ] : graph.adjacencies ) {
                    if ( neighbors.size() == 0 ) {
                        return true;
                    }
                }
            }
            return false;
        }

        template<typename T>
        bool is_null( const Graph<T>& graph ) {
            if ( graph.adjacencies.size() == 0 ) {
                return true;
            }
            return false;
        }

        template<typename T>
        bool is_child_node( const Graph<T>& graph, const T node_1 ) {
            if ( graph.adjacencies.find( node_1 ) == graph.adjacencies.end() ) {
                return false;
            }
            return true;
        }

        template<typename T>
        bool is_subgraph( const Graph<T>& graph, const Graph<T>& subgraph ) {
            for ( const auto& [ node, neighbors ] : subgraph.adjacencies ) {
                if ( graph.adjacencies.find( node ) == graph.adjacencies.end() ) {
                    return false;
                }
                else if ( !std::includes( graph.adjacencies.at( node ).begin(), graph.adjacencies.at( node ).end(), neighbors.begin(), neighbors.end() ) ) {
                    return false;
                }
            }
            return true;
        }

        template<typename T>
        bool is_proper_subgraph( const Graph<T>& graph, const Graph<T>& subgraph ) {
            if ( graph == subgraph ) {
                return false;
            }
            else if ( is_subgraph( graph, subgraph ) ) {
                return true;
            }
            return false;
        }

        template<typename T>
        bool is_improper_subgraph( const Graph<T>& graph, const Graph<T>& subgraph ) {
            return graph.adjacencies == subgraph.adjacencies;
        }

        template<typename T>
        bool is_spanning_subgraph( const Graph<T>& graph, const Graph<T>& subgraph ) {
            if ( graph.adjacencies.size() != subgraph.adjacencies.size() ) {
                return false;
            }
            else if ( is_subgraph( graph, subgraph ) ) {
                return true;
            }
            return false;
        }

        template<typename T>
        bool are_adjacent_nodes( const Graph<T>& graph, const T node_1, const T node_2 ) {
            if ( is_child_node( graph, node_1 ) ) {
                return false;
            }
            if ( is_child_node( graph, node_2 ) ) {
                return false;
            }
            if ( graph.adjacencies.at( node_1 ).find( node_2 ) == graph.adjacencies.at( node_1 ).end() ) {
                return false;
            }
            return true;
        }

        template<typename T>
        bool are_adjacent_edges( const Graph<T>& graph, const T node_1, const T node_2, const T node_3 ) {
            if ( are_adjacent_nodes( graph, node_1, node_2 ) ) {
                return false;
            }
            if ( are_adjacent_nodes( graph, node_2, node_3 ) ) {
                return false;
            }
            return true;
        }

        template<typename T>
        size_t order( const Graph<T>& graph ) {
            return graph.adjacencies.size();
        }

        template<typename T>
        size_t size( const Graph<T>& graph ) {
            size_t size = 0;
            for ( const auto& [ node, neighbors ] : graph.adjacencies ) {
                size += neighbors.size();
            }
            return size / 2;
        }

        template<typename T>
        void generate_dot_graph( const Graph<T>& graph, const std::string& filename ) {
            std::ofstream file( filename );
            file << "graph {" << std::endl;
            for ( const auto& [ node, neighbors ] : graph.adjacencies ) {
                for ( const T& neighbor : neighbors ) {
                    file << "\t" << node << " -- " << neighbor << ";" << std::endl;
                }
            }
            file << "}" << std::endl;
        }

        template class Graph<Id>;
        template std::ostream&  operator<<( std::ostream& os, const Graph<Id>& graph );
        template bool           operator==( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           operator!=( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           operator<=( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           operator>=( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           operator<( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           operator>( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template Graph<Id>      operator+( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template Graph<Id>      operator-( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           is_complete( const Graph<Id>& graph );
        template bool           is_empty( const Graph<Id>& graph );
        template bool           is_trivial( const Graph<Id>& graph );
        template bool           is_null( const Graph<Id>& graph );
        template bool           is_child_node( const Graph<Id>& graph, const Id node_1 );
        template bool           is_subgraph( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           is_proper_subgraph( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           is_improper_subgraph( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           is_spanning_subgraph( const Graph<Id>& lhs, const Graph<Id>& rhs );
        template bool           are_adjacent_nodes( const Graph<Id>& graph, const Id node_1, const Id node_2 );
        template bool           are_adjacent_edges( const Graph<Id>& graph, const Id node_1, const Id node_2, const Id node_3 );
        template size_t         order( const Graph<Id>& graph );
        template size_t         size( const Graph<Id>& graph );
        template void           generate_dot_graph( const Graph<Id>& graph, const std::string& filename );

        template class Graph<std::string>;
        template std::ostream&      operator<<( std::ostream& os, const Graph<std::string>& graph );
        template bool               operator==( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               operator!=( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               operator<=( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               operator>=( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               operator<( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               operator>( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template Graph<std::string> operator+( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template Graph<std::string> operator-( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               is_complete( const Graph<std::string>& graph );
        template bool               is_empty( const Graph<std::string>& graph );
        template bool               is_trivial( const Graph<std::string>& graph );
        template bool               is_null( const Graph<std::string>& graph );
        template bool               is_child_node( const Graph<std::string>& graph, std::string const node_1 );
        template bool               is_subgraph( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               is_proper_subgraph( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               is_improper_subgraph( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               is_spanning_subgraph( const Graph<std::string>& lhs, const Graph<std::string>& rhs );
        template bool               are_adjacent_nodes( const Graph<std::string>& graph, const std::string node_1, const std::string node_2 );
        template bool               are_adjacent_edges( const Graph<std::string>& graph, const std::string node_1, const std::string node_2, const std::string node_3 );
        template size_t             order( const Graph<std::string>& graph );
        template size_t             size( const Graph<std::string>& graph );
        template void               generate_dot_graph( const Graph<std::string>& graph, const std::string& filename );

    } // namespace graph

} // namespace atom