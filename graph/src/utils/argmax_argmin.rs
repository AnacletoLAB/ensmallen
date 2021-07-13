
pub trait ArgMaxArgMin<T> {
    fn argmax(&self) -> Option<(usize, T)>;
    fn argmin(&self) -> Option<(usize, T)>;
}

impl<T: PartialOrd + Copy> ArgMaxArgMin<T> for Vec<T> {
    fn argmax(&self) -> Option<(usize, T)> {
        self.iter()
            .enumerate()
            .fold(None, |current_max, (i, &value)| {
                current_max.map_or(Some((i, value)), |(j, current_max_value)| {
                    Some(if value > current_max_value {
                        (i, value)
                    } else {
                        (j, current_max_value)
                    })
                })
            })
    }

    fn argmin(&self) -> Option<(usize, T)> {
        self.iter()
            .enumerate()
            .fold(None, |current_max, (i, &value)| {
                current_max.map_or(Some((i, value)), |(j, current_max_value)| {
                    Some(if value < current_max_value {
                        (i, value)
                    } else {
                        (j, current_max_value)
                    })
                })
            })
    }
}

