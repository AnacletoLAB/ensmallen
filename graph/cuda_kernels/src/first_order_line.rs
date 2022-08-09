use core::intrinsics::wrapping_mul;
use vec_rand::splitmix64::splitmix64;

use crate::*;

#[no_mangle]
/// Computes a First-order LINE embedding.
///
/// # Arguments
///
pub unsafe extern "ptx-kernel" fn compute_first_order_line(
    embedding: *mut f32,
    node_degrees: *const NodeT,
    destinations: *const NodeT,
    learning_rate: f32,
    mut random_seed: u64,
    embedding_size: usize,
    number_of_nodes: usize,
    number_of_edges: usize,
    number_of_epochs: usize,
) {
    random_state = (block_idx_x() as u64)
        .wrapping_mul(block_dim_x() as u64)
        .wrapping_mul(thread_idx_x() as u64)
        .wrapping_mul(random_state);

    let embedding = core::slice::from_raw_parts_mut(
        embedding, 
        number_of_nodes * embedding_size
    );

    let node_degrees = core::slice::from_raw_parts_mut(
        node_degrees, 
        number_of_nodes
    );

    let destinations = core::slice::from_raw_parts_mut(
        destinations, 
        number_of_edges
    );
    
    (0..number_of_epochs).for_each(|_|{
        (0..number_of_edges).for_each(|edge_number|{

        });
    });
}
