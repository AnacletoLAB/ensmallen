use crate::*;

#[no_mangle]
/// Compute the CBOW mini-batch and updates the embedding and hidden layer.
///
/// # Arguments
///
pub unsafe extern "ptx-kernel" fn compute_cbow_mini_batch(
    embedding: *mut f32,
    hidden: *mut f32,
    random_walks: *const u32,
    negative_node_ids: *const u32,
    learning_rate: f32,
    window_size: isize,
    number_of_negative_samples: usize,
    random_walk_length: usize,
    embedding_size: usize,
    vocabulary_size: usize,
    batch_size: usize,
    iterations: usize,
) {
    let random_walk_number =
        block_idx_x() as usize * block_dim_x() as usize + thread_idx_x() as usize;

    // Embedding has shape (vocabulary_size, embedding_size)
    let embedding = core::slice::from_raw_parts_mut(embedding, vocabulary_size * embedding_size);
    // Hidden layer has shape (vocabulary_size, embedding_size), same as embedding
    let hidden = core::slice::from_raw_parts_mut(hidden, vocabulary_size * embedding_size);
    // Random walks matrix has shape (batch_size * iterations, random_walk_length)
    let number_of_random_walks = batch_size * iterations;
    let number_of_contexts_per_random_walk =
        (random_walk_length as isize - window_size * 2) as usize;
    let random_walks =
        core::slice::from_raw_parts(random_walks, number_of_random_walks * random_walk_length);
    // Negative node IDs have shape (batch_size * iterations * (random_walk_length - window_size * 2), )
    let negative_node_ids = core::slice::from_raw_parts(
        negative_node_ids,
        number_of_negative_samples
            * batch_size
            * iterations
            * (random_walk_length as isize - window_size * 2) as usize,
    );

    // We iterate for all skipgram batches of the random walk.
    for center in window_size..(random_walk_length as isize - window_size) {
        // BEGIN THE CONTRACTION STEP FOR THIS BATCH

        // We retrieve the value of the current central node ID
        let current_central_node_id =
            random_walks[random_walk_number * random_walk_length + center as usize] as usize;
        // and we retrieve its embedding
        let current_central_node_embedding = &mut hidden[current_central_node_id * embedding_size
            ..(current_central_node_id + 1) * embedding_size];
        // We iterate on the context around the center
        let mut dot: f32 = 0.0;
        // We compute the dot product of the sum of the contextual node embedding and the
        // current central node.
        for context in (-window_size..0).chain(1..window_size + 1) {
            let current_context_node_id = random_walks
                [random_walk_number * random_walk_length + (center + context) as usize]
                as usize;
            let current_context_node_embedding = &mut embedding[current_context_node_id
                * embedding_size
                ..(current_context_node_id + 1) * embedding_size];
            for feature in 0..embedding_size {
                dot += current_central_node_embedding[feature]
                    * current_context_node_embedding[feature];
            }
        }
        // We compute the exponentiation of the dot product.
        let exponentiated_dot = dot.exp2();
        // We compute the loss for the POSITIVE node
        let loss = (1.0 - (exponentiated_dot / (exponentiated_dot + 1.0))) * learning_rate;

        // We backpropagate the loss to the hidden layer and the embeddding layer
        // for context in (-window_size..0).chain(1..window_size + 1) {
        //     let current_context_node_id = random_walks
        //         [random_walk_number * random_walk_length + (center + context) as usize]
        //         as usize;
        //     let current_context_node_embedding = &mut embedding[current_context_node_id
        //         * embedding_size
        //         ..(current_context_node_id + 1) * embedding_size];
        //     for feature in 0..embedding_size {
        //         current_central_node_embedding[feature] +=
        //             current_context_node_embedding[feature] * loss;
        //         current_context_node_embedding[feature] +=
        //             current_central_node_embedding[feature] * loss;
        //     }
        // }

        // BEGIN THE RELAXATION STEP FOR THE NEGATIVES OF THIS CONTEXT

        // let start_negatives = (number_of_contexts_per_random_walk * random_walk_number
        //     + center as usize) as usize
        //     * number_of_negative_samples;
        // let end_negatives = (number_of_contexts_per_random_walk * random_walk_number
        //     + center as usize
        //     + 1) as usize
        //     * number_of_negative_samples;
        // for negative_number in start_negatives..end_negatives {
        //     // We retrieve the value of the current central node ID
        //     let current_negative_node_id = negative_node_ids
        //         [random_walk_number * random_walk_length + negative_number + center as usize]
        //         as usize;
        //     // and we retrieve its embedding
        //     let current_negative_node_embedding = &mut hidden[current_negative_node_id
        //         * embedding_size
        //         ..(current_negative_node_id + 1) * embedding_size];

        //     // We iterate on the context around the center
        //     let mut dot: f32 = 0.0;
        //     // We compute the dot product of the sum of the contextual node embedding and the
        //     // current central node.
        //     for context in (-window_size..0).chain(1..window_size + 1) {
        //         let current_context_node_id = random_walks
        //             [random_walk_number * random_walk_length + (center + context) as usize]
        //             as usize;
        //         let current_context_node_embedding = &mut embedding[current_context_node_id
        //             * embedding_size
        //             ..(current_context_node_id + 1) * embedding_size];
        //         for feature in 0..embedding_size {
        //             dot += current_negative_node_embedding[feature]
        //                 * current_context_node_embedding[feature];
        //         }
        //     }
        //     // We compute the exponentiation of the dot product.
        //     let exponentiated_dot = dot.exp2();
        //     // We compute the loss for the NEGATIVE node
        //     let loss = (exponentiated_dot / (exponentiated_dot + 1.0)) * learning_rate;

        //     // We backpropagate the loss to the hidden layer and the embeddding layer
        //     for context in (-window_size..0).chain(1..window_size + 1) {
        //         let current_context_node_id = random_walks
        //             [random_walk_number * random_walk_length + (center + context) as usize]
        //             as usize;
        //         let current_context_node_embedding = &mut embedding[current_context_node_id
        //             * embedding_size
        //             ..(current_context_node_id + 1) * embedding_size];
        //         for feature in 0..embedding_size {
        //             current_negative_node_embedding[feature] -=
        //                 current_context_node_embedding[feature] * loss;
        //             current_context_node_embedding[feature] -=
        //                 current_negative_node_embedding[feature] * loss;
        //         }
        //     }
        // }
    }
}
