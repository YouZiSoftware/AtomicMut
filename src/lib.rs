use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
pub struct AtomicMut<T> {
    inner: AtomicPtr<T>
}

unsafe impl<T> Send for AtomicMut<T> {}
unsafe impl<T> Sync for AtomicMut<T> {}

impl<T> AtomicMut<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: AtomicPtr::new(Box::into_raw(Box::new(value)))
        }
    }

    pub fn read(&self) -> &T {
        unsafe {
            &*self.inner.load(Ordering::Acquire)
        }
    }

    pub fn write(&self) -> AtomicMutGuard<T> {
        AtomicMutGuard::new(self, self.inner.load(Ordering::Acquire))
    }
}

impl <T: Clone> Clone for AtomicMut<T> {
    fn clone(&self) -> Self {
        Self::new(self.read().clone())
    }
}

pub struct AtomicMutGuard<'a, T> {
    inner: &'a AtomicMut<T>,
    mut_ref: *mut T,
}

unsafe impl<T> Send for AtomicMutGuard<'_, T> {}
unsafe impl<T> Sync for AtomicMutGuard<'_, T> {}

impl<'a, T> AtomicMutGuard<'a, T> {
    pub fn new(inner: &'a AtomicMut<T>, mut_ref: *mut T) -> Self {
        Self {
            inner,
            mut_ref
        }
    }

    pub fn update(&self) {
        self.inner.inner.store(self.mut_ref.clone(), Ordering::Release);
    }
}

impl<T> Deref for AtomicMutGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mut_ref }
    }
}

impl<T> DerefMut for AtomicMutGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mut_ref }
    }
}

impl <T> Drop for AtomicMutGuard<'_, T> {
    fn drop(&mut self) {
        self.inner.inner.store(self.mut_ref, Ordering::Release)
    }
}