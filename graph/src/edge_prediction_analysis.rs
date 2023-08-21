use std::sync::atomic::AtomicUsize;

use super::*;
use core::sync::atomic::Ordering;
use rayon::prelude::*;
use std::mem::transmute;

struct PredictionsReader {
    reader: CSVFileReader,
}

impl PredictionsReader {
    pub fn new(reader: CSVFileReader) -> Self {
        Self { reader }
    }

    fn par_iter_predictions(
        &self,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
    ) -> Result<impl ParallelIterator<Item = Result<(usize, (String, String, f32))>> + '_> {
        // If the user provided a source column name, we check that it exists
        // and the retrieve the column number associated to it. If the user did
        // not provide a source column name, we check whether a column with a plausible
        // name is available so as to automatically infer it.
        let source_column_number = if let Some(source_column_name) = source_column_name {
            self.reader.get_column_number(source_column_name)?
        } else {
            let mut source_column_number = usize::MAX;
            for candidate_source_column_name in &[
                "source",
                "src",
                "subject",
                "subjects",
                "sources",
                "source_name",
            ] {
                if let Ok(column_number) = self
                    .reader
                    .get_column_number(candidate_source_column_name.to_string())
                {
                    source_column_number = column_number;
                    break;
                }
            }
            if source_column_number == usize::MAX {
                return Err(format!(concat!(
                    "No source column found in the provided file. ",
                    "Please provide a source column name using source_column_name parameter."
                )));
            }
            source_column_number
        };

        // Same thing for destination column name.
        let destination_column_number = if let Some(destination_column_name) =
            destination_column_name
        {
            self.reader.get_column_number(destination_column_name)?
        } else {
            let mut destination_column_number = usize::MAX;
            for candidate_destination_column_name in &[
                "destination",
                "dst",
                "object",
                "objects",
                "destinations",
                "destination_name",
            ] {
                if let Ok(column_number) = self
                    .reader
                    .get_column_number(candidate_destination_column_name.to_string())
                {
                    destination_column_number = column_number;
                    break;
                }
            }
            if destination_column_number == usize::MAX {
                return Err(format!(
                    concat!(
                        "No destination column found in the provided file. ",
                        "Please provide a destination column name using destination_column_name parameter."
                    )
                ));
            }
            destination_column_number
        };

        // Same thing for prediction column name.
        let prediction_column_number = if let Some(prediction_column_name) = prediction_column_name
        {
            self.reader.get_column_number(prediction_column_name)?
        } else {
            let mut prediction_column_number = usize::MAX;
            for candidate_prediction_column_name in &[
                "prediction",
                "pred",
                "score",
                "scores",
                "predictions",
                "prediction_score",
            ] {
                if let Ok(column_number) = self
                    .reader
                    .get_column_number(candidate_prediction_column_name.to_string())
                {
                    prediction_column_number = column_number;
                    break;
                }
            }
            if prediction_column_number == usize::MAX {
                return Err(format!(
                    concat!(
                        "No prediction column found in the provided file. ",
                        "Please provide a prediction column name using prediction_column_name parameter."
                    )
                ));
            }
            prediction_column_number
        };

