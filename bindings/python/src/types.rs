use graph::NodeT;
use numpy::{PyArray1, PyArray2};
use pyo3::prelude::*;

pub struct ThreadDataRaceAware<'a, T> {
    pub(crate) t: &'a T,
}

unsafe impl<'a, T> Sync for ThreadDataRaceAware<'a, T> {}
