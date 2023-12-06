use graph::{Graph, NodeT};
use num_traits::{AsPrimitive, Float, One};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    any::TypeId,
    cell::SyncUnsafeCell,
    ops::{AddAssign, DivAssign},
};

#[derive(Clone, Deserialize, Serialize, Debug)]
/// Struct implementing Graph Convolution.
pub struct GraphConvolution {
    /// Number of hops.
    number_of_convolutions: usize,
    /// Concatenate the normalized and non-normalized features
    concatenate_features: bool,
    /// Whether to normalize the rows between convolutions.
    normalize_rows: bool,
    /// The embedding data type.
    dtype: String,
}

impl GraphConvolution {
    /// Creates a new GraphConvolution instance.
    ///
    /// # Arguments
    /// * `number_of_convolutions`: Option<usize> - The number of hops to use. Default is 2.
    /// * `concatenate_features`: Option<bool> - Whether to concatenate the normalized and non-normalized features.
    /// * `normalize_rows`: Option<bool> - Whether to normalize the rows between convolutions.
    /// * `dtype`: Option<String> - The embedding data type.
    ///
    /// # Raises
    /// * If the provided data type is not supported.
    pub fn new(
        number_of_convolutions: Option<usize>,
        concatenate_features: Option<bool>,
        normalize_rows: Option<bool>,
        dtype: Option<String>,
    ) -> Result<Self, String> {
        if !["f16", "f32", "f64"].contains(&dtype.as_ref().unwrap_or(&"f32".to_string()).as_str()) {
            return Err(format!(
                concat!(
                    "The data type `{}` is not supported. ",
                    "Supported data types are f16, f32 and f64."
                ),
                dtype.as_ref().unwrap_or(&"f32".to_string())
            ));
        }

        Ok(Self {
            number_of_convolutions: number_of_convolutions.unwrap_or(2),
            concatenate_features: concatenate_features.unwrap_or(true),
            normalize_rows: normalize_rows.unwrap_or(true),
            dtype: dtype.unwrap_or("f32".to_string()),
        })
    }

    /// Returns whether the features will be concatenated.
    pub fn get_concatenate_features(&self) -> bool {
        self.concatenate_features
    }

    /// Return the number of hops.
    pub fn get_number_of_convolutions(&self) -> usize {
        self.number_of_convolutions
    }

    /// Returns the dtype.
    pub fn get_dtype(&self) -> &str {
        &self.dtype
    }

