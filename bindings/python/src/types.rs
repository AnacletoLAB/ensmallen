pub struct ThreadDataRaceAware<'a, T> {
    pub(crate) t: &'a T,
}

unsafe impl<'a, T> Sync for ThreadDataRaceAware<'a, T> {}
