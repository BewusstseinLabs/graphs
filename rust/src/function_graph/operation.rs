use std::{
    any::Any,
    hash::Hash,
    ops::Deref
};

use thiserror::Error;

use crate::function_graph::{
    variable::{ Variable, Variables },
    function::Function
};

#[derive(Error, Debug)]
pub enum Error {}

#[derive( Debug )]
pub struct Operation<I> {
    variables: Variables<I>,
    function: Function<I>
}

impl<'a, I> Operation<I>
where
    I: 'a + Ord + Hash
{
    pub fn new<const N: usize, F>( variables: [ ( I, Variable ); N ], function: F ) -> Self
    where
        F: 'static + Fn( &Variables<I> ) + Send + Sync
    {
        Self {
            variables: Variables::new( variables ),
            function: Function::new( function )
        }
    }

    pub fn variables( &self ) -> &Variables<I> {
        &self.variables
    }

    pub fn variables_mut( &mut self ) -> &mut Variables<I> {
        &mut self.variables
    }

    pub fn function( &self ) -> &Function<I> {
        &self.function
    }

    pub fn function_mut( &mut self ) -> &mut Function<I> {
        &mut self.function
    }

    pub fn execute( &self ) -> Result<(), Error> {
        ( self.function )( &self.variables );
        Ok( () )
    }

    pub fn execute_mut( &mut self ) -> Result<(), Error> {
        ( self.function )( &mut self.variables );
        Ok( () )
    }
}

impl<I> PartialEq for Operation<I>
where
    I: 'static + Ord + Hash
{
    fn eq( &self, other: &Self ) -> bool {
        self.function().deref().as_ref().as_ref().type_id() == other.function().deref().as_ref().as_ref().type_id()
    }
}