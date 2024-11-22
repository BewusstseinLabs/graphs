use std::{
    ops::{ Deref, DerefMut },
    sync::Arc,
};

use crate::function_graph::variable::Variables;

type Inner<I> = dyn Fn( &Variables<I> ) + Send + Sync;

#[derive(Clone)]
pub struct Function<I>( Arc<Box<Inner<I>>> );

impl<I> Function<I> {
    pub fn new<T>( function: T ) -> Self
    where
    T: Fn( &Variables<I> ) + Send + Sync + 'static,
    {
        Self( Arc::new( Box::new( function ) ) )
    }
}

impl<I> Deref for Function<I> {
    type Target = Arc<Box<dyn Fn( &Variables<I> ) + Send + Sync>>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for Function<I> {
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}