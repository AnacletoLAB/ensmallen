struct DenseMatrix3D {
    data: Vec<f32>,
    batch_size: usize,
    n_rows: usize,
    n_cols: usize,
}

impl DenseMatrix3D {
    fn zeros(batch_size: usize, n_rows: usize, n_cols: usize) {
        DenseMatrix {
            data: vec![0; batch_size * n_rows * n_cols],
            batch_size,
            n_cols,
            n_rows,
        }
    }
}

struct DenseMatrix2D {
    data: Vec<f32>,
    n_rows: usize,
    n_cols: usize,
}

impl DenseMatrix2D {
    fn zeros(n_rows: usize, n_cols: usize) {
        DenseMatrix {
            data: vec![0; n_rows * n_cols],
            n_cols,
            n_rows,
        }
    }
    fn dot(&self, features: &Matrix, result: &mut Matrix);
    fn iter_row(&self) -> impl Iterator<Item = f32>;
    fn add_vector_inplace(&mut self, vector: &Vector);
}

trait Layer {
    fn get_input_size(&self) -> usize;
    fn get_output_size(&self) -> usize;

    fn get_gradient_size(&self) -> usize {
        self.get_input_size() * self.get_output_size()
    }

    fn forward(
        &self,
        input: &DenseMatrix2D,
        activation_output: &mut DenseMatrix2D,
    ) -> &DenseMatrix2D;
    fn backward(
        &self,
        input: &DenseMatrix2D,
        activation: &DenseMatrix2D,
        gradient: &mut DenseMatrix3D,
    ) -> &DenseMatrix3D;

    fn on_epoch_start(&mut self);
    fn on_epoch_end(&mut self);
}

struct GCNLayer {
    kernel: LaplacianLayer,

    alpha: f32,
    dropout_mask: Vec<bool>,

    weights: DenseMatrix2D,

    activation: Activation,
}

impl<K> GCNLayer<K> {
    fn forward(
        &self,
        kernel: K,
        node_features: &Matrix,
        activation_output: &mut DenseMatrix2D,
    ) -> &DenseMatrix2D {
        activation_output
            .par_iter_rows_mut()
            .enumerate()
            .for_each(|(node_id, activation_row)| {

                activation_row.fill(0);

                let neighbours_count = graph.iter_neightbours_node_ids_from_node_id(node_id)
                    .chain([node_id].iter())
                    .map(|neighbour_id| {
                        node_features.get_row(neighbour_id).zip(self.dropout_mask.iter())
                            .filter_map(|(feature, mask_value)| {
                                if mask_value {
                                    None
                                } else {
                                    Some(feature)
                                }
                            })
                            .for_each(|mut feature| {

                                feature *= kernel.get_score(node_id, neighbour_id);

                                self.weights.iter_rows()
                                    .zip(activation_row.iter())
                                    .for_each(|(weight, activation_dst)| {
                                        activation_dst += feature * weight;
                                    });
                            });
                        1
                    }).sum::<usize>();

                    let inverse_neighbours_count = 1.0 / neighbours_count as f32;

                    activation_row.iter_mut().for_each(|feature| {
                        feature *= inverse_neighbours_count;
                    })
            });

        activation_output
    }
}

struct LaplacianLayer {
    graph: &Graph,
    normalized_degrees: Vec<f32>,
}

impl LaplacianLayer {
    fn new(graph: &Graph) -> Self {
        let mut normalized_degrees = Vec::with_capacity(graph.get_nodes_number() as usize);
        graph
            .par_iter_nodes_degres()
            .map(|degree| 1.0 / degree.sqrt())
            .collect_into_vec(&mut normalized_degrees);
        LaplacianLayer {
            graph,
            normalized_degrees,
        }
    }

    fn get_score(&self, src: NodeT, dst: NodeT) -> f32 {
        if src == dst {
            1.0
        } else {
            self.normalized_degrees[src as usize] * self.normalized_degrees[dst as usize]
        }
    }
}

enum Activation {
    ReLU,
    Sigmoid,
    Softmax,
}

struct NodeLabelGCN {}

struct SequentialModel {
    layers: Vec<GCNLayer>,
}

impl SequentialModel {
    fn set_graph(&mut self, graph: &Graph) {
        for layer in layers {
            if let Laplacian(laplacian) = layer {
                laplacian.set_graph(graph);
            }
        }
    }

    fn predict(&self, graph: &Graph, node_features: &Matrix) {
        let kernel = Kernel::new(graph);
        let mut tmp = node_features;
        for layer in layers {
            tmp = layer.forward(tmp);
        }
    }

    fn fit(&mut self, graph: &Graph, node_features: &Matrix) {
        let kernel = Kernel::new(graph);
        let batch_size = graph.get_nodes_number();

        let activations = layers
            .iter()
            .map(|layer| DenseMatrix2D::zeros(batch_size, layer.get_output_size()))
            .collect::<Vec<DenseMatrix2D>>();

        let gradients = layers
            .iter()
            .map(|layer| {
                DenseMatrix3D::zeros(batch_size, layer.get_input_size(), layer.get_output_size())
            })
            .collect::<Vec<DenseMatrix3D>>();

        let mut tmp = node_features;
        for layer in layers {
            tmp = layer.forward(kernel, tmp);
        }

        (score, tmp) = loss(tmp, y);

        for layer in layers.rev() {
            tmp = layer.backward(kernel, tmp);
        }
    }
}
