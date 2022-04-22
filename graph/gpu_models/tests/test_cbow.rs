extern crate graph;

use gpu_models::*;
use graph::test_utilities::*;
use graph::CSVFileWriter;

#[test]
fn test_cbow_on_cora() -> Result<(), GPUError> {
    let mut cora = load_cora();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let cbow = CBOW::new(Some(128), None, Some(10), Some(5)).unwrap();
    let embedding_size = 128;
    let mut embedding = vec![0.0; embedding_size * cora.get_nodes_number() as usize];
    cbow.fit_transform(
        &cora,
        embedding.as_mut_slice(),
        Some(10),
        None,
        Some(1024),
        None,
    )?;

    let writer = CSVFileWriter::new("cora_embedding.tsv")
        .set_separator(Some("\t"))?
        .set_header(Some(true))
        .set_verbose(Some(true));

    writer
        .write_lines(
            Some(cora.get_nodes_number() as usize),
            (0..embedding_size)
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            embedding.chunks(128).map(|features| {
                features
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
            }),
        )
        .unwrap();

    Ok(())
}
