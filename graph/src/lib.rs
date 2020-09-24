#![warn(unused_macros)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![feature(map_first_last)]
//! # Ensmallen Graph
//! TODO! TEST DOC

const SEED_XOR: usize = 0xbad5eedbad5eed11;

mod vocabulary;
pub(crate) use self::vocabulary::Vocabulary;
mod vocabulary_vec;
pub(crate) use self::vocabulary_vec::VocabularyVec;

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
mod from_csv;

mod constructors;
pub(crate) use self::constructors::build_graph;

mod drop;
mod getters;
pub mod graph;
mod holdouts;
mod metrics;
mod operators;
mod preprocessing;
mod setters;
mod tarjan;
mod trees;
pub mod types;
mod walks;
mod walks_parameters;

pub mod test_utilities;

pub use self::graph::Graph;
pub use self::holdouts::*;
pub use self::operators::*;
pub use self::tarjan::*;
pub use self::trees::*;
pub use self::types::*;
pub use self::walks::*;
pub use self::walks_parameters::*;
pub use preprocessing::*;
pub use setters::set_num_threads;