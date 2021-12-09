use rayon::prelude::*;

pub trait MinMax<T> {
    fn minmax(self) -> Option<(T, T)>;
}

impl<I, T: PartialOrd + Copy + Sized + Send> MinMax<T> for I
where
    I: ParallelIterator<Item = T>,
{
    fn minmax(self) -> Option<(T, T)> {
        self.map(|e| Some((e, e))).reduce(
            || None,
            |a, b| match (a, b) {
                (Some((min_a, max_a)), Some((min_b, max_b))) => Some((
                    if min_a > min_b { min_b } else { min_a },
                    if max_a < max_b { max_b } else { max_a },
                )),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
        )
    }
}
