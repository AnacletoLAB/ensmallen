extern crate graph;

use gpu_models::*;
use graph::test_utilities::*;
use graph::{CSVFileWriter, WalksParameters};

#[test]
fn test_skipgram_on_cora() -> Result<(), GPUError> {
    let mut cora = load_cora();
    cora = cora.sort_by_decreasing_outbound_node_degree();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let embedding_size = 128;
    let walks = WalksParameters::new(128)
        .unwrap()
        .set_iterations(Some(10))
        .unwrap();
    let skipgram = SkipGram::new(Some(embedding_size), Some(walks), Some(10), Some(10)).unwrap();
    let mut embedding = vec![0.0; embedding_size * cora.get_number_of_nodes() as usize];
    skipgram.fit_transform(
        &cora,
        embedding.as_mut_slice(),
        Some(5),
        Some(0.01),
        Some(1024),
        None,
    )?;

    let writer = CSVFileWriter::new("cora_skipgram_embedding.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

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
                        .chain(features.iter().map(|e| e.to_string()))
                        .collect::<Vec<String>>()
                }),
        )
        .unwrap();

    Ok(())
}
