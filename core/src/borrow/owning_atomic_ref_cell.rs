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
pub struct OwningAtomicRefMutGuard<'a, T, U: ?Sized > {
    owner: Arc<AtomicRefCell<T>>,
    guard: Option<AtomicRefMut<'a, U>>,
}

impl<'a, T, U: ?Sized> Drop for OwningAtomicRefMutGuard<'a, T, U> {
    fn drop(&mut self) {
        // make sure the guard is dropped before the owner to avoid any use after free
        self.guard = None;
    }
}

impl<'a, T> OwningAtomicRefMutGuard<'a, T, T> {
    pub fn try_new(owner: Arc<AtomicRefCell<T>>) -> Result<Self, OwningAtomicRefError> {
        // Safety
        // The ownership of the lock is kept in the struct, thus lifetime of the guard can be safely extended.
        // This is an unsafe workaround to solve the self-referential issue that cannot be expressed in safe-rust.
        let guard = owner.try_borrow_mut().map_err(|_| OwningAtomicRefError::BorrowError)?;
        let guard = unsafe { mem::transmute(guard) };
        Ok(OwningAtomicRefMutGuard::from_raw(owner, guard))
    }

    /// # Panics
    /// This function panics if the resource has been already mutable borrowed.
    pub fn new(owner: Arc<AtomicRefCell<T>>) -> Self {
        Self::try_new(owner).unwrap()
    }
}

impl<'a, T, U: ?Sized> OwningAtomicRefMutGuard<'a, T, U> {
    fn from_raw(owner: Arc<AtomicRefCell<T>>, guard: AtomicRefMut<'a, U>) -> OwningAtomicRefMutGuard<'a, T, U> {
        Self {
            owner,
            guard: Some(guard),
        }
    }

    fn into_raw(mut self) -> (Arc<AtomicRefCell<T>>, AtomicRefMut<'a, U>) {
        (self.owner.clone(), self.guard.take().unwrap())
    }

    pub fn owner(&self) -> &Arc<AtomicRefCell<T>> {
        &self.owner
    }

    pub fn map<Q: ?Sized, F: FnOnce(&mut U) -> &mut Q>(self, map: F) -> OwningAtomicRefMutGuard<'a, T, Q> {
        let (owner, guard) = self.into_raw();
        let guard = AtomicRefMut::map(guard, map);
        OwningAtomicRefMutGuard::from_raw(owner, guard)
    }

    pub fn fiter_map<Q: ?Sized, F: FnOnce(&mut U) -> Option<&mut Q>>(
        self,
        map: F,
    ) -> Option<OwningAtomicRefMutGuard<'a, T, Q>> {
        let (owner, guard) = self.into_raw();
        let guard = AtomicRefMut::filter_map(guard, map)?;
        Some(OwningAtomicRefMutGuard::from_raw(owner, guard))
    }

    pub fn try_map<Q: ?Sized, E, F: FnOnce(&mut U) -> Result<&mut Q, E>>(
        self,
        map: F,
    ) -> Result<OwningAtomicRefMutGuard<'a, T, Q>, E> {
        let (owner, guard) = self.into_raw();
        // Atomic ref has no try_map, thus store the result in a temporary.
        let mut error = None;
        let guard = AtomicRefMut::filter_map(guard, |a| match map(a) {
            Err(err) => {
                error = Some(err);
                None
            }
            Ok(value) => Some(value),
        });
        if let Some(err) = error {
            Err(err)
        } else {
            Ok(OwningAtomicRefMutGuard::from_raw(owner, guard.unwrap()))
        }
    }

    pub fn get(&self) -> &U {
        // As object can be access mutable trough `get_mut`, the returned reference cannot outlive &self
        // compared to the same function in OwningAtomicRefGuard.
        self.guard.as_deref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut U {
        self.guard.as_deref_mut().unwrap()
    }
}

impl<'a, T, U: ?Sized> Deref for OwningAtomicRefMutGuard<'a, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T, U: ?Sized> DerefMut for OwningAtomicRefMutGuard<'a, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// A shared lock guard that owns the 'Arc'-ed lock.
pub struct OwningAtomicRefGuard<'a, T, U: ?Sized + 'a> {
    owner: Arc<AtomicRefCell<T>>,
    guard: Option<AtomicRef<'a, U>>,
}

impl<'a, T, U: ?Sized> Drop for OwningAtomicRefGuard<'a, T, U> {
    fn drop(&mut self) {
        // make sure the guard is dropped before the owner to avoid any use after free
        self.guard = None;
    }
}

impl<'a, T> OwningAtomicRefGuard<'a, T, T> {
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
}

impl<'a, T, U: ?Sized> OwningAtomicRefGuard<'a, T, U> {
    fn from_raw(owner: Arc<AtomicRefCell<T>>, guard: AtomicRef<'a, U>) -> OwningAtomicRefGuard<'a, T, U> {
        Self {
            owner,
            guard: Some(guard),
        }
    }

    fn into_raw(mut self) -> (Arc<AtomicRefCell<T>>, AtomicRef<'a, U>) {
        (self.owner.clone(), self.guard.take().unwrap())
    }

    pub fn owner(&self) -> &Arc<AtomicRefCell<T>> {
        &self.owner
    }

    pub fn map<Q: ?Sized, F: FnOnce(&U) -> &Q>(self, map: F) -> OwningAtomicRefGuard<'a, T, Q> {
        let (owner, guard) = self.into_raw();
        let guard = AtomicRef::map(guard, map);
        OwningAtomicRefGuard::from_raw(owner, guard)
    }

    pub fn fiter_map<Q: ?Sized, F: FnOnce(&U) -> Option<&Q>>(self, map: F) -> Option<OwningAtomicRefGuard<'a, T, Q>> {
        let (owner, guard) = self.into_raw();
        let guard = AtomicRef::filter_map(guard, map)?;
        Some(OwningAtomicRefGuard::from_raw(owner, guard))
    }

    pub fn try_map<Q: ?Sized, E, F: FnOnce(&U) -> Result<&Q, E>>(self, map: F) -> Result<OwningAtomicRefGuard<'a, T, Q>, E> {
        let (owner, guard) = self.into_raw();
        let mut error = None;
        let guard = AtomicRef::filter_map(guard, |v| match map(v) {
            Err(err) => {
                error = Some(err);
                None
            }
            Ok(value) => Some(value),
        });
        if let Some(err) = error {
            Err(err)
        } else {
            Ok(OwningAtomicRefGuard::from_raw(owner, guard.unwrap()))
        }
    }

    /// Compared to `deref` it returns the referenced object with an extended lifetime to 'a.
    pub fn get(&self) -> &'a U {
        // Safety
        // It is safe to increase the lifetime from 'self to 'a as the lock is held for the entire 'a
        // and only shared access can be create for the entire 'a.
        let value = self.guard.as_deref().unwrap();
        unsafe { mem::transmute(value) }
    }
}

impl<'a, T, U: ?Sized> Deref for OwningAtomicRefGuard<'a, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
