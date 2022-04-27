extern crate graph;

use cpu_models::*;
use graph::test_utilities::*;
use graph::{CSVFileWriter, EdgeFileReader, Graph, NodeFileReader, WalksParameters};

const EPOCHS: usize = 10;
const EMBEDDING_SIZE: usize = 20;

fn inernal_run_test(graph: &Graph, use_siamese: bool, log_sigmoid: bool) -> Result<(), String> {
    let walks = WalksParameters::new(128)
        .unwrap()
        .set_iterations(Some(16))
        .unwrap();
    
    let cbow = CBOW::new(
        Some(EMBEDDING_SIZE),
        Some(walks),
        Some(4),
        None,
        Some(10),
        Some(log_sigmoid),
        Some(use_siamese),  
    )
    .unwrap();
    let mut embedding = vec![0.0; EMBEDDING_SIZE * graph.get_nodes_number() as usize];
    cbow.fit_transform_racing(&graph, embedding.as_mut_slice(), Some(EPOCHS), None, None)?;

    let result_path = format!(
        "{}_racing_cbow_embedding_{}_{}.tsv",
        graph.get_name(),
        if log_sigmoid {
            "logsigmoid"
        } else {
            "sigmoid"
        },
        if use_siamese {
            "siamese"
        } else {
            "traditional"
        },
    );

    let writer = CSVFileWriter::new(result_path)
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(graph.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..EMBEDDING_SIZE).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(EMBEDDING_SIZE)
                .zip(graph.get_node_names().into_iter())
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
fn test_cbow_on_cora() -> Result<(), String> {
    let mut cora = load_cora();
    cora = cora.sort_by_decreasing_outbound_node_degree();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();

    for use_siamese in &[true, false] {
        for log_sigmoid in &[true, false] {
            inernal_run_test(&cora, *use_siamese, *log_sigmoid)?;
        }
    }

    Ok(())
}

fn inernal_run_test_thread_safe(graph: &Graph, use_siamese: bool, log_sigmoid: bool) -> Result<(), String> {
    let walks = WalksParameters::new(128)
        .unwrap()
        .set_iterations(Some(16))
        .unwrap();
    
    let cbow = CBOW::new(
        Some(EMBEDDING_SIZE),
        Some(walks),
        Some(4),
        None,
        Some(10),
        Some(log_sigmoid),
        Some(use_siamese),  
    )
    .unwrap();
    let mut embedding = vec![0.0; EMBEDDING_SIZE * graph.get_nodes_number() as usize];
    cbow.fit_transform(&graph, embedding.as_mut_slice(), Some(EPOCHS), None, Some(64), None)?;

    let result_path = format!(
        "{}_thread_safe_cbow_embedding_{}_{}.tsv",
        graph.get_name(),
        if log_sigmoid {
            "logsigmoid"
        } else {
            "sigmoid"
        },
        if use_siamese {
            "siamese"
        } else {
            "traditional"
        },
    );

    let writer = CSVFileWriter::new(result_path)
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(graph.get_nodes_number() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..EMBEDDING_SIZE).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(EMBEDDING_SIZE)
                .zip(graph.get_node_names().into_iter())
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

//#[test]
fn test_cbow_on_cora_thread_safe() -> Result<(), String> {
    let mut cora = load_cora();
    cora = cora.sort_by_decreasing_outbound_node_degree();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();

    for use_siamese in &[true, false] {
        for log_sigmoid in &[true, false] {
            inernal_run_test_thread_safe(&cora, *use_siamese, *log_sigmoid)?;
        }
    }

    Ok(())
}