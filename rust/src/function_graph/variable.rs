use std::{
    any::Any, collections::HashMap, hash::Hash, ops::{ Deref, DerefMut }, sync::{ Arc, RwLock, RwLockReadGuard, RwLockWriteGuard }
};

#[cfg(feature = "unstable")]
use std::sync::{ MappedRwLockReadGuard, MappedRwLockWriteGuard };

type Inner = dyn Any + Send + Sync;

pub enum Variable {
    Owned( Box<RwLock<Inner>> ),
    Shared( Arc<RwLock<Inner>> )
}

impl Variable {
    pub fn owned<T>( value: T ) -> Self
    where
        T: Any + Send + Sync
    {
        Self::Owned( Box::new( RwLock::new( value ) ) )
    }

    pub fn shared<T>( value: T ) -> Self
    where
        T: Any + Send + Sync
    {
        Self::Shared( Arc::new( RwLock::new( value ) ) )
    }

    #[inline(always)]
    pub fn read( &self ) -> RwLockReadGuard<Inner> {
        match &self {
            Self::Owned( value ) => value.read().expect( "Failed to acquire read lock" ),
            Self::Shared( value ) => value.read().expect( "Failed to acquire read lock" )
        }
    }

    #[inline(always)]
    pub fn write( &self ) -> RwLockWriteGuard<Inner> {
        match &self {
            Self::Owned( value ) => value.write().expect( "Failed to acquire write lock" ),
            Self::Shared( value ) => value.write().expect( "Failed to acquire write lock" )
        }
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub fn read_as<T>(&self) -> Option<MappedRwLockReadGuard<T>>
    where
        T: 'static + Send + Sync,
    {
        let guard = self.read();
        RwLockReadGuard::try_map( guard, |inner| {
            inner.downcast_ref::<T>()
        }).ok()
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub fn write_as<T>(&self) -> Option<MappedRwLockWriteGuard<T>>
    where
        T: 'static + Send + Sync,
    {
        let guard = self.write();
        RwLockWriteGuard::try_map( guard, |inner| {
            inner.downcast_mut::<T>()
        }).ok()
    }

    fn type_id( &self ) -> std::any::TypeId {
        self.read().deref().type_id()
    }
}

impl std::fmt::Debug for Variable {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        let type_id = self.type_id();
        match self {
            Self::Owned( value ) => {
                write!( f, "Owned(TypeId: {:?}, Location: {:p})", type_id, &**value )
            },
            Self::Shared( value ) => {
                write!( f, "Shared(TypeId: {:?}, Location: {:p})", type_id, &**value )
            }
        }
    }
}

impl From <Arc<RwLock<Inner>>> for Variable {
    fn from( value: Arc<RwLock<Inner>> ) -> Self {
        Self::Shared( value )
    }
}

impl From <Box<RwLock<Inner>>> for Variable {
    fn from( value: Box<RwLock<Inner>> ) -> Self {
        Self::Owned( value )
    }
}

impl PartialEq for Variable {
    fn eq( &self, other: &Self ) -> bool {
        self.type_id() == other.type_id()
    }
}

impl Clone for Variable {
    fn clone( &self ) -> Self {
        match self {
            Self::Owned( _ ) => Self::Owned( Box::new( RwLock::new( () ) ) ),
            Self::Shared( value ) => Self::Shared( value.clone() )
        }
    }
}

#[derive( Debug )]
pub struct Variables<I>( HashMap<I, Variable> );

impl<I> Variables<I>
where
    I: Ord + Hash
{
    pub fn new<const N: usize>( variables: [ ( I, Variable ); N ] ) -> Self {
        Self( HashMap::from( variables ) )
    }

    #[inline(always)]
    fn get( &self, key: &I ) -> &Variable {
        self.0.get( key ).expect( "Variable not found" )
    }

    #[inline(always)]
    pub fn read( &self, key: &I ) -> RwLockReadGuard<Inner> {
        self.get( key ).read()
    }

    #[inline(always)]
    pub fn write( &self, key: &I ) -> RwLockWriteGuard<Inner> {
        self.get( key ).write()
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub fn read_as<T>( &self, key: &I ) -> Option<MappedRwLockReadGuard<T>>
    where
        T: 'static + Send + Sync
    {
        self.get( key ).read_as::<T>()
    }

    #[cfg(feature = "unstable")]
    #[inline(always)]
    pub fn write_as<T>( &self, key: &I ) -> Option<MappedRwLockWriteGuard<T>>
    where
        T: 'static + Send + Sync
    {
        self.get( key ).write_as::<T>()
    }
}

impl<I> Deref for Variables<I> {
    type Target = HashMap<I, Variable>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<I> DerefMut for Variables<I> {
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_read() {
        let variable = Variable::owned( 42 );
        let guard = variable.read();
        assert_eq!( *guard.downcast_ref::<i32>().expect( "Failed to downcast" ), 42 );
    }

    #[test]
    fn test_variable_write() {
        let variable = Variable::owned( 42 );
        let mut guard = variable.write();
        *guard.downcast_mut::<i32>().expect( "Failed to downcast" ) = 43;
        assert_eq!( *guard.downcast_ref::<i32>().expect( "Failed to downcast" ), 43 );
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_variable_read_as() {
        let variable = Variable::owned( 42 );
        let guard = variable.read_as::<i32>().expect( "Failed to downcast" );
        assert_eq!( *guard, 42 );
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_variable_write_as() {
        let variable = Variable::owned( 42 );
        let mut guard = variable.write_as::<i32>().expect( "Failed to downcast" );
        *guard = 43;
        assert_eq!( *guard, 43 );
    }
}