        Ok(self
            .reader
            .read_lines(Some(vec![
                source_column_number,
                destination_column_number,
                prediction_column_number,
            ]))?
            .map(|line| {
                match line {
                    Ok((line_number, line)) => {
                        let [source, destination, prediction] = line.as_slice() else {
                            return Err(format!(
                                concat!(
                                    "Invalid number of columns at line {} in the provided file. ",
                                    "Expected 3 columns, found {}."
                                ),
                                line_number,
                                line.len()
                            ));
                        };
                        if source.is_none() {
                            return Err(format!(
                                "Missing source at line {} in the provided file.",
                                line_number
                            ));
                        }
                        if destination.is_none() {
                            return Err(format!(
                                "Missing destination at line {} in the provided file.",
                                line_number
                            ));
                        }
                        if prediction.is_none() {
                            return Err(format!(
                                "Missing prediction at line {} in the provided file.",
                                line_number
                            ));
                        }

                        let source = source.clone().unwrap();
                        let destination = destination.clone().unwrap();
                        // We convert the prediction to a float.
                        let prediction: f32 =
                            prediction.clone().unwrap().parse::<f32>().map_err(|_| {
                                format!(
                                    "Invalid prediction at line {} in the provided file.",
                                    line_number
                                )
                            })?;

                        // We check that the prediction is within 0.0 and 1.0
                        if prediction < 0.0 || prediction > 1.0 {
                            return Err(format!(
                                concat!(
                                    "Invalid prediction at line {} in the provided file. ",
                                    "Predictions must be between 0.0 and 1.0, found {}."
                                ),
                                line_number, prediction
                            ));
                        }

                        Ok((line_number, (source, destination, prediction)))
                    }
                    Err(err) => Err(err),
                }
            })
            .unwrap_parallel())
    }

    /// Return the predictions within the provided minimum and maximum predictions.
    ///
    /// # Arguments
    /// * `source_column_name`: Option<String> - Name of the source column.
    /// * `destination_column_name`: Option<String> - Name of the destination column.
    /// * `prediction_column_name`: Option<String> - Name of the prediction column.
    /// * `allowed_source_nodes`: Option<Vec<String>> - List of allowed source nodes.
    /// * `allowed_destination_nodes`: Option<Vec<String>> - List of allowed destination nodes.
    /// * `allowed_source_node_prefixes`: Option<Vec<String>> - List of allowed source node prefixes.
    /// * `allowed_destination_node_prefixes`: Option<Vec<String>> - List of allowed destination node prefixes.
    /// * `min_prediction`: Option<f32> - Minimum prediction to consider.
    /// * `max_prediction`: Option<f32> - Maximum prediction to consider.
    ///
    fn par_iter_filtered_predictions(
        &self,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
        allowed_source_nodes: Option<Vec<String>>,
        allowed_destination_nodes: Option<Vec<String>>,
        allowed_source_node_prefixes: Option<Vec<String>>,
        allowed_destination_node_prefixes: Option<Vec<String>>,
        min_prediction: Option<f32>,
        max_prediction: Option<f32>,
    ) -> Result<impl ParallelIterator<Item = Result<(usize, (String, String, f32))>> + '_> {
        let min_prediction = min_prediction.unwrap_or(0.0);
        let max_prediction = max_prediction.unwrap_or(1.0);

        Ok(self
            .par_iter_predictions(
                source_column_name,
                destination_column_name,
                prediction_column_name,
            )?
            .filter(move |line| {
                if let Ok((_, (src, dst, prediction))) = line {
                    allowed_source_nodes
                        .as_ref()
                        .map_or(true, |allowed_source_nodes| {
                            allowed_source_nodes.contains(src)
                        })
                        && allowed_destination_nodes
                            .as_ref()
                            .map_or(true, |allowed_destination_nodes| {
                                allowed_destination_nodes.contains(dst)
                            })
                        && allowed_source_node_prefixes.as_ref().map_or(
                            true,
                            |allowed_source_node_prefixes| {
                                allowed_source_node_prefixes
                                    .iter()
                                    .any(|prefix| src.starts_with(prefix))
                            },
                        )
                        && allowed_destination_node_prefixes.as_ref().map_or(
                            true,
                            |allowed_destination_node_prefixes| {
                                allowed_destination_node_prefixes
                                    .iter()
                                    .any(|prefix| dst.starts_with(prefix))
                            },
                        )
                        && prediction >= &min_prediction
                        && prediction <= &max_prediction
                } else {
                    true
                }
            }))
    }
}

