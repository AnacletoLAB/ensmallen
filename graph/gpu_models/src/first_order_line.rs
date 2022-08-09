use crate::*;
use cpu_models::{BasicEmbeddingModel, GraphEmbedder, MatrixShape};
use graph::{EdgeT, Graph, NodeT};
use indicatif::ProgressIterator;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct FirstOrderLINE {
    model: BasicEmbeddingModel,
}

impl From<BasicEmbeddingModel> for FirstOrderLINE {
    fn from(model: BasicEmbeddingModel) -> Self {
        Self { model }
    }
}

impl GraphEmbedder for FirstOrderLINE {
    fn get_model_name(&self) -> String {
        "First-order LINE".to_string()
    }

    fn get_number_of_epochs(&self) -> usize {
        self.model.get_number_of_epochs()
    }

    fn is_verbose(&self) -> bool {
        self.model.is_verbose()
    }

    fn get_random_state(&self) -> u64 {
        self.model.get_random_state()
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String> {
        Ok(vec![(
            graph.get_number_of_nodes() as usize,
            self.model.get_embedding_size(),
        )
            .into()])
    }

    fn _fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String> {
        let mut random_state = splitmix64(self.get_random_state());

        // get all the devices in the system
        let devices = Device::get_devices()?;
        // we use the first device
        let device = devices[0];
        // setup this device for computation
        let mut gpu = GPU::new(device)?;
        // load our compiled code
        let mut ptx = gpu.load_ptx(PTX_SOURCE)?;
        // get a function from the compiled code
        let compute_first_order_line = ptx.get_kernel("compute_first_order_line")?;

        // set the parallelizzation specs
        let grid = Grid::default().set_block_x(1024)?;

        // allocate a gpu buffer and copy data from the host
        let embedding_on_gpu = gpu.buffer_from_slice::<f32>(embedding[0])?;
        let comulative_node_degrees = graph.get_cumulative_node_degrees();
        let destinations = graph.get_destination_node_ids(false);
        let gpu_comulative_node_degrees =
            gpu.buffer_from_slice::<EdgeT>(comulative_node_degrees.as_ref())?;
        let gpu_destinations = gpu.buffer_from_slice::<NodeT>(destinations.as_ref())?;

        let progress_bar = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in (0..self.model.get_number_of_epochs()).progress_with(progress_bar) {
            random_state = splitmix64(random_state);

            gpu.launch_kernel(
                &compute_first_order_line,
                &grid,
                args![
                    embedding_on_gpu.as_device_ptr(),
                    gpu_comulative_node_degrees.as_device_ptr(),
                    gpu_destinations.as_device_ptr(),
                    self.model.get_learning_rate(),
                    self.model.get_embedding_size(),
                    comulative_node_degrees.len(),
                    destinations.len(),
                ],
            )?;

            // wait for the gpu to finish
            gpu.synchronize()?;
        }

        embedding_on_gpu.copy_gpu2host(embedding[0])?;
        Ok(())
    }
}
