mod test;

//: Standard
use std::collections::{ HashSet, HashMap, VecDeque };
use std::hash::Hash;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    NodeNotFound,
    NodeAlreadyExists,
    AdjacencyNotFound,
    AdjacencyAlreadyExists,
}

trait GraphDataTraits<Id, Node, Edge>
where
    Id: Eq + Hash + Display,
    Node: Display,
    Edge: Display,
{
    fn new() -> Self;
    fn add_node(&mut self, node: Id, data: Node) -> Result<(), Error>;
    fn remove_node(&mut self, node: Id) -> Result<(), Error>;
    fn add_adjacency(&mut self, node1: Id, node2: Id, data: Edge) -> Result<(), Error>;
    fn remove_adjacency(&mut self, node1: Id, node2: Id) -> Result<(), Error>;
    fn dfs_step(&mut self, stack: &mut Vec<Id>, visited: &mut HashSet<Id>) -> Vec<(Id, Id)>;
    fn bfs_step(&mut self, queue: &mut VecDeque<Id>, visited: &mut HashSet<Id>) -> Vec<(Id, Id)>;
}

type GraphData<Id, Node, Edge> = HashMap<Id, (HashMap<Id, Edge>, Node)>;

impl<Id, Node, Edge> GraphDataTraits<Id, Node, Edge> for GraphData<Id, Node, Edge>
where
    Id: Clone + Eq + Hash + Display,
    Node: Display,
    Edge: Display,
{
    fn new() -> Self {
        HashMap::new()
    }

    fn add_node(&mut self, node: Id, data: Node) -> Result<(), Error> {
        if self.contains_key(&node) {
            return Err(Error::NodeAlreadyExists);
        }
        self.insert(node, (HashMap::new(), data));
        Ok(())
    }

    fn remove_node(&mut self, node: Id) -> Result<(), Error> {
        if self.remove(&node).is_none() {
            return Err(Error::NodeNotFound);
        }
        Ok(())
    }

    fn add_adjacency(&mut self, node1: Id, node2: Id, data: Edge) -> Result<(), Error> {
        if let Some(node) = self.get_mut(&node1) {
            if node.0.contains_key(&node2) {
                return Err(Error::AdjacencyAlreadyExists);
            }
            node.0.insert(node2, data);
            Ok(())
        } else {
            Err(Error::NodeNotFound)
        }
    }

    fn remove_adjacency(&mut self, node1: Id, node2: Id) -> Result<(), Error> {
        if let Some(node) = self.get_mut(&node1) {
            if node.0.remove(&node2).is_none() {
                return Err(Error::AdjacencyNotFound);
            }
            Ok(())
        } else {
            Err(Error::NodeNotFound)
        }
    }

    fn dfs_step(&mut self, stack: &mut Vec<Id>, visited: &mut HashSet<Id>) -> Vec<(Id, Id)> {
        let mut edges = Vec::new();
        while let Some(current) = stack.pop() {
            if !visited.contains(&current) {
                visited.insert(current.clone());
                if let Some((adjacencies, _)) = self.get(&current) {
                    for next in adjacencies.keys() {
                        if !visited.contains(next) {
                            stack.push(next.clone());
                            edges.push((current.clone(), next.clone()));
                        }
                    }
                }
            }
        }
        edges
    }

    fn bfs_step(&mut self, queue: &mut VecDeque<Id>, visited: &mut HashSet<Id>) -> Vec<(Id, Id)> {
        let mut edges = Vec::new();
        while let Some(current) = queue.pop_front() {
            if !visited.contains(&current) {
                visited.insert(current.clone());
                if let Some((adjacencies, _)) = self.get(&current) {
                    for next in adjacencies.keys() {
                        if !visited.contains(next) {
                            queue.push_back(next.clone());
                            edges.push((current.clone(), next.clone()));
                        }
                    }
                }
            }
        }
        edges
    }
}

pub trait GraphTraits<Id, Node, Edge>
where
    Id: Clone + Eq + Hash + Display,
    Node: Display,
    Edge: Display,
{
    fn new() -> Self;
    fn add_node(&mut self, node: Id, data: Node) -> Result<(), Error>;
    fn remove_node(&mut self, node: Id) -> Result<(), Error>;
    fn add_adjacency(&mut self, node1: Id, node2: Id, data: Edge) -> Result<(), Error>;
    fn remove_adjacency(&mut self, node1: Id, node2: Id) -> Result<(), Error>;
    fn bfs(&mut self, start: Id);
    fn dfs(&mut self, start: Id);
    fn generate_dot_to_file(&self, file_name: String);
}

