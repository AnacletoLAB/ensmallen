#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod cbow;
mod edge_prediction_perceptron;
mod first_order_line;
mod glove;
mod graph_embedder;
mod node2vec;
mod optimizers;
mod second_order_line;
mod skipgram;
mod spine;
mod transe;
mod transh;
mod unstructured;
mod utils;
mod walk_transformer;
mod walklets;
mod basic_embedding_model;
mod weighted_spine;
mod basic_siamese_model;

pub(crate) use basic_embedding_model::*;
pub(crate) use basic_siamese_model::*;
pub(crate) use utils::*;

pub use cbow::*;
pub use edge_prediction_perceptron::*;
pub use first_order_line::*;
pub use glove::*;
pub use graph_embedder::*;
pub use node2vec::*;
pub use optimizers::*;
pub use second_order_line::*;
pub use skipgram::*;
pub use spine::*;
pub use transe::*;
pub use transh::*;
pub use unstructured::*;
pub use walk_transformer::*;
pub use walklets::*;
pub use weighted_spine::*;
