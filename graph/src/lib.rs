//! Ensmallen its an efficient graph manipulation library.
//!
//! # Definitions
//! * `edge`: an arch between to nodes
//! * `selfloop`: An edge which source and destination are equal
//! * `singleton`: A node with in degree and out degree 0
//! * `singleton_with_selfloop`: A node which has only selfloops
//! * `connected_node`: a node which is nor a `singleton` nor a `singleton_with_selfloops`.

#![warn(unused_macros)]
#![feature(is_sorted)]
#![feature(map_first_last)]
#![feature(core_intrinsics)]
#![type_length_limit = "3764086"]
#![feature(exclusive_range_pattern)]
#![feature(option_result_unwrap_unchecked)]
#![feature(macro_attributes_in_derive_output)]

mod vocabulary;
pub use self::vocabulary::*;
mod node_type_vocabulary;
pub use self::node_type_vocabulary::*;
mod edge_type_vocabulary;
pub use self::edge_type_vocabulary::*;

mod csv_file_writer;
pub(crate) use self::csv_file_writer::compose_lines;
pub use self::csv_file_writer::CSVFileWriter;
mod csv_file_reader;
pub use self::csv_file_reader::CSVFileReader;
mod node_file_reader;
pub use self::node_file_reader::NodeFileReader;
mod node_file_writer;
pub use self::node_file_writer::NodeFileWriter;
mod edge_file_reader;
pub use self::edge_file_reader::EdgeFileReader;
mod edge_file_writer;
pub use self::edge_file_writer::EdgeFileWriter;
mod type_file_reader;
pub use self::type_file_reader::TypeFileReader;
mod type_file_writer;
pub use self::type_file_writer::TypeFileWriter;
mod compression;
mod from_csv;
pub(crate) use self::compression::*;

mod parameters_validators;
pub use self::parameters_validators::*;
mod getters_boolean;
pub use self::getters_boolean::*;
mod replace;
pub use self::replace::*;

mod dijkstra;
pub use self::dijkstra::*;

mod constructors;
pub use constructors::*;

pub mod utils;
pub use self::utils::*;

mod bitmaps;
mod centrality;
mod dense;
mod edge_list_utils;
mod edge_lists;
mod edge_metrics;
mod filters;
mod getters;
mod graph;
mod hash;
mod holdouts;
mod iter_queries;
mod iters;
mod laplacian;
mod modifiers;
mod operators;
mod polygons;
mod preprocessing;
mod random_graphs;
mod remap;
mod remove;
mod selfloops;
mod setters;
mod sort;
mod tarjan;
mod tfidf;
mod thickeners;
mod to_conversions;
mod transitivity;
mod trees;
mod types;
mod url_utilities;
mod vertex_cover;
mod walks;
pub mod walks_parameters;
pub use edge_list_utils::*;

mod report;
pub use self::report::*;

mod queries;
mod queries_boolean;
mod queries_walk;
pub use self::queries::*;
pub use self::queries_boolean::*;
pub use self::url_utilities::*;

pub mod test_utilities;

pub use self::edge_metrics::*;
pub use self::getters::*;
pub use self::graph::Graph;
pub use self::holdouts::*;
pub use self::operators::*;
pub use self::setters::*;
pub use self::tarjan::*;
pub use self::trees::*;
pub use self::types::*;
pub use self::walks::*;
pub use self::walks_parameters::*;
pub use preprocessing::*;
pub use tfidf::*;

mod dijkstra_queue;
pub use dijkstra_queue::*;

use vec_rand::splitmix64;

use tags::*;

mod cache;
use cache::*;

mod getters_cached;
pub use getters_cached::*;

mod visualizations;
pub use visualizations::*;

mod memory;
pub use memory::*;

mod louvain;
pub use louvain::*;
mod nodes_sampling;
pub use nodes_sampling::*;

mod subgraphs;
pub use subgraphs::*;

mod chains;
pub use chains::*;

mod circles;
pub use circles::*;