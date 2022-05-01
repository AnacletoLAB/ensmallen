extern crate graph;

use cpu_models::*;
use graph::{CSVFileWriter, EdgeFileReader, Graph, WalksParameters};

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_kgmicrobe() -> Graph {
    let graph_name = "KGMicrobe".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/kgmicrobe/edges.tsv")
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_verbose(Some(false))
        .set_sources_column(Some("subject"))
        .unwrap()
        .set_destinations_column(Some("object"))
        .unwrap()
        .set_edge_types_column(Some("edge_type"))
        .unwrap();

    Graph::from_file_readers(
        Some(edges_reader),
        None,
        None,
        None,
        true,
        true,
        false,
        graph_name.clone(),
    )
    .unwrap()
}

#[test]
fn test_racing_cbow_on_kgmicrobe_logsigmoid() -> Result<(), String> {
    let mut kgmicrobe = load_kgmicrobe();
    kgmicrobe
        .enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let embedding_size = 128;
    let walks = WalksParameters::new(128)
        .unwrap()
        .set_iterations(Some(10))
        .unwrap();
    let cbow = CBOW::new(
        Some(embedding_size),
        Some(walks),
        Some(10),
        None,
        Some(10),
        Some(true),
        Some(false),
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * kgmicrobe.get_nodes_number() as usize];
    cbow.fit_transform_racing(&kgmicrobe, embedding.as_mut_slice(), Some(10), None, None, None)?;

    let writer = CSVFileWriter::new("kgmicrobe_racing_cbow_embedding_logsigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(kgmicrobe.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..embedding_size).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(kgmicrobe.get_node_names().into_iter())
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

#[test]
fn test_racing_cbow_on_kgmicrobe_sigmoid() -> Result<(), String> {
    let mut kgmicrobe = load_kgmicrobe();
    kgmicrobe
        .enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let embedding_size = 128;
    let walks = WalksParameters::new(128)
        .unwrap()
        .set_iterations(Some(10))
        .unwrap();
    let cbow = CBOW::new(
        Some(embedding_size),
        Some(walks),
        Some(10),
        None,
        Some(10),
        Some(false),
        Some(false),
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * kgmicrobe.get_nodes_number() as usize];
    cbow.fit_transform_racing(&kgmicrobe, embedding.as_mut_slice(), Some(10), None, None, None)?;

    let writer = CSVFileWriter::new("kgmicrobe_racing_cbow_embedding_sigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(kgmicrobe.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..embedding_size).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(kgmicrobe.get_node_names().into_iter())
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
