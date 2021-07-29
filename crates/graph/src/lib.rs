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
#![feature(option_result_unwrap_unchecked)]

mod vocabularies;
pub use vocabularies::*;
mod io;
pub use self::io::*;

mod memory;
mod cache;
mod compression;
mod from_csv;
mod graph_impl;

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
mod random_graphs;
mod remap;
mod remove;
mod selfloops;
mod setters;
mod sort;
mod tarjan;
mod thickeners;
mod to_conversions;
mod transitivity;
mod trees;
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

pub use self::edge_metrics::*;
pub use self::getters::*;
pub use self::graph::Graph;
pub use self::holdouts::*;
pub use self::operators::*;
pub use self::setters::*;
pub use self::tarjan::*;
pub use self::trees::*;
pub use self::walks::*;
pub use self::walks_parameters::*;
pub use self::preprocessing::*;

mod dijkstra_queue;

mod getters_cached;
pub use getters_cached::*;

mod visualizations;
pub use visualizations::*;
