// Copyright 2024 Bewusstsein Labs

//: Standard
use std::{
    collections::{BTreeSet, VecDeque},
    fmt::Display,
    sync::{ Arc, Mutex }
};

use thiserror::Error;

use crate::graph::{
    Error as GraphError,
    Graph,
    GraphType,
    GraphTraits
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: {0}")]
    GraphError(GraphError),
    #[error("Function Graph Error: Execution Error")]
    ExecutionError,
}

impl From<GraphError> for Error {
    fn from(error: GraphError) -> Self {
        Error::GraphError(error)
    }
}

pub trait FunctionNodeTraits<In, Out>
where
    In: Clone + PartialEq + Ord,
    Out: Clone + PartialEq + Ord
{
    fn execute(&self) -> Result<(), Error>;
    fn execute_mut(&mut self) -> Result<(), Error>;
}

type Func<In, Out> = dyn Fn(&In, &mut Out) + Send + Sync;

#[derive(Clone)]
pub struct FunctionNode<In, Out> {
    input: Arc<Mutex<In>>,
    function: Arc<Mutex<Box<Func<In, Out>>>>,
    output: Arc<Mutex<Out>>,
}

impl<In, Out> FunctionNode<In, Out>
where
    In: Clone + PartialEq + Ord,
    Out: Clone + PartialEq + Ord
{
    pub fn new(
        input: Arc<Mutex<In>>,
        function: Box<Func<In, Out>>,
        output: Arc<Mutex<Out>>,
    ) -> Self {
        Self {
            input,
            function: Arc::new(Mutex::new(function)),
            output,
        }
    }
}

impl<In, Out> PartialEq for FunctionNode<In, Out>
where
In: Clone + PartialEq + Ord,
Out: Clone + PartialEq + Ord
{
    fn eq(&self, other: &Self) -> bool {
        let self_input = self.input.lock().unwrap();
        let other_input = other.input.lock().unwrap();
        let self_output = self.output.lock().unwrap();
        let other_output = other.output.lock().unwrap();
        *self_input == *other_input && *self_output == *other_output
    }
}

impl<In, Out> Eq for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{}

impl<In, Out> FunctionNodeTraits<In, Out> for FunctionNode<In, Out>
where
    In: Clone + PartialEq + Ord ,
    Out: Clone + PartialEq + Ord
{
    fn execute(&self) -> Result<(), Error> {
        let function = self.function.lock().map_err(|_| Error::ExecutionError)?;
        let input = self.input.lock().map_err(|_| Error::ExecutionError)?;
        let mut output = self.output.lock().map_err(|_| Error::ExecutionError)?;
        function(&input, &mut output);
        Ok(())
    }

    fn execute_mut(&mut self) -> Result<(), Error> {
        let function = self.function.lock().map_err(|_| Error::ExecutionError)?;
        let input = self.input.lock().map_err(|_| Error::ExecutionError)?;
        let mut output = self.output.lock().map_err(|_| Error::ExecutionError)?;
        function(&input, &mut output);
        Ok(())
    }
}

pub struct Function();
impl GraphType for Function {}

impl<'a, I, In, Out> Graph<Function, I, FunctionNode<In, Out>, ()>
where
    I: 'a + Clone + PartialEq + Ord + Display,
    In: 'a + Clone + PartialEq + Ord + Display,
    Out: 'a + Clone + PartialEq + Ord + Display
{
    pub fn generate_dot_to_file(&self, file_name: String) {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");
        for (node_id, node_data) in self.get_nodes().iter() {
            let input = node_data.data.input.lock().unwrap();
            let output = node_data.data.output.lock().unwrap();
            dot.push_str(&format!(
                " {} [label=\"Node {}\\nInput: {}\\nOutput: {}\"];\n",
                node_id, node_id, *input, *output
            ));
            for adj_node_id in node_data.adjacencies.keys() {
                dot.push_str(&format!(" {} -> {};\n", node_id, adj_node_id));
            }
        }
        dot.push_str("}\n");
        std::fs::write(file_name, dot).unwrap();
    }
}

impl<'a, I, In, Out> GraphTraits<'a, I, FunctionNode<In, Out>, ()> for Graph<Function, I, FunctionNode<In, Out>, ()>
where
    I: 'a + Clone + PartialEq + Ord,
    In: 'a + Clone + PartialEq + Ord,
    Out: 'a + Clone + PartialEq + Ord
{
    fn bfs(&self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while let Some(current_id) = queue.pop_front() {
            if !visited.contains(&current_id) {
                visited.insert(current_id.clone());
                if let Some(node) = self.data.get_node(current_id.clone()) {
                    node.execute().unwrap();
                }
                if let Some(adjacencies) = self.data.get_edges(current_id) {
                    for next_id in adjacencies.keys() {
                        if !visited.contains(next_id) {
                            queue.push_back(next_id.clone());
                        }
                    }
                }
            }
        }
    }

    fn dfs(&self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while let Some(current_id) = stack.pop() {
            if !visited.contains(&current_id) {
                visited.insert(current_id.clone());
                if let Some(node) = self.data.get_node(current_id.clone()) {
                    node.execute().unwrap();
                }
                if let Some(adjacencies) = self.data.get_edges(current_id) {
                    for next_id in adjacencies.keys() {
                        if !visited.contains(next_id) {
                            stack.push(next_id.clone());
                        }
                    }
                }
            }
        }
    }

    fn bfs_mut(&mut self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while let Some(current_id) = queue.pop_front() {
            if !visited.contains(&current_id) {
                visited.insert(current_id.clone());
                if let Some(node) = self.data.get_node_mut(current_id.clone()) {
                    node.execute_mut().unwrap();
                }
                if let Some(adjacencies) = self.data.get_edges(current_id) {
                    for next_id in adjacencies.keys() {
                        if !visited.contains(next_id) {
                            queue.push_back(next_id.clone());
                        }
                    }
                }
            }
        }
    }

    fn dfs_mut(&mut self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while let Some(current_id) = stack.pop() {
            if !visited.contains(&current_id) {
                visited.insert(current_id.clone());
                if let Some(node) = self.data.get_node_mut(current_id.clone()) {
                    node.execute_mut().unwrap();
                }
                if let Some(adjacencies) = self.data.get_edges(current_id) {
                    for next_id in adjacencies.keys() {
                        if !visited.contains(next_id) {
                            stack.push(next_id.clone());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{ Arc, Mutex };
    use crate::{
        graph::{
            Graph,
            GraphTraits,
        },
        function_graph::{
            Function,
            FunctionNode,
        }
    };

    #[test]
    fn test_matrix_vector_multiplication_graph() {

    }
}
