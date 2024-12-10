use std::{
    any::Any,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
    pin::Pin,
    future::Future,
};
use thiserror::Error;
use crate::{
    function_graph::variable::{Variable, Variables},
    async_function_graph::function::AsyncFunction,
};

#[derive(Error, Debug)]
pub enum Error {}

#[derive(Debug)]
pub struct AsyncOperation<I> {
    variables: Variables<I>,
    function: AsyncFunction<I>,
}

impl<'a, I> AsyncOperation<I>
where
    I: 'a + Ord + Hash,
{
    pub fn new<const N: usize, F>(variables: [(I, Variable); N], function: F) -> Self
    where
        F: for<'b> Fn(&'b Variables<I>) -> Pin<Box<dyn Future<Output = ()> + Send + 'b>>
        + Send
        + Sync
        + 'static,
    {
        Self {
            variables: Variables::new( variables ),
            function: AsyncFunction::new( function ),
        }
    }

    pub fn variables( &self ) -> &Variables<I> {
        &self.variables
    }

    pub fn variables_mut( &mut self ) -> &mut Variables<I> {
        &mut self.variables
    }

    pub fn function( &self ) -> &AsyncFunction<I> {
        &self.function
    }

    pub fn function_mut( &mut self ) -> &mut AsyncFunction<I> {
        &mut self.function
    }

    pub async fn execute( &self ) -> Result<(), Error> {
        self.function.call( &self.variables ).await;
        Ok( () )
    }

    pub async fn execute_mut( &mut self ) -> Result<(), Error> {
        self.function.call( &mut self.variables ).await;
        Ok( () )
    }
}

impl<I> PartialEq for AsyncOperation<I>
where
    I: 'static + Ord + Hash,
{
    fn eq( &self, other: &Self ) -> bool {
        self.function().deref().as_ref().type_id() == other.function().deref().as_ref().type_id()
    }
}