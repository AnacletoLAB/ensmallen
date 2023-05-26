use super::*;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub(crate) struct ConcurrentCSRBuilder {
    outbounds: Vec<AtomicU64>,
    destinations: Vec<AtomicU32>,
}

impl ConcurrentCSRBuilder {
    pub fn new(number_of_edges: EdgeT, number_of_nodes: NodeT) -> Self {
        let outbounds = vec![0; number_of_nodes as usize + 1];

        let mut destinations = Vec::with_capacity(number_of_edges as usize);
        unsafe { destinations.set_len(number_of_edges as usize) };

        Self {
            outbounds: unsafe { core::mem::transmute::<Vec<EdgeT>, Vec<AtomicU64>>(outbounds) },
            destinations: unsafe {
                core::mem::transmute::<Vec<NodeT>, Vec<AtomicU32>>(destinations)
            },
        }
    }

    /// this assumes that is always called correctly
    pub fn set(&self, index: EdgeT, src: NodeT, dst: NodeT) {
        self.outbounds[1 + src as usize].fetch_max(1 + index, Ordering::Relaxed);
        self.destinations[index as usize].store(dst, Ordering::Relaxed);
    }

    pub fn build(self) -> CSR {
        // TODO!: parallellize this stuff
        let mut outbounds_degrees =
            unsafe { core::mem::transmute::<Vec<AtomicU64>, Vec<EdgeT>>(self.outbounds) };

        // fill singletons
        let mut previous = 0;
        outbounds_degrees.iter_mut().for_each(|x| {
            if *x == 0 {
                *x = previous;
            } else {
                previous = *x;
            }
        });

        CSR {
            outbounds_degrees,
            destinations: unsafe {
                core::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(self.destinations)
            },
            sources: None,
        }
    }
}
