#![feature(impl_trait_in_assoc_type)]
#![feature(adt_const_params)]
#![feature(atomic_from_mut)]
#![feature(sync_unsafe_cell)]
#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

mod alpine;
mod basic_embedding_model;
mod basic_siamese_model;
mod cbow;
mod dag_resnik;
mod degree_spine;
mod degree_wine;
mod distance_node_label_prediction_perceptron;
mod edge_prediction_perceptron;
mod first_order_line;
mod glove;
mod hyper_jaccard;
mod hyper_sketching;
mod graph_embedder;
mod node2vec;
mod node_label_prediction_perceptron;
mod node_type_spine;
mod node_type_wine;
mod optimizers;
mod rubicone;
mod ruine;
mod score_spine;
mod score_wine;
mod second_order_line;
mod skipgram;
mod spine;
mod structured_embedding;
mod transe;
mod unstructured;
mod utils;
mod walk_transformer;
mod walklets;
mod weighted_spine;
mod wine;
mod graph_convolution;

pub use alpine::*;
pub use basic_embedding_model::*;
pub use basic_siamese_model::*;
pub use utils::*;

pub use dag_resnik::*;
pub use degree_spine::*;
pub use degree_wine::*;
pub use distance_node_label_prediction_perceptron::*;
pub use edge_prediction_perceptron::*;
pub use first_order_line::*;
pub use graph_embedder::*;
pub use hyper_jaccard::*;
pub use hyper_sketching::*;
pub use node2vec::*;
pub use node_label_prediction_perceptron::*;
pub use node_type_spine::*;
pub use node_type_wine::*;
pub use optimizers::*;
pub use rubicone::*;
pub use ruine::*;
pub use score_spine::*;
pub use score_wine::*;
pub use second_order_line::*;
pub use spine::*;
pub use structured_embedding::*;
pub use transe::*;
pub use unstructured::*;
pub use walk_transformer::*;
pub use walklets::*;
pub use weighted_spine::*;
pub use wine::*;
pub use graph_convolution::*;
