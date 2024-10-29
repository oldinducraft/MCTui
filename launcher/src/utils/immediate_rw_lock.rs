use std::sync::{LockResult, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

pub struct ImmediateRwLock<T: Clone> {
    inner: RwLock<T>,
}

impl<T: Clone> ImmediateRwLock<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: RwLock::new(inner),
        }
    }

    /// Acquires a write lock, blocking the current thread until lock can be
    /// acquired
    #[allow(dead_code)]
    pub fn write_lock(&self) -> LockResult<RwLockWriteGuard<T>> { self.inner.write() }

    /// Attempts to set the value blocking the current thread until lock can be
    /// acquired
    pub fn set(&self, value: T) -> Result<(), PoisonError<RwLockWriteGuard<T>>> {
        *self.inner.write()? = value;
        Ok(())
    }

    /// Attempts to get the value blocking the current thread until lock can be
    /// acquired
    pub fn get(&self) -> Result<T, PoisonError<RwLockReadGuard<T>>> {
        let lock = self.inner.read()?;
        Ok(lock.clone())
    }

    /// Attempts to set the value immediately
    pub fn try_set(&self, value: T) -> Result<(), TryLockError<RwLockWriteGuard<T>>> {
        *self.inner.try_write()? = value;
        Ok(())
    }
}

impl<T: Clone + Default> Default for ImmediateRwLock<T> {
    fn default() -> Self {
        Self {
            inner: RwLock::new(T::default()),
        }
    }
}
