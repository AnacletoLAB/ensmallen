use super::*;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub(crate) struct ConcurrentCSRBuilder {
    outbounds: Vec<AtomicU64>,
    destinations: Vec<AtomicU32>,
}

impl ConcurrentCSRBuilder {
    pub fn new(edges_number: EdgeT, nodes_number: NodeT) -> Self {
        let outbounds = vec![0; nodes_number as usize + 1];

        let mut destinations = Vec::with_capacity(edges_number as usize);
        unsafe { destinations.set_len(edges_number as usize) };

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
