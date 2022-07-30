use mmap::MemoryMappedReadOnly;
use memchr;
use rayon::iter::plumbing::{bridge_unindexed, UnindexedProducer};
use rayon::prelude::*;
use std::sync::Arc;

pub const READER_CAPACITY: usize = 1 << 17;

type IterType = (usize, Result<String, String>);

pub struct ParallelLinesWithIndex {
    mmap: Arc<MemoryMappedReadOnly>,
    comment_symbol: Option<String>,
    number_of_lines: Option<usize>,
    number_of_rows_to_skip: Option<usize>,
    max_producers: usize,
}

impl ParallelLinesWithIndex {
    pub fn new(path: &str) -> Result<ParallelLinesWithIndex, String> {
        Ok(ParallelLinesWithIndex {
            mmap: Arc::new(MemoryMappedReadOnly::new(path)?),
            number_of_lines: None,
            comment_symbol: None,
            number_of_rows_to_skip: None,
            max_producers: num_cpus::get(),
        })
    }

    pub fn set_max_producers(&mut self, max_producers: usize) {
        self.max_producers = max_producers;
    }

    pub fn set_skip_rows(&mut self, number_of_rows_to_skip: usize) {
        self.number_of_rows_to_skip = Some(number_of_rows_to_skip);
    }

    pub fn set_comment_symbol(&mut self, comment_symbol: Option<String>) {
        self.comment_symbol = comment_symbol;
    }
}

impl ParallelIterator for ParallelLinesWithIndex {
    type Item = IterType;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let mut data = self.mmap.as_str();
        // Skip the first rows (as specified by the user)
        if let Some(rts) = self.number_of_rows_to_skip {
            for _ in 0..rts {
                let mut _buffer = String::new();
                let (_, rest) = data.split_once("\n").unwrap_or(("", data));
                data = rest;
            }
        }

        // Create the first producer
        let producer = ParalellLinesProducerWithIndex {
            mmap: self.mmap.clone(),
            data,
            line_count: 0,
            modulus_mask: 0,
            depth: 0,
            remainder: 0,
            maximal_depth: (self.max_producers as f64).log2().ceil() as usize,
            comment_symbol: self.comment_symbol.clone(),
        };
        bridge_unindexed(producer, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.number_of_lines
    }
}

#[derive(Debug)]
struct ParalellLinesProducerWithIndex {
    mmap: Arc<MemoryMappedReadOnly>,
    data: &'static str,
    line_count: usize,
    modulus_mask: usize,
    remainder: usize,
    maximal_depth: usize,
    depth: usize,
    comment_symbol: Option<String>,
}

impl ParalellLinesProducerWithIndex {
    #[inline]
    fn get_modulus(&self) -> usize {
        self.modulus_mask + 1
    }
}

impl Iterator for ParalellLinesProducerWithIndex {
    type Item = IterType;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.data.trim().is_empty() {
                return None;
            }

            let line_length =
                memchr::memchr(b'\n', self.data.as_bytes()).unwrap_or(self.data.len() - 1);

            if let Some(comment_symbol) = self.comment_symbol.as_deref() {
                if self.data.starts_with(comment_symbol) {
                    // skip this line and go to the next
                    self.data = &self.data[line_length + 1..];
                    continue;
                }
            }

            // skip empty lines
            if self.data[..line_length].trim().is_empty() {
                self.data = &self.data[line_length + 1..];
                continue;
            }

            // skip lines until we met the one with the right remainder
            if (self.line_count & self.modulus_mask) != self.remainder {
                self.data = &self.data[line_length + 1..];
                self.line_count += 1;
                continue;
            }

            let result = &self.data[..line_length];
            self.data = &self.data[line_length + 1..];
            self.line_count += 1;
            return Some((self.line_count - 1, Ok(result.to_string())));
        }
    }
}

impl UnindexedProducer for ParalellLinesProducerWithIndex {
    type Item = IterType;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split the stream
        if self.depth >= self.maximal_depth.saturating_sub(1) {
            return (self, None);
        }
        // Since we only do binary splits, the modulus will always be a power of
        // two. So when checking if the current line should be retrieved,
        // we don't need to use the slower % but we can use the faster &.
        // In particular, if modulus is a power of two, it holds that:
        // x % modulus == x & (modulus - 1)
        // So what we call modulus_mask is (modulus - 1), and thus to get it back
        // we can do modulus = (modulus_mask + 1).
        //
        // Here we want to double the modulus, so we compute:
        // new_modulus_mask = (2 * modulus) - 1 = (modulus_mask * 2) + 1
        // which can be rewritten in terms of shift and or for faster
        let new_modulus_mask = (self.modulus_mask << 1) | 1;

        // We need to split in half, we need to return half the lines in
        // each child. Therefore we must double the modulus and we need to
        // offset one of the two childs.
        //
        // It can be proven that the following two properties holds:
        // x mod n = (x mod 2*n) \cup (x + n mod 2*n)
        // (x mod 2*n) \cap (x + n mod 2*n) = null
        // |(x mod 2*n)| == |(x + n mod 2*n)
        // So these are two perfect half-splits of the range.
        //
        // # Example:
        // Suppose that we have mod: 4, rem: 1, we will sign with `_` the lines
        // to skip, and with `$` the lines to return:
        //
        // Line idx: 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
        // Father:   _  $  _  _  _  $  _  _  _  $  _  _  _  $  _  _
        // Split1:   _  $  _  _  _  _  _  _  _  $  _  _  _  _  _  _
        // Split2:   _  _  _  _  _  $  _  _  _  _  _  _  _  $  _  _
        //
        // So we get the two childs with:
        // mod 8 = 2 * old_mod, rem 1 = old_rem
        // mod 8 = 2 * old_mod, rem 5 = old_rem + old_modulus
        //
        let new = ParalellLinesProducerWithIndex {
            modulus_mask: new_modulus_mask,
            remainder: self.get_modulus() + self.remainder,
            comment_symbol: self.comment_symbol.clone(),
            depth: self.depth + 1,
            maximal_depth: self.maximal_depth,
            mmap: self.mmap.clone(),
            data: self.data,
            line_count: self.line_count,
        };

        // Update the father modulus so that the lines are equally splitted
        self.modulus_mask = new_modulus_mask;
        self.depth += 1;

        // Returns the two new producers
        (self, Some(new))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
