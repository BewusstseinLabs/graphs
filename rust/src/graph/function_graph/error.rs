// Copyright 2024 Bewusstsein Labs

use thiserror::Error;

use crate::graph::graph_data;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: {0}")]
    GraphError(graph_data::error::Error),
    #[error("Function Graph Error: Execution Error")]
    ExecutionError,
}

impl graph_data::error::GraphErrorTraits for Error {}

impl From<graph_data::error::Error> for Error {
    fn from(error: graph_data::error::Error) -> Self {
        Error::GraphError(error)
    }
}