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
//! let g = Graph::from_sorted_csv(edges_reader, None, false, false, 6108, 242, "Graph").unwrap();
//!
//!
//! ```
#![warn(unused_macros)]
#![feature(map_first_last)]
#![type_length_limit = "3764086"]

const SEED_XOR: usize = 0xbad5eedbad5eed11;

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
mod compression;
mod from_csv;
pub(crate) use self::compression::*;


mod constructors;

pub mod utils;
pub(crate) use self::utils::*;

mod bitmaps;
mod edge_lists;
mod filters;
mod getters;
mod graph;
mod hash;
mod holdouts;
mod iters;
mod metrics;
mod modifiers;
mod operators;
mod preprocessing;
mod remap;
mod remove;
mod setters;
mod tarjan;
mod trees;
mod types;
mod walks;
pub mod walks_parameters;

mod queries;
mod queries_boolean;
mod queries_unchecked;
mod queries_walk;
pub use self::queries::*;
pub use self::queries_boolean::*;

pub mod test_utilities;

pub use self::getters::*;
pub use self::graph::Graph;
pub use self::holdouts::*;
pub use self::metrics::*;
pub use self::operators::*;
pub use self::setters::*;
pub use self::tarjan::*;
pub use self::trees::*;
pub use self::types::*;
pub use self::walks::*;
pub use self::walks_parameters::*;
pub use preprocessing::*;
