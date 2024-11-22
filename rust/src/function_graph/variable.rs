use std::{
    any::Any, collections::HashMap, hash::Hash, ops::{ Deref, DerefMut }, sync::{ Arc, RwLock, RwLockReadGuard, RwLockWriteGuard }
};

type Inner = dyn Any + Send + Sync;

#[derive(Clone)]
pub struct Variable( Arc<RwLock<Inner>> );

impl Variable {
    pub fn new<T>( value: T ) -> Self
    where
        T: 'static + Send + Sync
    {
        Self ( Arc::new( RwLock::new( value ) ) )
    }

    pub fn empty() -> Self {
        Self ( Arc::new( RwLock::new( () ) ) )
    }

    #[inline]
    pub fn read( &self ) -> RwLockReadGuard<Inner> {
        self.0.read().expect( "Failed to acquire read lock" )
    }

    #[inline]
    pub fn write( &self ) -> RwLockWriteGuard<Inner> {
        self.0.write().expect( "Failed to acquire write lock" )
    }
}

impl Deref for Variable {
    type Target = Arc<RwLock<Inner>>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Variable {
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Variable
where
    Self: 'static
{
    fn eq( &self, other: &Self ) -> bool {
        self.read().deref().type_id() == other.read().deref().type_id()
    }
}

#[derive(Clone)]
pub struct Variables<I>( HashMap<I, Variable> );

impl<I> Variables<I>
where
    I: Clone + PartialEq + Ord + Hash + 'static
{
    pub fn new<const N: usize>( variables: [ ( I, Variable ); N ] ) -> Self {
        Self( HashMap::from( variables ) )
    }

    #[inline]
    fn get( &self, key: &I ) -> &Variable {
        self.0.get( key ).expect( "Variable not found" )
    }

    #[inline]
    pub fn read( &self, key: &I ) -> RwLockReadGuard<Inner> {
        self.get( key ).read()
    }

    #[inline]
    pub fn write( &self, key: &I ) -> RwLockWriteGuard<Inner> {
        self.get( key ).write()
    }
}

impl<I> Deref for Variables<I>
where
    I: Clone + PartialEq + Ord + Hash + 'static
{
    type Target = HashMap<I, Variable>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for Variables<I>
where
    I: Clone + PartialEq + Ord + Hash + 'static
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}
