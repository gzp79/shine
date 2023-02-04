use crossbeam::sync::{ShardedLock, ShardedLockReadGuard, ShardedLockWriteGuard};
use std::{
    mem,
    ops::{Deref, DerefMut},
    sync::{Arc, TryLockError},
};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum OwningShardedLockError {
    #[error("Operation would block")]
    WouldBlock,
}

/// An exclusive lock guard that owns the 'Arc'-ed lock.
pub struct OwningShardedWriteGuard<'a, T> {
    owner: Arc<ShardedLock<T>>,
    guard: Option<ShardedLockWriteGuard<'a, T>>,
}

impl<'a, T> Drop for OwningShardedWriteGuard<'a, T> {
    fn drop(&mut self) {
        // make sure the guard is dropped before the owner to avoid any use after free
        self.guard = None;
    }
}

impl<'a, T> OwningShardedWriteGuard<'a, T> {
    pub fn try_new(owner: Arc<ShardedLock<T>>) -> Result<Self, OwningShardedLockError> {
        let guard = owner.try_write().map_err(|err| match err {
            TryLockError::WouldBlock => OwningShardedLockError::WouldBlock,
            _ => panic!(),
        })?;
        // SAFETY
        // The ownership of the lock is kept in the struct, thus lifetime of the guard can be safely extended.
        // This is an unsafe workaround to solve the self-referential issue that cannot be expressed in safe-rust.
        let guard = unsafe { mem::transmute(guard) };
        Ok(OwningShardedWriteGuard::from_raw(owner, guard))
    }

    pub fn new(owner: Arc<ShardedLock<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    fn from_raw(owner: Arc<ShardedLock<T>>, guard: ShardedLockWriteGuard<'a, T>) -> OwningShardedWriteGuard<'a, T> {
        Self {
            owner,
            guard: Some(guard),
        }
    }

    /*fn into_raw(mut self) -> (Arc<ShardedLock<T>>, ShardedLockWriteGuard<'a, T>) {
        (self.owner.clone(), self.guard.take().unwrap())
    }*/

    pub fn owner(&self) -> &Arc<ShardedLock<T>> {
        &self.owner
    }

    pub fn get(&self) -> &T {
        // As object can be access mutable trough `get_mut`, the returned reference cannot outlive &self
        // compared to the same function in OwningShardedReadGuard.
        self.guard.as_deref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.guard.as_deref_mut().unwrap()
    }
}

impl<'a, T> Deref for OwningShardedWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> DerefMut for OwningShardedWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// A shared lock guard that owns the 'Arc'-ed lock.
pub struct OwningShardedReadGuard<'a, T> {
    owner: Arc<ShardedLock<T>>,
    guard: Option<ShardedLockReadGuard<'a, T>>,
}

impl<'a, T> Drop for OwningShardedReadGuard<'a, T> {
    fn drop(&mut self) {
        // make sure the guard is dropped before the owner to avoid any use after free
        self.guard = None;
    }
}

impl<'a, T> OwningShardedReadGuard<'a, T> {
    pub fn try_new(owner: Arc<ShardedLock<T>>) -> Result<Self, OwningShardedLockError> {
        let guard = owner.try_read().map_err(|err| match err {
            TryLockError::WouldBlock => OwningShardedLockError::WouldBlock,
            _ => panic!(),
        })?;
        // SAFETY
        // The ownership of the lock is kept in the struct, thus lifetime of the guard can be safely extended.
        // This is an unsafe workaround to solve the self-referential issue that cannot be expressed in safe-rust.
        let guard = unsafe { mem::transmute(guard) };
        Ok(OwningShardedReadGuard::from_raw(owner, guard))
    }

    pub fn new(owner: Arc<ShardedLock<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    fn from_raw(owner: Arc<ShardedLock<T>>, guard: ShardedLockReadGuard<'a, T>) -> OwningShardedReadGuard<'a, T> {
        Self {
            owner,
            guard: Some(guard),
        }
    }

    /*fn into_raw(mut self) -> (Arc<ShardedLock<T>>, ShardedLockReadGuard<'a, T>) {
        (self.owner.clone(), self.guard.take().unwrap())
    }*/

    pub fn owner(&self) -> &Arc<ShardedLock<T>> {
        &self.owner
    }

    pub fn get(&self) -> &'a T {
        // Safety
        // It is safe to increase the lifetime from 'self to 'a as the lock is held for the entire 'a
        // and only shared access can be create for the entire 'a.
        let value = self.guard.as_deref().unwrap();
        unsafe { mem::transmute(value) }
    }
}

impl<'a, T> Deref for OwningShardedReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
