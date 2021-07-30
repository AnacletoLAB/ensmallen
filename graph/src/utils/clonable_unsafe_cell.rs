use std::cell::UnsafeCell;
use std::fmt::Debug;

/// A Clonable unsafe cell
pub(crate) struct ClonableUnsafeCell<T> {
    value: UnsafeCell<T>,
}

impl<T: Debug> Debug for ClonableUnsafeCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.get().fmt(f)
    }
}

impl<T: Default> Default for ClonableUnsafeCell<T> {
    fn default() -> Self {
        ClonableUnsafeCell::new(T::default())
    }
}

impl<T: Clone> Clone for ClonableUnsafeCell<T> {
    fn clone(&self) -> Self {
        ClonableUnsafeCell {
            value: UnsafeCell::new(unsafe { (*self.value.get()).clone() }),
        }
    }
}

impl<T> ClonableUnsafeCell<T> {
    pub unsafe fn get(&self) -> *mut T {
        self.value.get()
    }

    pub fn new(value: T) -> Self {
        ClonableUnsafeCell {
            value: UnsafeCell::new(value),
        }
    }
}

unsafe impl<T> Sync for ClonableUnsafeCell<T> {}