/// Tooling for analysis of edge prediction CSVs and TSVs.
impl Graph {
    /// Return parallel iterator over the provided file.
    ///
    /// # Arguments
    /// * `path`: String - Path to the file to read.
    /// * `separator`: Option<char> - Separator used in the file.
    /// * `remove_chevrons`: Option<bool> - Whether to remove chevrons from the file.
    /// * `remove_spaces`: Option<bool> - Whether to remove spaces from the file.
    /// * `support_balanced_quotes`: Option<bool> - Whether to support balanced quotes in the file.
    /// * `comment_symbol`: Option<String> - Symbol used to indicate that a line is a comment.
    /// * `max_rows_number`: Option<usize> - Maximum number of rows to read.
    ///
    /// # Raises
    /// * If the file does not exist.
    /// * If the file is not a CSV or TSV file.
    /// * If the file does not contain the provided source column name.
    /// * If the file does not contain the provided destination column name.
    /// * If the file does not contain the provided prediction column name.
    /// * If the file contains invalid predictions.
    /// * If the file contains invalid lines.
    /// * If the file contains invalid numbers of columns.
    /// * If the file contains missing sources.
    /// * If the file contains missing destinations.
    /// * If the file contains missing predictions.
    ///
    fn get_predictions_reader(
        &self,
        path: String,
        separator: Option<char>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        support_balanced_quotes: Option<bool>,
        comment_symbol: Option<String>,
        max_rows_number: Option<usize>,
    ) -> Result<PredictionsReader> {
        let reader: CSVFileReader = CSVFileReader::new(path, "predictions")?
            .set_parallel(Some(true))
            .set_remove_chevrons(remove_chevrons)
            .set_remove_spaces(remove_spaces)
            .set_support_balanced_quotes(support_balanced_quotes)
            .set_comment_symbol(comment_symbol)?
            .set_max_rows_number(max_rows_number)?
            .set_separator(separator)?;

        Ok(PredictionsReader::new(reader))
    }

    /// Return the number of existing and non-existing edges in the provided file.
    ///
    /// # Arguments
    /// * `path`: String - Path to the file to read.
    /// * `source_column_name`: Option<String> - Name of the source column.
    /// * `destination_column_name`: Option<String> - Name of the destination column.
    /// * `prediction_column_name`: Option<String> - Name of the prediction column.
    /// * `allowed_source_nodes`: Option<Vec<String>> - List of allowed source nodes.
    /// * `allowed_destination_nodes`: Option<Vec<String>> - List of allowed destination nodes.
    /// * `allowed_source_node_prefixes`: Option<Vec<String>> - List of allowed source node prefixes.
    /// * `allowed_destination_node_prefixes`: Option<Vec<String>> - List of allowed destination node prefixes.
    /// * `min_prediction`: Option<f32> - Minimum prediction to consider.
    /// * `max_prediction`: Option<f32> - Maximum prediction to consider.
    /// * `bins`: Option<usize> - Number of bins to use for the histogram.
    /// * `separator`: Option<char> - Separator used in the file.
    /// * `remove_chevrons`: Option<bool> - Whether to remove chevrons from the file.
    /// * `remove_spaces`: Option<bool> - Whether to remove spaces from the file.
    /// * `support_balanced_quotes`: Option<bool> - Whether to support balanced quotes in the file.
    /// * `comment_symbol`: Option<String> - Symbol used to indicate that a line is a comment.
    /// * `max_rows_number`: Option<usize> - Maximum number of rows to read.
    ///
    pub fn get_predictions_histograms(
        &self,
        path: String,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
        allowed_source_nodes: Option<Vec<String>>,
        allowed_destination_nodes: Option<Vec<String>>,
        allowed_source_node_prefixes: Option<Vec<String>>,
        allowed_destination_node_prefixes: Option<Vec<String>>,
        min_prediction: Option<f32>,
        max_prediction: Option<f32>,
        bins: Option<usize>,
        separator: Option<char>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        support_balanced_quotes: Option<bool>,
        comment_symbol: Option<String>,
        max_rows_number: Option<usize>,
    ) -> Result<(Vec<usize>, Vec<usize>)> {
        let bins = bins.unwrap_or(100);

        let existing_edges_histograms: Vec<AtomicUsize> = unsafe { transmute(vec![0_usize; bins]) };
        let non_existing_edges_histograms: Vec<AtomicUsize> = unsafe { transmute(vec![0_usize; bins]) };

        self.get_predictions_reader(
            path,
            separator,
            remove_chevrons,
            remove_spaces,
            support_balanced_quotes,
            comment_symbol,
            max_rows_number,
        )?
        .par_iter_filtered_predictions(
            source_column_name,
            destination_column_name,
            prediction_column_name,
            allowed_source_nodes,
            allowed_destination_nodes,
            allowed_source_node_prefixes,
            allowed_destination_node_prefixes,
            min_prediction,
            max_prediction,
        )?
        .map(|line| match line {
            Ok((line_number, (src_name, dst_name, prediction))) => {
                let prediction_bin = ((prediction * bins as f32) as usize).max(bins - 1);

                if self.has_edge_from_node_names(&src_name, &dst_name) {
                    existing_edges_histograms[prediction_bin].fetch_add(1, Ordering::Relaxed);
                } else {
                    non_existing_edges_histograms[prediction_bin].fetch_add(1, Ordering::Relaxed);
                }

                Ok(())
            }
            Err(err) => Err(err),
        })
        .collect::<Result<()>>()?;

        Ok((unsafe { transmute(existing_edges_histograms) }, unsafe {
            transmute(non_existing_edges_histograms)
        }))
    }

