use rayon::prelude::*;

pub trait ArgMaxArgMin<T> {
    fn argmax(self) -> Option<(usize, T)>;
    fn argmin(self) -> Option<(usize, T)>;
}

impl<I, T: PartialOrd + Copy + Sized + Send> ArgMaxArgMin<T> for I
where
    I: IndexedParallelIterator<Item = T>,
{
    fn argmax(self) -> Option<(usize, T)> {
        self.enumerate().map(|e| Some(e)).reduce(
            || None,
            |a, b| match (a, b) {
                (Some((i, a)), Some((j, b))) => Some(if a > b { (i, a) } else { (j, b) }),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
        )
    }

    fn argmin(self) -> Option<(usize, T)> {
        self.enumerate().map(|e| Some(e)).reduce(
            || None,
            |a, b| match (a, b) {
                (Some((i, a)), Some((j, b))) => Some(if a < b { (i, a) } else { (j, b) }),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
        )
    }
}
