#[derive(Debug)]
pub enum Error {
    NodeNotFound,
    NodeAlreadyExists,
    EdgeNotFound,
    EdgeAlreadyExists,
}