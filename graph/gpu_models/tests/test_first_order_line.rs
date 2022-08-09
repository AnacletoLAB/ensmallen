extern crate graph;

use gpu_models::*;
use graph::test_utilities::*;
use cpu_models::GraphEmbedder;
use graph::{CSVFileWriter, WalksParameters};

#[test]
fn test_skipgram_on_cora() -> Result<(), GPUError> {
    let mut cora = load_cora();
    let line = FirstOrderLINE::default();
    let mut embedding = vec![0.0; embedding_size * cora.get_number_of_nodes() as usize];
    line.fit_transform(&cora, &mut [embedding.as_mut_slice()])?;

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
                .chain((0..line.get_embedding_size()).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(line.get_embedding_size())
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
