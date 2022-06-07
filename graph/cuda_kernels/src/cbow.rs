use crate::*;

#[no_mangle]
/// Compute the CBOW mini-batch and updates the embedding and hidden layer.
///
/// # Arguments
///
pub unsafe extern "ptx-kernel" fn compute_cbow_mini_batch(
    embedding: *mut f32,
    total_contexts: *mut f32,
    contexts_gradient: *mut f32,
    random_walks: *const u32,
    negative_node_ids: *const u32,
    learning_rate: f32,
    window_size: usize,
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
    // Random walks matrix has shape (batch_size * iterations, random_walk_length)
    let number_of_random_walks = batch_size * iterations;
    let number_of_contexts_per_random_walk = (random_walk_length - window_size * 2) as usize;
    let random_walks =
        core::slice::from_raw_parts(random_walks, number_of_random_walks * random_walk_length);
    let total_contexts =
        core::slice::from_raw_parts_mut(total_contexts, number_of_random_walks * embedding_size);
    let contexts_gradient =
        core::slice::from_raw_parts_mut(contexts_gradient, number_of_random_walks * embedding_size);

    let random_walk = &random_walks
        [random_walk_number * random_walk_length..(random_walk_number + 1) * random_walk_length];

    let total_contexts = &mut total_contexts
        [random_walk_number * embedding_size..(random_walk_number + 1) * embedding_size];

    let contexts_gradient = &mut contexts_gradient
        [random_walk_number * embedding_size..(random_walk_number + 1) * embedding_size];

    // Negative node IDs have shape (batch_size * iterations * (random_walk_length - window_size * 2), )
    let negative_node_ids = core::slice::from_raw_parts(
        negative_node_ids,
        number_of_negative_samples
            * batch_size
            * iterations
            * (random_walk_length - window_size * 2) as usize,
    );

    let negative_node_ids = &negative_node_ids[random_walk_number
        * number_of_contexts_per_random_walk
        * number_of_negative_samples
        ..(random_walk_number + 1)
            * number_of_contexts_per_random_walk
            * number_of_negative_samples];

    let scale_factor = (embedding_size as f32).sqrt();
    let context_size = (window_size * 2) as f32;

    // Create the closure to apply a gradient to a provided node's embedding
    let weighted_vector_sum = |vector: &mut [f32], variation: &[f32], weight: f32| {
        vector.iter_mut().zip(variation.iter().cloned()).for_each(
            |(feature, gradient_feature): (&mut f32, f32)| {
                *feature += weight * gradient_feature;
            },
        );
    };

    let compute_mini_batch_step = |total_context_embedding: &[f32],
                                   context_embedding_gradient: &mut [f32],
                                   node_embedding: &mut [f32],
                                   label: f32| {
        let dot = node_embedding
            .iter()
            .cloned()
            .zip(total_context_embedding.iter().cloned())
            .map(|(node_feature, contextual_feature)| node_feature * contextual_feature)
            .sum::<f32>()
            / context_size
            / scale_factor;

        if dot > 20.0 || dot < -20.0 {
            return;
        }

        let exp_dot = dot.exp2();
        let loss = (label - exp_dot / ((exp_dot + 1.0) * (exp_dot + 1.0))) * learning_rate;

        weighted_vector_sum(node_embedding, total_context_embedding, loss / context_size);
        weighted_vector_sum(context_embedding_gradient, node_embedding, loss);
    };

    // We start to compute the new gradients.
    (window_size..random_walk_length - window_size)
        .map(|central_index| {
            (
                &random_walk[(central_index - window_size)..central_index],
                &random_walk[(central_index + 1)..(central_index + window_size)],
                random_walk[central_index],
            )
        })
        .zip(negative_node_ids.chunks(number_of_negative_samples))
        .for_each(
            |((left_context, right_context, central_node_id), negative_node_ids)| {
                // We compute the total context embedding.
                total_contexts.iter_mut().for_each(|value| {
                    *value = 0.0;
                });

                contexts_gradient.iter_mut().for_each(|value| {
                    *value = 0.0;
                });

                // Then we sum over it the other values.
                left_context
                    .iter()
                    .chain(right_context.iter())
                    .for_each(|contextual_node_id| {
                        let contextual_node_id = *contextual_node_id as usize;
                        embedding[(contextual_node_id * embedding_size)
                            ..((contextual_node_id + 1) * embedding_size)]
                            .iter()
                            .zip(total_contexts.iter_mut())
                            .for_each(|(feature, total_feature)| {
                                *total_feature += *feature;
                            });
                    });

                // We now compute the gradient relative to the positive
                compute_mini_batch_step(
                    total_contexts,
                    contexts_gradient,
                    &mut embedding[((central_node_id as usize) * embedding_size)
                        ..(((central_node_id as usize) + 1) * embedding_size)],
                    1.0,
                );

                // We compute the gradients relative to the negative classes.
                negative_node_ids
                    .iter()
                    .cloned()
                    .filter(|non_central_node_id| *non_central_node_id != central_node_id)
                    .for_each(|non_central_node_id| {
                        compute_mini_batch_step(
                            total_contexts,
                            contexts_gradient,
                            &mut embedding[((non_central_node_id as usize) * embedding_size)
                                ..(((non_central_node_id as usize) + 1) * embedding_size)],
                            0.0,
                        );
                    });
                left_context
                    .iter()
                    .chain(right_context.iter())
                    .cloned()
                    .for_each(|contextual_node_id| {
                        let contextual_node_id = contextual_node_id as usize;
                        weighted_vector_sum(
                            &mut embedding[(contextual_node_id * embedding_size)
                                ..((contextual_node_id + 1) * embedding_size)],
                            contexts_gradient,
                            1.0,
                        );
                    });
            },
        );
}
