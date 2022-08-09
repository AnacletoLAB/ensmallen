use crate::*;
use core::intrinsics::unlikely;
use vec_rand::{splitmix64, xorshift};

#[no_mangle]
/// Computes a First-order LINE embedding.
///
/// # Arguments
/// * `embedding`: *mut f32 - The raw pointer to the memory area of the embedding.
/// * `comulative_node_degrees`: *const u64 - The raw pointer to the constant memory of the comulative node degrees.
/// * `destinations`: *const u32 - The raw pointer to the constant memory of the node destinations.
/// * `learning_rate`: f32 - The learning rate to use. We expect something in the order of `0.01`.
/// * `mut random_state`: u64 - The random state to reproduce the embedding.
/// * `embedding_size`: usize - The dimensionality of the embedding.
/// * `number_of_nodes`: usize - The number of nodes, equal to the length of the provided `comulative_node_degrees`.
/// * `number_of_edges`: usize - The number of edges, equal to the length of the provided `destinations`.
///
/// # Safety
/// The function will expect that the provided values are EXACTLY of the correct shape.
pub unsafe extern "ptx-kernel" fn compute_first_order_line(
    embedding: *mut f32,
    comulative_node_degrees: *const u64,
    destinations: *const u32,
    learning_rate: f32,
    mut random_state: u64,
    embedding_size: usize,
    number_of_nodes: usize,
    number_of_edges: usize,
) {
    random_state = (thread_idx_x() as u64).wrapping_mul(random_state);

    let embedding = core::slice::from_raw_parts_mut(embedding, number_of_nodes * embedding_size);

    let node_degrees = core::slice::from_raw_parts(comulative_node_degrees, number_of_nodes);

    let destinations = core::slice::from_raw_parts(destinations, number_of_edges);

    let batch_size = (number_of_edges / block_dim_x() as usize).max(1);

    let get_node_degree = |node_id: usize| {
        let comulative_degree = node_degrees[node_id];
        // let previous_comulative_degree =
        //     ((node_id == 0) as u64).wrapping_sub(1) & node_degrees[node_id - 1];
        let previous_comulative_degree = if node_id == 0 {
            0
        } else {
            node_degrees[node_id - 1]
        };
        let degree = comulative_degree - previous_comulative_degree;
        (previous_comulative_degree, degree)
    };

    (0..batch_size).for_each(|edge_number| {
        let mut random_state =
            xorshift((edge_number as u64 + random_state).wrapping_mul(random_state));
        let src = random_state as usize % number_of_nodes;

        random_state = splitmix64(random_state);
        let (previous_comulative_degree, src_degree) = get_node_degree(src);
        if unlikely(src_degree == 0) {
            return;
        }
        let true_dst = destinations
            [(previous_comulative_degree + (xorshift(random_state) % src_degree)) as usize]
            as usize;

        random_state = splitmix64(random_state);
        let false_dst = destinations[xorshift(random_state) as usize % number_of_edges] as usize;

        if unlikely(true_dst == false_dst) {
            return;
        };

        let (true_dot, false_dot, src_squared, true_dst_squared, false_dst_squared) = embedding
            [(embedding_size * src)..(embedding_size * (src + 1))]
            .iter()
            .zip(
                embedding[(embedding_size * true_dst)..(embedding_size * (true_dst + 1))]
                    .iter()
                    .zip(
                        embedding[(embedding_size * false_dst)..(embedding_size * (false_dst + 1))]
                            .iter(),
                    ),
            )
            .map(|(&src_value, (&true_dst_value, &false_dst_value))| {
                (
                    src_value * true_dst_value,
                    src_value * false_dst_value,
                    src_value * src_value,
                    true_dst_value * true_dst_value,
                    false_dst_value * false_dst_value,
                )
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4))
            .unwrap();

        let src_norm = src_squared.sqrt();
        let true_dst_norm = true_dst_squared.sqrt();
        let false_dst_norm = false_dst_squared.sqrt();
        let true_cosine_similarity = true_dot / (src_norm * true_dst_norm);
        let false_cosine_similarity = false_dot / (src_norm * false_dst_norm);
        let true_variation = 1.0 / (1.0 + (-true_cosine_similarity).exp2()) - 1.0;
        let false_variation = 1.0 / (1.0 + (-false_cosine_similarity).exp2());
        let (_, true_dst_degree) = get_node_degree(true_dst);
        let (_, false_dst_degree) = get_node_degree(false_dst);

        let src_prior = learning_rate * (number_of_nodes as f32 / (src_degree as f32 + 1.0));
        let src_true_variation = true_variation * src_prior;
        let src_false_variation = false_variation * src_prior;
        let true_dst_variation = true_variation
            * (number_of_nodes as f32 / (true_dst_degree as f32 + 1.0))
            * learning_rate;
        let false_dst_variation = false_variation
            * (number_of_nodes as f32 / (false_dst_degree as f32 + 1.0))
            * learning_rate;

        (0..embedding_size)
            .zip((0..embedding_size).zip(0..embedding_size))
            .for_each(|(i, (j, k))| {
                embedding[src * embedding_size + i] /= src_norm;
                embedding[true_dst * embedding_size + j] /= true_dst_norm;
                embedding[false_dst * embedding_size + k] /= false_dst_norm;
                let src_value = embedding[src * embedding_size + i];
                let true_dst_value = embedding[true_dst * embedding_size + j];
                let false_dst_value = embedding[false_dst * embedding_size + k];
                embedding[src * embedding_size + i] -=
                    src_true_variation * true_dst_value + src_false_variation * false_dst_value;
                embedding[true_dst * embedding_size + j] -= true_dst_variation * src_value;
                embedding[false_dst * embedding_size + k] -= false_dst_variation * src_value;
            });
    });
}
