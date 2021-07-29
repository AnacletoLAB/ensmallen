#![feature(core_intrinsics)]

mod get_minmax_node_from_numeric_edge_list;
pub use get_minmax_node_from_numeric_edge_list::*;
mod get_selfloops_number_from_edge_list;
pub use get_selfloops_number_from_edge_list::*;
mod sort_numeric_edge_list;
pub use sort_numeric_edge_list::*;
mod filter_duplicates_from_edge_list;
pub use filter_duplicates_from_edge_list::*;
mod convert_directed_edge_list_to_undirected;
pub use convert_directed_edge_list_to_undirected::*;
mod add_numeric_id_to_csv;
pub use add_numeric_id_to_csv::*;
mod are_there_selfloops_in_edge_list;
pub use are_there_selfloops_in_edge_list::*;
mod is_numeric_edge_list;
pub use is_numeric_edge_list::*;
mod convert_undirected_edge_list_to_directed;
pub use convert_undirected_edge_list_to_directed::*;