use thiserror::Error;

pub trait GraphErrorTraits: std::fmt::Debug + std::fmt::Display {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graph Error: Node Not Found")]
    NodeNotFound,
    #[error("Graph Error: Node Already Exists")]
    NodeAlreadyExists,
    #[error("Graph Error: Edge Not Found")]
    EdgeNotFound,
    #[error("Graph Error: Edge Already Exists")]
    EdgeAlreadyExists,
}

impl GraphErrorTraits for Error {}