    /// Returns the convolution over the provided support.
    ///
    /// # Arguments
    /// * `support`: &Graph - The graph to convolve with.
    /// * `node_features`: &[F1] - The node features to convolve.
    /// * `dimensionality`: usize - The dimensionality of the node features.
    /// * `convolved_node_features`: &mut [F2] - The memory area where to store the convolved node features.
    ///
    /// # Raises
    /// * If the provided node features slice has a length different than the number of nodes in the support.
    /// * If the provided convolved node features slice has a length different than the number of nodes in the graph multiplied by the dimensionality.
    ///
    pub fn transform<
        F1: Send + Sync + AsPrimitive<F2>,
        F2: Float + Send + Sync + Copy + One + AddAssign + DivAssign + 'static,
    >(
        &self,
        support: &Graph,
        node_features: &[F1],
        dimensionality: usize,
        convolved_node_features: &mut [F2],
    ) -> Result<(), String> {
        // Check whether the provided node features is divisible exactly by the provided dimensionality.
        if node_features.len() % dimensionality != 0 {
            return Err(format!(
                concat!(
                    "The provided node features slice has a length of `{}` ",
                    "but it should be divisible exactly by the provided dimensionality `{}`."
                ),
                node_features.len(),
                dimensionality
            ));
        }

        // Check whether the provided node features has exactly number of nodes * dimensionality elements.
        // We also need to provide a meaningful error, checking how many nodes are missing in the node features
        // or, conversely, how many nodes seem to be in excess.
        if node_features.len() / dimensionality != support.get_number_of_nodes() as usize {
            return Err(format!(
                concat!(
                    "The provided node features have `{}` rows, but the provided graph has `{}` nodes. ",
                    "The number of rows in the node features should be equal to the number of nodes."
                ),
                node_features.len() / dimensionality,
                support.get_number_of_nodes()
            ));
        }

        // The user may choose to concatenate the features obtained at all the different
        // convolution steps. We need to check that the provided convolved node features
        // slice has the expected length, also taking into account the concatenations.
        let factor = if self.concatenate_features {
            self.number_of_convolutions + 1
        } else {
            1
        };

        // We compute the number of elements per row in the convolved node features.
        let convolved_node_features_row_size = dimensionality * factor;

        // Check whether the provided convolved node features is divisible exactly by the provided dimensionality,
        // multiplied by the number of convolutions.
        if convolved_node_features.len() % (convolved_node_features_row_size) != 0 {
            return Err(format!(
                concat!(
                    "The provided convolved node features slice has a length of `{}` ",
                    "but it should be divisible exactly by the provided dimensionality `{}` ",
                    "multiplied by the number of convolutions `{}`."
                ),
                convolved_node_features.len(),
                dimensionality,
                factor
            ));
        }

        // Check whether the provided convolved node features has exactly number of nodes * dimensionality elements,
        // multiplied by the number of convolutions.

        if convolved_node_features.len() / (convolved_node_features_row_size)
            != support.get_number_of_nodes() as usize
        {
            return Err(format!(
                concat!(
                    "The provided convolved node features have `{}` rows, but the provided graph has `{}` nodes. ",
                    "The number of rows in the convolved node features should be equal to the number of nodes."
                ),
                convolved_node_features.len() / (convolved_node_features_row_size),
                support.get_number_of_nodes()
            ));
        }

        // First, we need to copy the node features into the convolved node features.
        convolved_node_features
            .par_chunks_exact_mut(convolved_node_features_row_size)
            .zip(node_features.par_chunks_exact(dimensionality))
            .for_each(
                |(convoluted_row, original_row): (&mut [F2], &[F1])| unsafe {
                    // If the source and target features have the same type, we can use a copy
                    // non-overlapping avoiding any iterative operation.
                    if TypeId::of::<F1>() == TypeId::of::<F2>() {
                        // Copy the estimated overlaps
                        std::ptr::copy_nonoverlapping(
                            original_row.as_ptr() as *const F2,
                            convoluted_row.as_mut_ptr(),
                            dimensionality,
                        );
                    }
                    // Otherwise, we need to iterate over the elements.
                    else {
                        for (source, target) in original_row.iter().zip(convoluted_row.iter_mut()) {
                            *target = source.as_();
                        }
                    }
                },
            );

        // If requested, we normalize the features associated to the 0-th iteration.
        if self.normalize_rows {
            convolved_node_features
                .par_chunks_exact_mut(convolved_node_features_row_size)
                .for_each(|convoluted_row| {
                    // We compute the norm of the current row.
                    let norm = convoluted_row
                        .iter()
                        .take(dimensionality)
                        .fold(F2::zero(), |acc, x| acc + x.powi(2))
                        .sqrt()
                        .max(F2::epsilon());

                    // We normalize the convolved node features by the degree.
                    convoluted_row
                        .iter_mut()
                        .take(dimensionality)
                        .for_each(|node_feature| {
                            *node_feature /= norm;
                        });
                });
        }

        // We start the convolutions process.
        if self.concatenate_features {
            // If we are concatenating the features, we do not need to allocate any temporary memory area.
            // We can simply use the previous convolved node features as the source of the next convolution.
            // While we are sure there cannot be thread races, we will have to access non-mutable an area of
            // memory which is also accessed in the meantime mutably, i.e. the area of memory where the
            // last convolution is stores, and the area of memory where the current convolution will be stored.
            // For this reason, we will have to use an unsafe cell, and specifically since we have to access
            // the memory area within threads we will have to use a sync unsafe cell.
            let convolved_node_features = SyncUnsafeCell::new(convolved_node_features);

            (0..self.number_of_convolutions).for_each(|convolution_number| {
                unsafe {
                    (*convolved_node_features.get())
                        .par_chunks_exact_mut(convolved_node_features_row_size)
                }
                .enumerate()
                .for_each(|(node_id, convoluted_row)| {
                    // First of all, we copy the previously computed convolved node features into the
                    // next convolution memory area.
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            convoluted_row
                                .as_ptr()
                                .add(dimensionality * convolution_number),
                            convoluted_row
                                .as_mut_ptr()
                                .add(dimensionality * (convolution_number + 1)),
                            dimensionality,
                        );
                    }
                    // We get the area of the convolved node features where we will store the
                    // current convolution.
                    let current_node_convolution: &mut [F2] = &mut convoluted_row[dimensionality
                        * (convolution_number + 1)
                        ..dimensionality * (convolution_number + 2)];

                    // We compute the degree of the current node.
                    let mut degree = F2::one();

                    // Next, we sum to this memory area the features of the neighbours.
                    unsafe {
                        support
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .filter(|&dst| dst != node_id as NodeT)
                    .for_each(|dst| {
                        let dst_feature_row: &[F2] = unsafe {
                            &(*convolved_node_features.get())[(dst as usize)
                                * convolved_node_features_row_size
                                ..(dst as usize + 1) * convolved_node_features_row_size]
                        };

                        let previous_dst_convolution: &[F2] = &dst_feature_row[dimensionality
                            * (convolution_number)
                            ..dimensionality * (convolution_number + 1)];

                        // We sum the feature associated to the current neighbour to the
                        // convolved node features.
                        for (node_feature, neighbour_feature) in current_node_convolution
                            .iter_mut()
                            .zip(previous_dst_convolution.iter())
                        {
                            degree += F2::one();
                            *node_feature += *neighbour_feature;
                        }
                    });

                    // We normalize the convolved node features by the degree.
                    current_node_convolution
                        .iter_mut()
                        .for_each(|node_feature| {
                            *node_feature /= degree;
                        });
                });

                // If requested, we normalize the features associated to the i-th iteration.
                if self.normalize_rows {
                    unsafe {
                        (*convolved_node_features.get())
                            .par_chunks_exact_mut(convolved_node_features_row_size)
                    }
                    .for_each(|convoluted_row| {
                        // We compute the norm of the current row.
                        let norm = convoluted_row
                            .iter()
                            .skip(dimensionality * (convolution_number + 1))
                            .fold(F2::zero(), |acc, x| acc + x.powi(2))
                            .sqrt()
                            .max(F2::epsilon());

                        // We normalize the convolved node features by the degree.
                        convoluted_row
                            .iter_mut()
                            .skip(dimensionality * (convolution_number + 1))
                            .for_each(|node_feature| {
                                *node_feature /= norm;
                            });
                    });
                }
            });
        } else {
            // Now, if we are not concatenating the features, in order to execute the convolutions without
            // having data races, we are forced to allocate a temporary memory area with the same size of the
            // convolved node features. We cannot simply swap the two pointers, but we will need to adjust
            // the pointers to the memory area where the convolved node features are stored.

            // We allocate the temporary memory area.
            let mut temporary_convolved_node_features: Vec<F2> = convolved_node_features.to_vec();
            let mut convolved_node_features_ref: &mut [F2] = convolved_node_features;
            let mut temporary_convolved_node_features_ref: &mut [F2] =
                temporary_convolved_node_features.as_mut();

            (0..self.number_of_convolutions).for_each(|_| {
                convolved_node_features_ref
                    .par_chunks_exact_mut(dimensionality)
                    .zip(temporary_convolved_node_features_ref.par_chunks_exact(dimensionality))
                    .enumerate()
                    .for_each(|(node_id, (convoluted_row, temporary_convoluted_row))| {
                        // First of all, we copy the previously computed convolved node features into the
                        // next convolution memory area.
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                temporary_convoluted_row.as_ptr(),
                                convoluted_row.as_mut_ptr(),
                                dimensionality,
                            );
                        }

                        // We compute the degree of the current node.
                        let mut degree = F2::one();

                        // Next, we sum to this memory area the features of the neighbours.
                        unsafe {
                            support.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                node_id as NodeT,
                            )
                        }
                        .filter(|&dst| dst != node_id as NodeT)
                        .for_each(|dst| {
                            let dst_feature_row: &[F2] = &temporary_convolved_node_features_ref[(dst
                                as usize)
                                * convolved_node_features_row_size
                                ..(dst as usize + 1) * convolved_node_features_row_size];

                            // We sum the feature associated to the current neighbour to the
                            // convolved node features.
                            for (node_feature, neighbour_feature) in
                                convoluted_row.iter_mut().zip(dst_feature_row.iter())
                            {
                                degree += F2::one();
                                *node_feature += *neighbour_feature;
                            }
                        });

                        // We normalize the convolved node features by the degree.
                        convoluted_row.iter_mut().for_each(|node_feature| {
                            *node_feature /= degree;
                        });
                    });

                // If requested, we normalize the features associated to the i-th iteration.
                if self.normalize_rows {
                    convolved_node_features_ref
                        .par_chunks_exact_mut(dimensionality)
                        .for_each(|convoluted_row| {
                            // We compute the norm of the current row.
                            let norm = convoluted_row
                                .iter()
                                .fold(F2::zero(), |acc, x| acc + x.powi(2))
                                .sqrt()
                                .max(F2::epsilon());

                            // We normalize the convolved node features by the degree.
                            convoluted_row.iter_mut().for_each(|node_feature| {
                                *node_feature /= norm;
                            });
                        });
                }

                // We swap the two memory areas.
                std::mem::swap(
                    &mut convolved_node_features_ref,
                    &mut temporary_convolved_node_features_ref,
                );
            });

            // Depending on the number of convluions, we need to copy the temporary convolved node features
            // into the convolved node features.
            if self.number_of_convolutions % 2 == 1 {
                convolved_node_features.copy_from_slice(&temporary_convolved_node_features);
            }
        }

        Ok(())
    }

    pub fn dump(&self, path: &str) -> Result<(), String> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        serde_json::from_reader(std::fs::File::open(path).map_err(move |e| e.to_string())?)
            .map_err(move |e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
