use thiserror::Error;

use crate::graph;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: {0}")]
    GraphError(graph::error::Error),
    #[error("Function Graph Error: Execution Error")]
    ExecutionError,
}