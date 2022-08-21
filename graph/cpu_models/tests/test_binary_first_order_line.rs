extern crate graph;

use cpu_models::*;
use graph::test_utilities::*;
use graph::CSVFileWriter;
use vec_rand::splitmix64;

#[test]
fn test_binary_first_order_line() -> Result<(), String> {
    let mut cora = load_cora();
    cora.enable(Some(true), Some(true), Some(true), Some(false))
        .unwrap();
    let model = BinaryFirstOrderLINE::default();
    let mut random_state = model.get_random_state();

    let mut embedding = (0..(model.get_number_of_words() * (cora.get_number_of_nodes() as usize)))
        .map(|_| {
            random_state = splitmix64(random_state);
            random_state
        })
        .collect::<Vec<u64>>();

    model.fit_transform(&cora, embedding.as_mut_slice())?;

    let writer = CSVFileWriter::new("binary_line_cora.tsv")
        .set_separator(Some('\t'))
        .unwrap()
        .set_header(Some(true))
        .set_verbose(Some(true));

    let embedding_size = model.get_number_of_words();

    writer
        .write_lines(
            Some(cora.get_number_of_nodes() as usize),
            vec!["node_name".to_string()]
                .into_iter()
                .chain((0..(embedding_size * 64)).map(|e| e.to_string()))
                .collect::<Vec<String>>(),
            embedding
                .chunks(embedding_size)
                .zip(cora.get_node_names().into_iter())
                .map(|(features, node_name)| {
                    vec![node_name.to_string()]
                        .into_iter()
                        .chain(
                            features.iter().flat_map(|word| {
                                (0..64).map(move |i| (word & (1 << i) != 0).to_string())
                            }),
                        )
                        .collect::<Vec<String>>()
                }),
        )
        .unwrap();

    Ok(())
}
