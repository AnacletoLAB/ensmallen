use super::*;
use std::sync::atomic::{AtomicU8, Ordering};
use rayon::prelude::*;

const WORD_SIZE: usize = 1;
const WORD_SHIFT: usize = 8;

#[derive(Debug)]
/// A bitvector that can safely be built in parallel
pub struct ConcurrentBitVec {
    pub(crate) bitmap: Vec<AtomicU8>,
    len: usize,
}

pub struct ConcurrentBitVecOnesIterator<'a> {
    father: &'a ConcurrentBitVec,
    code: u8,
    index: usize,
}

impl<'a> ConcurrentBitVecOnesIterator<'a> {
    fn new(father: &'a ConcurrentBitVec) -> ConcurrentBitVecOnesIterator<'a>{
        ConcurrentBitVecOnesIterator{
            code: father.bitmap[0].load(Ordering::SeqCst),
            father,
            index: 0,
        }
    }
}

impl<'a> Iterator for ConcurrentBitVecOnesIterator<'a> {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.code == 0 {
            self.index += 1;
            if self.index >= self.father.bitmap.len() {
                return None;
            }
            self.code = self.father.bitmap[self.index].load(Ordering::SeqCst);
        }

        let t = self.code.trailing_zeros() as usize;

        // clean the curret lowest setted bit
        self.code &= self.code - 1;

        Some((self.index << 3) + t)
    }
}

impl ConcurrentBitVec {
    /// Create a new bitvec with the given size
    pub fn with_capacity(capacity: usize) -> Self {
        ConcurrentBitVec {
            bitmap: vec_atomic![AtomicU8; 0; (capacity >> 3) + 1],
            len: capacity,
        }
    }
    
    /// Set to 1 the bit of index `index`
    pub fn set(&self, index: usize) {
        let word_id = index >> 3;
        self.bitmap[word_id].fetch_or(1 << (index & 7), Ordering::SeqCst);
    }

    /// Set to 0 the bit of index `index`
    pub fn clear(&self, index: usize) {
        let word_id = index >> 3;
        self.bitmap[word_id].fetch_and(!(1 << (index & 7)), Ordering::SeqCst);
    }

    /// Returns an iterator over the indices of all the bits set to 1.
    pub fn iter_ones(&self) -> ConcurrentBitVecOnesIterator {
        ConcurrentBitVecOnesIterator::new(self)
    }

    /// Returns the number of ones in the bitvector
    pub fn count_ones(&self) -> usize {
        self.bitmap.par_iter().map(
            |x| x.load(Ordering::SeqCst).count_ones() as usize
        ).sum::<usize>() 
    }

    /// Returns the number of zeros in the bitvector
    pub fn count_zeros(&self) -> usize {
        self.len - self.count_ones()
    }
}

