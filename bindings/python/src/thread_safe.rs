pub struct ThreadSafe<'a, T> {
    pub(crate) t: &'a T,
}

unsafe impl<'a, T> Sync for ThreadSafe<'a, T> {}
