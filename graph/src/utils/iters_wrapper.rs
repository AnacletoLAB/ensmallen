use super::*;
use rayon::prelude::*;
use std::fmt;
use std::iter::FromIterator;

/// Struct that we use to threat sequential and parallel iterators in an
/// homogeneus way.
///
/// # Example
/// ```ignore
/// let x = true;
/// let v = 69420;
/// let iter = if x {
///     ItersWrapper::Sequential((0..100).into_iter())
/// } else {
///     ItersWrapper::Parallel((0..100).into_par_iter().map(|x| x + 1))
/// };
///
/// println!("{:?}", iter.map(|x: i32| -> i32 {v + x * 2}).collect::<Vec<_>>());
/// ```
pub enum ItersWrapper<Item, I: Iterator<Item = Item>, P: ParallelIterator<Item = Item>> {
    Sequential(I),
    Parallel(P),
}

impl<Item, I: Iterator<Item = Item>, P: ParallelIterator<Item = Item>> fmt::Debug
    for ItersWrapper<Item, I, P>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItersWrapper")
            .field(
                "iter_type",
                &match &self {
                    ItersWrapper::Parallel(_) => "Parallel",
                    ItersWrapper::Sequential(_) => "Sequential",
                },
            )
            .field("items_type", &std::any::type_name::<Item>())
            .finish()
    }
}

