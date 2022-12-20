use super::*;
use core::sync::atomic::{Ordering, AtomicU64, AtomicU32};

pub(crate) struct ConcurrentCSRBuilder {
    outbounds: Vec<AtomicU64>,
    destinations: Vec<AtomicU32>,
}

impl ConcurrentCSRBuilder {
    pub fn new(edges_number: EdgeT, nodes_number: NodeT) -> Self {
        let outbounds = vec![0; nodes_number as usize + 1];

        let mut destinations = Vec::with_capacity(edges_number as usize);
        unsafe{destinations.set_len(edges_number as usize)};

        Self {
            outbounds:unsafe{core::mem::transmute::<Vec<EdgeT>, Vec<AtomicU64>>(outbounds)},
            destinations: unsafe{core::mem::transmute::<Vec<NodeT>, Vec<AtomicU32>>(destinations)},
        }
    }

    /// this assumes that is always called correctly
    pub fn set(&self, index: EdgeT, src: NodeT, dst: NodeT) {
        self.destinations[index as usize].store(dst, Ordering::Relaxed);
        self.outbounds[1 + src as usize].fetch_max(index, Ordering::Relaxed);
    }

    pub fn build(self) -> CSR {
        CSR{
            outbounds_degrees: unsafe{core::mem::transmute::<Vec<AtomicU64>, Vec<EdgeT>>(self.outbounds)},
            destinations: unsafe{core::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(self.destinations)},
            sources: None,
        }
    } 
}