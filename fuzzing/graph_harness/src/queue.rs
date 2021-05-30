use super::*;
use arbitrary::Arbitrary;
use graph::DijkstraQueue;

#[derive(Arbitrary, Debug, Clone)]
pub struct QueueParams{
    vals: Vec<(u8, f64)>,
    root: u8,
}

const NODES_NUMBER: usize = 16;

pub fn queue_harness(data: QueueParams){

    if data.vals.is_empty() {
        return;
    }  

    if data.root as usize > data.vals.len() {
        return;
    }

    let mut queue = DijkstraQueue::with_capacity_from_root(NODES_NUMBER, data.root as usize % NODES_NUMBER);

    for val in data.vals {
        if val.1.is_finite() && val.1 > 0.0 {
            queue.push(val.0 as usize % NODES_NUMBER, val.1.abs());
        }
    }
}
