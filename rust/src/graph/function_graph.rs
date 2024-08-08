pub mod error;

use self::error::Error;

use crate::graph::{
    GraphTraits,
    graph_data::{ GraphData, GraphDataTraits, error::Error as GraphDataError }
};

//: Standard
use std::collections::{ BTreeSet, BTreeMap, VecDeque };
use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub trait FunctionNodeTraits<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn execute( &mut self ) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct FunctionNode<In, Out> {
    input: Arc<Mutex<In>>,
    function: Arc<Mutex<Box<dyn Fn(&In, &mut Out) + Send + Sync>>>,
    output: Arc<Mutex<Out>>,
}

impl<In, Out> FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    pub fn new(input: Arc<Mutex<In>>, function: Box<dyn Fn(&In, &mut Out) + Send + Sync>, output: Arc<Mutex<Out>>) -> Self {
        Self {
            input: input,
            function: Arc::new(Mutex::new(function)),
            output: output,
        }
    }
}

impl<In, Out> PartialOrd for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_input = self.input.lock().unwrap();
        let other_input = other.input.lock().unwrap();
        let self_output = self.output.lock().unwrap();
        let other_output = other.output.lock().unwrap();

        if *self_input == *other_input && *self_output == *other_output {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl<In, Out> Ord for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_input = self.input.lock().unwrap();
        let other_input = other.input.lock().unwrap();
        let self_output = self.output.lock().unwrap();
        let other_output = other.output.lock().unwrap();

        if *self_input == *other_input && *self_output == *other_output {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl<In, Out> Eq for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
}

impl<In, Out> PartialEq for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn eq(&self, other: &Self) -> bool {
        let self_input = self.input.lock().unwrap();
        let other_input = other.input.lock().unwrap();
        let self_output = self.output.lock().unwrap();
        let other_output = other.output.lock().unwrap();

        *self_input == *other_input && *self_output == *other_output
    }
}

impl<In, Out> Display for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input: {}, Output: {}", self.input.lock().unwrap(), self.output.lock().unwrap())
    }
}

impl<In, Out> FunctionNodeTraits<In, Out> for FunctionNode<In, Out>
where
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn execute(&mut self) -> Result<(), Error> {
        match self.function.lock() {
            Ok(function) => {
                let input = self.input.lock().unwrap();
                let mut output = self.output.lock().unwrap();
                function(&input, &mut output);
            }
            Err(_) => return Err(Error::ExecutionError),
        };
        Ok(())
    }
}

pub struct FunctionGraph<I, In, Out> {
    data: GraphData<I, FunctionNode<In, Out>, ()>,
}

impl<I, In, Out> GraphTraits<I, FunctionNode<In, Out>, (), Error> for FunctionGraph<I, In, Out>
where
    I: Clone + Ord + PartialEq + Display,
    In: Clone + Ord + PartialEq + Display,
    Out: Clone + Ord + PartialEq + Display,
{
    fn new() -> Self {
        Self {
            data: GraphData::<I, FunctionNode<In, Out>, ()>::new(),
        }
    }

    fn add_node(&mut self, node: I, data: FunctionNode<In, Out>) -> Result<(), Error> {
        self.data.add_node(node, data).map_err(|e| e.into())
    }

    fn get_node(&self, node: I) -> Option<&FunctionNode<In, Out>> {
        self.data.get_node(node)
    }

    fn get_node_mut(&mut self, node: I) -> Option<&mut FunctionNode<In, Out>> {
        self.data.get_node_mut(node)
    }

    fn contains_node(&self, node: I) -> bool {
        self.data.contains_node(node)
    }

    fn remove_node(&mut self, node: I) -> Result<FunctionNode<In, Out>, Error> {
        self.data.remove_node(node).map_err(|e| e.into())
    }

    fn delete_node(&mut self, node: I) -> Result<(), Error> {
        self.data.delete_node(node).map_err(|e| e.into())
    }

    fn add_edge(&mut self, node1: I, node2: I, data: ()) -> Result<(), Error> {
        self.data.add_directed_edge(node1, node2, data).map_err(|e| e.into())
    }

    fn get_edge(&self, node1: I, node2: I) -> Option<&()> {
        self.data.get_edge(node1, node2)
    }

    fn get_edge_mut(&mut self, node1: I, node2: I) -> Option<&mut ()> {
        self.data.get_edge_mut(node1, node2)
    }

    fn contains_edge(&self, node1: I, node2: I) -> bool {
        self.data.contains_edge(node1, node2)
    }

    fn remove_edge(&mut self, node1: I, node2: I) -> Result<(), Error> {
        self.data.remove_edge(node1, node2).map_err(|e| e.into())
    }

    fn delete_edge(&mut self, node1: I, node2: I) -> Result<(), Error> {
        self.data.delete_edge(node1, node2).map_err(|e| e.into())
    }

    fn clear( &mut self ) {
        self.data.clear();
    }

    fn clear_edges( &mut self ) {
        self.data.clear_edges();
    }

    fn bfs(&mut self, start: I) {
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        queue.push_back(start.clone());
        while !queue.is_empty() {
            if let Some( current_id ) = self.data.bfs_step(&mut queue, &mut visited) {
                self.data.get_node_mut( current_id.clone() ).unwrap().execute().unwrap();
                println!("Visited: {}", current_id);
            }
        }
    }

    fn dfs(&mut self, start: I) {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();
        stack.push(start.clone());
        while !stack.is_empty() {
            if let Some( current_id ) = self.data.dfs_step(&mut stack, &mut visited) {
                self.data.get_node_mut( current_id.clone() ).unwrap().execute().unwrap();
                println!("Visited: {}", current_id);
            }
        }
    }

    fn is_complete( graph: &Self ) -> bool {
        GraphDataTraits::is_complete(&graph.data)
    }

    fn is_empty( graph: &Self ) -> bool {
        GraphDataTraits::is_empty(&graph.data)
    }

    fn is_trivial( graph: &Self ) -> bool {
        GraphDataTraits::is_trivial(&graph.data)
    }

    fn is_null( graph: &Self ) -> bool {
        GraphDataTraits::is_null(&graph.data)
    }

    fn is_child_node( graph: &Self, node_1: I ) -> bool {
        GraphDataTraits::is_child_node(&graph.data, node_1)
    }

    fn is_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_subgraph(&graph.data, &subgraph.data)
    }

    fn is_proper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_proper_subgraph(&graph.data, &subgraph.data)
    }

    fn is_improper_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_improper_subgraph(&graph.data, &subgraph.data)
    }

    fn is_spanning_subgraph( graph: &Self, subgraph: &Self ) -> bool {
        GraphDataTraits::is_spanning_subgraph(&graph.data, &subgraph.data)
    }

    fn are_adjacent_nodes( graph: &Self, node_1: I, node_2: I ) -> bool {
        GraphDataTraits::are_adjacent_nodes(&graph.data, node_1, node_2)
    }

    fn are_adjacent_edges( graph: &Self, node_1: I, node_2: I, node_3: I ) -> bool {
        GraphDataTraits::are_adjacent_edges(&graph.data, node_1, node_2, node_3)
    }

    fn order( graph: &Self ) -> usize {
        GraphDataTraits::order(&graph.data)
    }

    fn size( graph: &Self ) -> usize {
        GraphDataTraits::size(&graph.data)
    }
}