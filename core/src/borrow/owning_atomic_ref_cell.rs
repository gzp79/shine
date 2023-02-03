use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::{
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum OwningAtomicRefError {
    #[error("Cell already mutably borrowed")]
    BorrowError,
}

/// An exclusive lock guard that owns the 'Arc'-ed lock.
pub struct OwningAtomicRefMutGuard<'a, T> {
    owner: Arc<AtomicRefCell<T>>,
    guard: AtomicRefMut<'a, T>,
}

impl<'a, T> OwningAtomicRefMutGuard<'a, T> {
    pub fn try_new(owner: Arc<AtomicRefCell<T>>) -> Result<Self, OwningAtomicRefError> {
        let guard = owner.try_borrow_mut().map_err(|_| OwningAtomicRefError::BorrowError)?;
        // SAFETY
        // The ownership of the lock is kept in the struct, thus lifetime of the guard can be safely extended.
        // This is an unsafe workaround to solve the self-referential issue that cannot be expressed in safe-rust.
        let guard = unsafe { mem::transmute(guard) };
        Ok(Self { owner, guard })
    }

    /// # Panics
    /// This function panics if the resource has been already mutable borrowed.
    pub fn new(owner: Arc<AtomicRefCell<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    pub fn owner(&self) -> &Arc<AtomicRefCell<T>> {
        &self.owner
    }

    pub fn get(&self) -> &T {
        // As object can be access mutable trough `get_mut`, the returned reference cannot outlive &self
        // compared to the same function in OwningAtomicRefGuard.
        &self.guard
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<'a, T> Deref for OwningAtomicRefMutGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> DerefMut for OwningAtomicRefMutGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// A shared lock guard that owns the 'Arc'-ed lock.
pub struct OwningAtomicRefGuard<'a, T> {
    owner: Arc<AtomicRefCell<T>>,
    guard: AtomicRef<'a, T>,
}

impl<'a, T> OwningAtomicRefGuard<'a, T> {
    pub fn try_new(owner: Arc<AtomicRefCell<T>>) -> Result<Self, OwningAtomicRefError> {
        let guard = owner.try_borrow().map_err(|_| OwningAtomicRefError::BorrowError)?;
        // SAFETY
        // The ownership of the lock is kept in the struct, thus lifetime of the guard can be safely extended.
        // This is an unsafe workaround to solve the self-referential issue that cannot be expressed in safe-rust.
        let guard = unsafe { mem::transmute(guard) };
        Ok(Self { owner, guard })
    }

    /// # Panics
    /// This function panics if the resource has been already mutable borrowed.
    pub fn new(owner: Arc<AtomicRefCell<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }

    pub fn owner(&self) -> &Arc<AtomicRefCell<T>> {
        &self.owner
    }

    /// Compared to `deref` it returns the referenced object with an extended lifetime to 'a.
    pub fn get(&self) -> &'a T {
        // SAFETY
        // It is safe to increase the lifetime from 'self to 'a as the lock is held for the entire 'a
        // and only shared access can be create for the entire 'a.
        unsafe { &*(&*self.guard as *const _) }
    }
}

impl<'a, T> Deref for OwningAtomicRefGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
