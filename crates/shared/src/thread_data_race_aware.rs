use std::cell::UnsafeCell;

pub struct ThreadDataRaceAware<T> {
    pub value: UnsafeCell<T>,
}

unsafe impl<T> Sync for ThreadDataRaceAware<T> {}

pub struct ThreadDataRaceAwareMutable<T> {
    pub value: T,
}

unsafe impl<T> Sync for ThreadDataRaceAwareMutable<T> {}