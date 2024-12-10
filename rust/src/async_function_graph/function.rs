use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    pin::Pin,
    future::Future,
};
use crate::function_graph::variable::Variables;

type Inner<I> = dyn Fn(&Variables<I>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync;

#[derive(Clone)]
pub struct AsyncFunction<I>( Arc<Box<Inner<I>>> );

impl<I> AsyncFunction<I> {
    pub fn new<T, F>(function: T) -> Self
    where
        T: 'static + Fn(&Variables<I>) -> F + Send + Sync,
        F: Future<Output = ()> + Send + 'static,
    {
        Self( Arc::new( Box::new( move |vars| Box::pin( function( vars ) ) ) ) )
    }
}

impl<I> Deref for AsyncFunction<I> {
    type Target = Arc<Box<Inner<I>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for AsyncFunction<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I> std::fmt::Debug for AsyncFunction<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}