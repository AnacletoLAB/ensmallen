#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod cbow;
mod edge_prediction_perceptron;
mod glove;
mod graph_embedder;
mod node2vec;
mod skipgram;
mod spine;
mod transe;
mod utils;
mod walk_transformer;
mod walklets;
mod weighted_spine;
mod optimizers;

pub(crate) use utils::*;

pub use cbow::*;
pub use edge_prediction_perceptron::*;
pub use glove::*;
pub use graph_embedder::*;
pub use node2vec::*;
pub use skipgram::*;
pub use spine::*;
pub use transe::*;
pub use walk_transformer::*;
pub use walklets::*;
pub use weighted_spine::*;
pub use optimizers::*;