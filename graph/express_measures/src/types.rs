use core::fmt::Debug;
use num_traits::{Float, Unsigned};

pub trait ThreadFloat: Float + Send + Sync + Copy {}
pub trait ThreadUnsigned: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

impl<T> ThreadFloat for T where T: Float + Send + Sync + Copy {}
impl<T> ThreadUnsigned for T where T: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}
