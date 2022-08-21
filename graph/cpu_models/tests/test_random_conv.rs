extern crate graph;

use cpu_models::*;
use express_measures::normalize_vector_inplace;
use graph::test_utilities::*;
use rayon::prelude::*;
use graph::CSVFileWriter;
use vec_rand::splitmix64;

#[test]
fn test_random_graph_convolution_embedding() -> Result<(), String> {
    let mut cora = load_cora();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let model = RandomGraphConvolutionEmbedding::default();
    let mut random_state = model.get_random_state();

    let mut embedding = (0..(model.get_embedding_size() * (cora.get_number_of_nodes() as usize)))
        .map(|i| {
            random_state = splitmix64(random_state.wrapping_mul(i as u64 + 1));
            random_state as f32 / u64::MAX as f32
        })
        .collect::<Vec<f32>>();

    embedding
        .par_chunks_mut(model.get_embedding_size())
        .for_each(|row| {
            normalize_vector_inplace(row);
        });

    model.fit_transform(&cora, embedding.as_mut_slice())?;

    let writer = CSVFileWriter::new("random_conv.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    let embedding_size = model.get_embedding_size();

    writer
        .write_lines(
            Some(cora.get_number_of_nodes() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..embedding_size).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(cora.get_node_names().into_iter())
                .map(|(features, node_name)| {
                    vec![node_name.to_string()]
                        .into_iter()
                        .chain(features.iter().map(|feature| feature.to_string()))
                        .collect::<Vec<String>>()
                }),
        )
        .unwrap();

    Ok(())
}
