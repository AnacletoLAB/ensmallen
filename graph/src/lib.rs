//! Ensmallen its an efficient graph manipulation library.
//!
//! # Example:
//!
//! ```rust
//! use graph::{EdgeFileReader, Graph};
//! let edges_reader = EdgeFileReader::new("tests/data/test_components.csv",).unwrap()
//!     .set_separator(Some(",")).unwrap()
//!     .set_verbose(Some(false))
//!     .set_numeric_node_ids(Some(true))
//!     .set_header(Some(false));
//!  
//! let g = Graph::from_sorted_csv(edges_reader, None, false, false, 6, 6, "Graph").unwrap();
//!
//!
//! ```
//! # Definitions
//! * `edge`: an arch between to nodes
//! * `selfloop`: An edge which source and destination are equal
//! * `singleton`: A node with in degree and out degree 0
//! * `singleton_with_selfloop`: A node which has only selfloops
//! * `connected_node`: a node which is nor a `singleton` nor a `singleton_with_selfloops`.

#![warn(unused_macros)]
#![feature(is_sorted)]
#![feature(map_first_last)]
#![type_length_limit = "3764086"]
#![feature(option_result_unwrap_unchecked)]

mod method_caller;
pub(crate) use method_caller::*;

mod vocabulary;
pub use self::vocabulary::Vocabulary;
mod node_type_vocabulary;
pub use self::node_type_vocabulary::NodeTypeVocabulary;
mod edge_type_vocabulary;
pub use self::edge_type_vocabulary::EdgeTypeVocabulary;

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
mod compression;
mod from_csv;
pub(crate) use self::compression::*;

mod validators;
pub use self::validators::*;
mod getters_boolean;
pub use self::getters_boolean::*;
mod replace;
pub use self::replace::*;

mod dijkstra;
pub use self::dijkstra::*;

mod constructors;

pub mod utils;
pub(crate) use self::utils::*;

mod bitmaps;
mod centrality;
mod dense;
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
mod remap;
mod remove;
mod selfloops;
mod setters;
mod tarjan;
mod url_utilities;
mod tfidf;
mod thickeners;
mod to_conversions;
mod transitivity;
mod trees;
mod types;
mod sort;
mod vertex_cover;
mod walks;
pub mod walks_parameters;

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
