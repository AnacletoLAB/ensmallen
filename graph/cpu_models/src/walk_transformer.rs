use crate::must_not_be_zero;
use core::fmt::Debug;
use rayon::prelude::*;

pub trait WalkTransformer: Send + Sync + Clone + Debug + Default {
    type I<'a, T>: IndexedParallelIterator<Item = (usize, Vec<T>)> + 'a
    where
        Self: 'a,
        T: Copy + Send + Sync + 'a;

    fn par_transform_walk<'a, T>(&'a self, i: usize, walk: Vec<T>) -> Self::I<'a, T>
    where
        T: Copy + Send + Sync + 'a;
}

#[derive(Debug, Clone, Default)]
pub struct IdentifyWalkTransformer();

unsafe impl Sync for IdentifyWalkTransformer {}
unsafe impl Send for IdentifyWalkTransformer {}

impl WalkTransformer for IdentifyWalkTransformer {
    type I<'a, T> = impl IndexedParallelIterator<Item = (usize, Vec<T>)> + 'a where
        Self: 'a,
        T: Copy + Send + Sync + 'a;

    fn par_transform_walk<'a, T>(&'a self, i: usize, walk: Vec<T>) -> Self::I<'a, T>
    where
        T: Copy + Send + Sync + 'a,
    {
        vec![(i, walk)].into_par_iter()
    }
}

#[derive(Debug, Clone)]
pub struct WalkletsWalkTransformer {
    power: usize,
}

impl Default for WalkletsWalkTransformer {
    fn default() -> Self {
        WalkletsWalkTransformer { power: 0 }
    }
}

unsafe impl Sync for WalkletsWalkTransformer {}
unsafe impl Send for WalkletsWalkTransformer {}

impl WalkletsWalkTransformer {
    pub fn new(power: usize) -> Result<Self, String> {
        let power = must_not_be_zero(Some(power), 5, "power")?;
        Ok(WalkletsWalkTransformer { power })
    }
}

impl WalkTransformer for WalkletsWalkTransformer {
    type I<'a, T> = impl IndexedParallelIterator<Item = (usize, Vec<T>)> + 'a where
        Self: 'a,
        T: Copy + Send + Sync + 'a;

    fn par_transform_walk<'a, T>(&'a self, i: usize, walk: Vec<T>) -> Self::I<'a, T>
    where
        T: Copy + Send + Sync + 'a,
    {
        (0..(self.power + 1)).into_par_iter().map(move |step_size| {
            (
                i * (self.power + 1) + step_size,
                walk[step_size..]
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(position, _)| (position % self.power) == 0)
                    .map(|(_, node)| node)
                    .collect(),
            )
        })
    }
}
