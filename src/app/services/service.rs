use std::borrow::{Borrow, BorrowMut};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Service<T>
where
    T: ?Sized,
{
    service: RwLock<T>,
}

impl<T> From<RwLock<T>> for Service<T> {
    fn from(value: RwLock<T>) -> Self {
        Self { service: value }
    }
}

impl<T> From<T> for Service<T> {
    fn from(value: T) -> Self {
        let service = RwLock::new(value);
        Self { service }
    }
}

impl<T> Service<T>
where
    T: ?Sized,
{
    pub async fn read(&self) -> ServiceLockReadGuard<T> {
        let read_guard = self.service.read().await;
        ServiceLockReadGuard {
            service: self,
            lock_guard: read_guard,
        }
    }
    pub async fn write(&self) -> ServiceLockWriteGuard<T> {
        let write_guard = self.service.write().await;
        ServiceLockWriteGuard {
            service: self,
            lock_guard: write_guard,
        }
    }
}

// Read Guard
pub struct ServiceLockReadGuard<'a, T>
where
    T: ?Sized,
{
    service: &'a Service<T>,
    lock_guard: RwLockReadGuard<'a, T>,
}

impl<'a, T> ServiceLockReadGuard<'a, T>
where
    T: ?Sized,
{
    pub fn as_ref(&self) -> &T {
        let lock_guard = self.lock_guard.borrow();
        lock_guard
    }
}

// Write Guard
pub struct ServiceLockWriteGuard<'a, T>
where
    T: ?Sized,
{
    service: &'a Service<T>,
    lock_guard: RwLockWriteGuard<'a, T>,
}

impl<'a, T> ServiceLockWriteGuard<'a, T>
where
    T: ?Sized,
{
    pub fn as_ref(&self) -> &T {
        let lock_guard = self.lock_guard.borrow();
        lock_guard
    }

    pub fn as_mut(&mut self) -> &mut T {
        let lock_guard = self.lock_guard.borrow_mut();
        &mut *lock_guard
    }
}
