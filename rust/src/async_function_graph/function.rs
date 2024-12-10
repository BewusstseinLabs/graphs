use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    pin::Pin,
    future::Future,
};
use crate::function_graph::variable::Variables;

pub trait AsyncFn<I>: Send + Sync {
    fn call<'a>(&'a self, vars: &'a Variables<I>) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

impl<I, T> AsyncFn<I> for T
where
    T: for<'a> Fn(&'a Variables<I>) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Send
        + Sync
        + 'static,
{
    fn call<'a>(
        &'a self,
        vars: &'a Variables<I>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        (self)(vars)
    }
}

#[derive(Clone)]
pub struct AsyncFunction<I>(Arc<dyn AsyncFn<I> + 'static>);

impl<I> AsyncFunction<I> {
    pub fn new<T>(function: T) -> Self
    where
        T: for<'a> Fn(&'a Variables<I>) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        Self(Arc::new(function))
    }

    // Provide a method to call the function.
    pub fn call<'a>(
        &'a self,
        vars: &'a Variables<I>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        self.0.call( vars )
    }
}

impl<I> Deref for AsyncFunction<I> {
    type Target = Arc<dyn AsyncFn<I> + 'static>;
    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for AsyncFunction<I> {
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I> std::fmt::Debug for AsyncFunction<I> {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!( f, "Function" )
    }
}

async fn async_test_function( vars: &Variables<()> ) {
    println!( "Hello, world!" );
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_test() {
        use crate::async_function_graph::function::AsyncFunction;
        use crate::function_graph::variable::Variables;
        let function = AsyncFunction::new(|vars| Box::pin(async_test_function(vars)));
    }
}