impl<Item, I, P> ItersWrapper<Item, I, P>
where
    Item: Send,
    I: Iterator<Item = Item>,
    P: ParallelIterator<Item = Item>,
{
    pub fn unwrap_sequential(self) -> I {
        match self {
            ItersWrapper::Parallel(_) => panic!("Cannot unwrap a parallel iterator as sequential"),
            ItersWrapper::Sequential(i) => i,
        }
    }

    /// Returns true if the iterator is sequential.
    pub fn is_sequential(&self) -> bool {
        match &self {
            ItersWrapper::Parallel(_) => false,
            ItersWrapper::Sequential(_) => true,
        }
    }

    /// Returns true if the iterator is parallel.
    pub fn is_parallel(&self) -> bool {
        match &self {
            ItersWrapper::Parallel(_) => true,
            ItersWrapper::Sequential(_) => false,
        }
    }

    pub fn unwrap_parallel(self) -> P {
        match self {
            ItersWrapper::Sequential(_) => {
                panic!("Cannot unwrap a sequential iterator as parallel")
            }
            ItersWrapper::Parallel(i) => i,
        }
    }

    pub fn sum<S>(self) -> S
    where
        S: Send + std::iter::Sum<Item> + std::iter::Sum<S>,
    {
        match self {
            Self::Parallel(p) => p.sum(),
            Self::Sequential(i) => i.sum(),
        }
    }

    pub fn map<F, R>(self, op: F) -> ItersWrapper<R, std::iter::Map<I, F>, rayon::iter::Map<P, F>>
    where
        R: Send,
        F: Fn(Item) -> R + Sync + Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(p.map(op)),
            Self::Sequential(i) => ItersWrapper::Sequential(i.map(op)),
        }
    }

    pub fn filter_map<F, R>(
        self,
        op: F,
    ) -> ItersWrapper<R, std::iter::FilterMap<I, F>, rayon::iter::FilterMap<P, F>>
    where
        R: Send,
        F: Fn(Item) -> Option<R> + Sync + Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(p.filter_map(op)),
            Self::Sequential(i) => ItersWrapper::Sequential(i.filter_map(op)),
        }
    }

    pub fn flat_map<F, R, U>(
        self,
        op: F,
    ) -> ItersWrapper<R, std::iter::FlatMap<I, U, F>, rayon::iter::FlatMap<P, F>>
    where
        R: Send,
        U: IntoParallelIterator<Item = R> + IntoIterator<Item = R>,
        F: Fn(Item) -> U + Sync + Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(p.flat_map(op)),
            Self::Sequential(i) => ItersWrapper::Sequential(i.flat_map(op)),
        }
    }

    pub fn flat_map_iter<F, R, U>(
        self,
        op: F,
    ) -> ItersWrapper<R, std::iter::FlatMap<I, U, F>, rayon::iter::FlatMapIter<P, F>>
    where
        R: Send,
        U: IntoParallelIterator<Item = R> + IntoIterator<Item = R>,
        F: Fn(Item) -> U + Sync + Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(p.flat_map_iter(op)),
            Self::Sequential(i) => ItersWrapper::Sequential(i.flat_map(op)),
        }
    }

    pub fn method_caller<'a, R, S>(
        self,
        sequential_op: fn(&mut S, Item) -> R,
        parallel_op: fn(&mut S, Item) -> R,
        context: &'a mut S,
    ) -> ItersWrapper<R, SequentialMethodCaller<'a, Item, R, S, I>, MethodCaller<Item, R, S, P>>
    where
        R: Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(MethodCaller::new(
                p,
                parallel_op,
                context as *const S as usize,
            )),
            Self::Sequential(i) => {
                ItersWrapper::Sequential(SequentialMethodCaller::new(i, sequential_op, context))
            }
        }
    }

    pub fn filter<F>(
        self,
        op: F,
    ) -> ItersWrapper<Item, std::iter::Filter<I, F>, rayon::iter::Filter<P, F>>
    where
        F: Fn(&Item) -> bool + Sync + Send,
    {
        match self {
            Self::Parallel(p) => ItersWrapper::Parallel(p.filter(op)),
            Self::Sequential(i) => ItersWrapper::Sequential(i.filter(op)),
        }
    }

    /// Implements the `count` reduce method for the iterator.
    pub fn count(self) -> usize {
        match self {
            Self::Parallel(p) => p.count(),
            Self::Sequential(i) => i.count(),
        }
    }

    pub fn for_each<F>(self, op: F)
    where
        F: Fn(Item) + Sync + Send,
    {
        match self {
            Self::Parallel(p) => p.for_each(op),
            Self::Sequential(i) => i.for_each(op),
        }
    }

    pub fn reduce<ID, F>(self, identity: ID, op: F) -> Item
    where
        F: Fn(Item, Item) -> Item + Sync + Send,
        ID: Fn() -> Item + Sync + Send,
    {
        match self {
            Self::Parallel(p) => p.reduce(identity, op),
            Self::Sequential(i) => i.chain(vec![identity()].into_iter()).reduce(op).unwrap(),
        }
    }

    pub fn all<F>(self, op: F) -> bool
    where
        F: Fn(Item) -> bool + Sync + Send,
    {
        match self {
            Self::Parallel(p) => p.all(op),
            Self::Sequential(mut i) => i.all(op),
        }
    }

    pub fn any<F>(self, op: F) -> bool
    where
        F: Fn(Item) -> bool + Sync + Send,
    {
        match self {
            Self::Parallel(p) => p.any(op),
            Self::Sequential(mut i) => i.any(op),
        }
    }

    pub fn collect<B>(self) -> B
    where
        B: FromIterator<Item> + FromParallelIterator<Item>,
    {
        match self {
            Self::Parallel(p) => p.collect::<B>(),
            Self::Sequential(i) => i.collect::<B>(),
        }
    }
}

impl<Item, I, P> ItersWrapper<Item, I, P>
where
    Item: Send + Ord,
    I: Iterator<Item = Item>,
    P: ParallelIterator<Item = Item>,
{
    pub fn max<F>(self) -> Option<Item> {
        match self {
            Self::Parallel(p) => p.max(),
            Self::Sequential(i) => i.max(),
        }
    }

    pub fn min<F>(self) -> Option<Item> {
        match self {
            Self::Parallel(p) => p.min(),
            Self::Sequential(i) => i.min(),
        }
    }
}