pub struct UndirectedGraph<Id, Node, Edge> {
    data: GraphData<Id, Node, Edge>,
}

impl<Id, Node, Edge> GraphTraits<Id, Node, Edge> for UndirectedGraph<Id, Node, Edge>
where
    Id: Clone + Eq + Hash + Display,
    Node: Display,
    Edge: Display,
{
    fn new() -> Self {
        Self {
            data: GraphData::new(),
        }
    }

    fn add_node(&mut self, node: Id, data: Node) -> Result<(), Error> {
        self.data.add_node(node, data)?;
        Ok( () )
    }

    fn remove_node(&mut self, node: Id) -> Result<(), Error> {
        self.data.remove_node(node)?;
        Ok( () )
    }

    fn add_adjacency(&mut self, node1: Id, node2: Id, data: Edge) -> Result<(), Error> {
        self.data.add_adjacency(node1, node2, data)?;
        Ok( () )
    }

    fn remove_adjacency(&mut self, node1: Id, node2: Id) -> Result<(), Error> {
        self.data.remove_adjacency(node1, node2)?;
        Ok( () )
    }

    fn bfs(&mut self, start: Id) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start.clone());
        visited.insert(start.clone());

        while !queue.is_empty() {
            let edges = self.data.bfs_step(&mut queue, &mut visited);
            for edge in edges {
                println!("[ {}, {} ]", edge.0, edge.1);
            }
        }
    }

    fn dfs(&mut self, start: Id) {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start.clone());
        visited.insert(start.clone());

        while !stack.is_empty() {
            let edges = self.data.dfs_step(&mut stack, &mut visited);
            for edge in edges {
                println!("[ {}, {} ]", edge.0, edge.1);
            }
        }
    }

    fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node1, (adjacencies, data)) in &self.data {
            dot.push_str(&format!(" {} [label=\"{}\"];\n", node1, data));
            for (node2, data) in adjacencies {
                dot.push_str(&format!(" {} -> {} [label=\"{}\"];\n", node1, node2, data));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}

pub struct DirectedGraph<Id, Node, Edge> {
    data: GraphData<Id, Node, Edge>,
}

impl<Id, Node, Edge> GraphTraits<Id, Node, Edge> for DirectedGraph<Id, Node, Edge>
where
    Id: Clone + Eq + Hash + Display,
    Node: Display,
    Edge: Display,
{
    fn new() -> Self {
        Self {
            data: GraphData::new(),
        }
    }

    fn add_node(&mut self, node: Id, data: Node) -> Result<(), Error> {
        self.data.add_node(node, data)?;
        Ok( () )
    }

    fn remove_node(&mut self, node: Id) -> Result<(), Error> {
        self.data.remove_node(node)?;
        Ok( () )
    }

    fn add_adjacency(&mut self, node1: Id, node2: Id, data: Edge) -> Result<(), Error> {
        if self.data.get(&node2).map_or(false, |reverse_node| reverse_node.0.contains_key(&node1)) {
            return Err(Error::AdjacencyAlreadyExists);
        }
        self.data.add_adjacency(node1, node2, data)?;
        Ok( () )
    }

    fn remove_adjacency(&mut self, node1: Id, node2: Id) -> Result<(), Error> {
        self.data.remove_adjacency(node1, node2)?;
        Ok( () )
    }

    fn bfs(&mut self, start: Id) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start.clone());
        visited.insert(start.clone());

        while !queue.is_empty() {
            let edges = self.data.bfs_step(&mut queue, &mut visited);
            for edge in edges {
                println!("[ {}, {} ]", edge.0, edge.1);
            }
        }
    }

    fn dfs(&mut self, start: Id) {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start.clone());
        visited.insert(start.clone());

        while !stack.is_empty() {
            let edges = self.data.dfs_step(&mut stack, &mut visited);
            for edge in edges {
                println!("[ {}, {} ]", edge.0, edge.1);
            }
        }
    }

    fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node1, (adjacencies, data)) in &self.data {
            dot.push_str(&format!(" {} [label=\"{}\"];\n", node1, data));
            for (node2, data) in adjacencies {
                dot.push_str(&format!(" {} -> {} [label=\"{}\"];\n", node1, node2, data));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}