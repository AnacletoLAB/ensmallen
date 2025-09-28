//! Ensmallen its an efficient graph manipulation library.
//!
//! # Definitions
//! * `edge`: an arch between to nodes
//! * `selfloop`: An edge which source and destination are equal
//! * `singleton`: A node with in degree and out degree 0
//! * `singleton_with_selfloop`: A node which has only selfloops
//! * `connected_node`: a node which is nor a `singleton` nor a `singleton_with_selfloops`.

#![warn(unused_macros)]
#![feature(iter_advance_by)]
#![feature(impl_trait_in_assoc_type)]
#![feature(string_remove_matches)]
#![feature(exit_status_error)]
#![feature(sync_unsafe_cell)]
#![feature(pattern)]
#![deny(unconditional_recursion)]
#![type_length_limit = "3764086"]

use std::sync::Arc;

mod types;
pub use types::*;

pub mod utils;
use tags::*;
pub use utils::*;

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
mod from_csv;

mod parameters_validators;
mod getters_boolean;

mod dijkstra;
pub use self::dijkstra::*;

mod coo;

mod edge_prediction_analysis;

mod heterogeneous_graphlets;

mod constructors;
pub use constructors::*;

mod bitmaps;
mod centrality;
mod dense;
mod distributions;
mod edge_isomorphism;
mod edge_list_utils;
mod edge_lists;
mod edge_metrics;
mod filters;
mod getters;
mod graph;
mod hash;
mod hashes;
mod holdouts;
mod hyperball;
mod isomorphism;
pub mod isomorphism_iter;
mod iter_queries;
mod iters;
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
mod triad_census;
mod url_utilities;
mod vertex_cover;
mod walks;
pub mod walks_parameters;
pub use edge_list_utils::*;

mod report;

mod queries;
mod queries_boolean;
mod queries_walk;
pub use self::url_utilities::*;

pub mod test_utilities;

pub use self::graph::Graph;
pub use self::walks_parameters::*;
pub use edge_isomorphism::*;
pub use preprocessing::*;
pub use tfidf::*;

mod dijkstra_queue;
pub use dijkstra_queue::*;

use vec_rand::splitmix64;

mod cache;
use cache::*;

mod getters_cached;
mod visualizations;

mod memory;
pub use memory::*;

mod louvain;
mod nodes_sampling;

mod subgraphs;

mod chains;
pub use chains::*;

mod tendrils;
pub use tendrils::*;

mod circles;
pub use circles::*;

mod stars;
pub use stars::*;

mod node_tuples;
pub use node_tuples::*;

mod dendritic_tree;
pub use dendritic_tree::*;

mod cliques;
pub use cliques::*;

mod graphs_from_edge_lists;

mod builder;
pub use builder::*;
