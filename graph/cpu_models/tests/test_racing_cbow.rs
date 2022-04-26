extern crate graph;

use cpu_models::*;
use graph::test_utilities::*;
use graph::{CSVFileWriter, EdgeFileReader, Graph, NodeFileReader, WalksParameters};

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_ctd() -> Graph {
    let graph_name = "CTD".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/ctd/edges.tsv")
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
    let nodes_reader = NodeFileReader::new(Some("tests/data/ctd/nodes.tsv".to_owned()))
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_nodes_column(Some("node_name"))
        .unwrap()
        .set_verbose(Some(false))
        .set_node_types_column(Some("node_type"))
        .unwrap();
    Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
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
fn test_racing_cbow_on_cora_logsigmoid() -> Result<(), String> {
    let mut cora = load_cora();
    cora = cora.sort_by_decreasing_outbound_node_degree();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
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
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * cora.get_nodes_number() as usize];
    cbow.fit_transform_racing(&cora, embedding.as_mut_slice(), Some(10), None, None)?;

    let writer = CSVFileWriter::new("cora_racing_cbow_embedding_logsigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(cora.get_nodes_number() as usize),
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

#[test]
fn test_racing_cbow_on_cora_sigmoid() -> Result<(), String> {
    let mut cora = load_cora();
    cora = cora.sort_by_decreasing_outbound_node_degree();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
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
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * cora.get_nodes_number() as usize];
    cbow.fit_transform_racing(&cora, embedding.as_mut_slice(), Some(10), None, None)?;

    let writer = CSVFileWriter::new("cora_racing_cbow_embedding_sigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(cora.get_nodes_number() as usize),
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

#[test]
fn test_racing_cbow_on_ctd_logsigmoid() -> Result<(), String> {
    let mut ctd = load_ctd();
    ctd = ctd.sort_by_decreasing_outbound_node_degree();
    ctd.enable(Some(true), Some(true), Some(true), Some(false))
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
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * ctd.get_nodes_number() as usize];
    cbow.fit_transform_racing(&ctd, embedding.as_mut_slice(), Some(10), None, None)?;

    let writer = CSVFileWriter::new("ctd_racing_cbow_embedding_logsigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(ctd.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..embedding_size).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(ctd.get_node_names().into_iter())
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
fn test_racing_cbow_on_ctd_sigmoid() -> Result<(), String> {
    let mut ctd = load_ctd();
    ctd = ctd.sort_by_decreasing_outbound_node_degree();
    ctd.enable(Some(true), Some(true), Some(true), Some(false))
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
    )
    .unwrap();
    let mut embedding = vec![0.0; embedding_size * ctd.get_nodes_number() as usize];
    cbow.fit_transform_racing(&ctd, embedding.as_mut_slice(), Some(10), None, None)?;

    let writer = CSVFileWriter::new("ctd_racing_cbow_embedding_sigmoid.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(ctd.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..embedding_size).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(ctd.get_node_names().into_iter())
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