    /// Returns triple with source, destination and predictions according to provided filters.
    ///
    /// # Arguments
    /// * `path`: String - Path to the file to read.
    /// * `source_column_name`: Option<String> - Name of the source column.
    /// * `destination_column_name`: Option<String> - Name of the destination column.
    /// * `prediction_column_name`: Option<String> - Name of the prediction column.
    /// * `allowed_source_nodes`: Option<Vec<String>> - List of allowed source nodes.
    /// * `allowed_destination_nodes`: Option<Vec<String>> - List of allowed destination nodes.
    /// * `allowed_source_node_prefixes`: Option<Vec<String>> - List of allowed source node prefixes.
    /// * `allowed_destination_node_prefixes`: Option<Vec<String>> - List of allowed destination node prefixes.
    /// * `min_prediction`: Option<f32> - Minimum prediction to consider.
    /// * `max_prediction`: Option<f32> - Maximum prediction to consider.
    /// * `exclude_existing_edges`: Option<bool> - Whether to exclude existing edges.
    /// * `exclude_non_existing_edges`: Option<bool> - Whether to exclude non existing edges.
    /// * `separator`: Option<char> - Separator used in the file.
    /// * `remove_chevrons`: Option<bool> - Whether to remove chevrons from the file.
    /// * `remove_spaces`: Option<bool> - Whether to remove spaces from the file.
    /// * `support_balanced_quotes`: Option<bool> - Whether to support balanced quotes in the file.
    /// * `comment_symbol`: Option<String> - Symbol used to indicate that a line is a comment.
    /// * `max_rows_number`: Option<usize> - Maximum number of rows to read.
    ///
    pub fn get_filtered_predictions(
        &self,
        path: String,
        source_column_name: Option<String>,
        destination_column_name: Option<String>,
        prediction_column_name: Option<String>,
        allowed_source_nodes: Option<Vec<String>>,
        allowed_destination_nodes: Option<Vec<String>>,
        allowed_source_node_prefixes: Option<Vec<String>>,
        allowed_destination_node_prefixes: Option<Vec<String>>,
        min_prediction: Option<f32>,
        max_prediction: Option<f32>,
        exclude_existing_edges: Option<bool>,
        exclude_non_existing_edges: Option<bool>,
        separator: Option<char>,
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        support_balanced_quotes: Option<bool>,
        comment_symbol: Option<String>,
        max_rows_number: Option<usize>,
    ) -> Result<Vec<(String, String, f32)>> {
        let exclude_existing_edges = exclude_existing_edges.unwrap_or(false);
        let exclude_non_existing_edges = exclude_non_existing_edges.unwrap_or(false);

        if exclude_existing_edges && exclude_non_existing_edges {
            return Err(format!(concat!(
                "Invalid parameters: exclude_existing_edges and exclude_non_existing_edges ",
                "cannot be both set to true."
            )));
        }

        self.get_predictions_reader(
            path,
            separator,
            remove_chevrons,
            remove_spaces,
            support_balanced_quotes,
            comment_symbol,
            max_rows_number,
        )?
        .par_iter_filtered_predictions(
            source_column_name,
            destination_column_name,
            prediction_column_name,
            allowed_source_nodes,
            allowed_destination_nodes,
            allowed_source_node_prefixes,
            allowed_destination_node_prefixes,
            min_prediction,
            max_prediction,
        )?
        .filter_map(|line| match line {
            Ok((_, (src_name, dst_name, prediction))) => {
                let has_edge = self.has_edge_from_node_names(&src_name, &dst_name);
                if exclude_existing_edges && has_edge {
                    None
                } else if exclude_non_existing_edges && !has_edge {
                    None
                } else {
                    Some(Ok((src_name, dst_name, prediction)))
                }
            }
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<(String, String, f32)>>>()
    }
}
