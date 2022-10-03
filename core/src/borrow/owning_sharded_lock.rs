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

/// A write lock guard that owns the 'Arc'-ed lock.
pub struct OwningShardedWriteGuard<'a, T> {
    owner: Arc<ShardedLock<T>>,
    guard: ShardedLockWriteGuard<'a, T>,
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
        Ok(Self { owner, guard })
    }

    pub fn new(owner: Arc<ShardedLock<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    pub fn owner(&self) -> &Arc<ShardedLock<T>> {
        &self.owner
    }

    pub fn get(&self) -> &T {
        // As object can be access mutable trough `get_mut`, the returned reference cannot outlive &self
        // compared to the same function in OwningShardedReadGuard.
        &self.guard
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.guard
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

/// A write lock guard that owns the 'Arc'-ed lock.
pub struct OwningShardedReadGuard<'a, T> {
    owner: Arc<ShardedLock<T>>,
    guard: ShardedLockReadGuard<'a, T>,
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
        Ok(Self { owner, guard })
    }

    pub fn new(owner: Arc<ShardedLock<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    pub fn owner(&self) -> &Arc<ShardedLock<T>> {
        &self.owner
    }

    pub fn get(&self) -> &'a T {
        // SAFETY
        // It is safe to increase the lifetime from 'self to 'a as the lock is held for the entire 'a
        // and only shared access can be create for the entire 'a.
        unsafe { &*(&*self.guard as *const _) }
    }
}

impl<'a, T> Deref for OwningShardedReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
