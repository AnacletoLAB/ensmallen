extern crate graph;

use gpu_models::*;
use graph::*;

#[test]
fn test_cbow_on_cora() -> Result<(), GPUError> {
    let mut cora = load_cora();
    let cbow = CBOW::new(Some(128), None, Some(10), Some(5)).unwrap();
    let embedding = vec![0.0; 128 * cora.get_nodes_number() as usize];
    cbow.fit_transform(graph, embedding, Some(10), None, Some(32), None);
    Ok(())
}
