use super::*;
use arbitrary::Arbitrary;
use std::collections::HashSet;

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_weight_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_edge_degreee_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub walk_length : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub walk_length : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_iterations_Params {
	pub iterations : Option<NodeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_max_neighbours_Params {
	pub max_neighbours : Option<NodeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_random_state_Params {
	pub random_state : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_dense_node_mapping_Params {
	pub dense_node_mapping : Option<HashMap<NodeT, NodeT>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_return_weight_Params {
	pub return_weight : Option<WeightT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_explore_weight_Params {
	pub explore_weight : Option<WeightT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_change_node_type_weight_Params {
	pub change_node_type_weight : Option<WeightT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_change_edge_type_weight_Params {
	pub change_edge_type_weight : Option<WeightT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Compose_lines_Params {
	pub number_of_columns : usize,
	pub pairs : Vec<(String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub directed : bool,
	pub unique_self_loop_number : NodeT,
	pub self_loop_number : EdgeT,
	pub not_singleton_nodes_number : NodeT,
	pub singleton_nodes_with_self_loops_number : NodeT,
	pub unique_edges_number : EdgeT,
	pub edges : EliasFano,
	pub unique_sources : Option<EliasFano>,
	pub nodes : Vocabulary<NodeT>,
	pub node_bit_mask : EdgeT,
	pub node_bits : u8,
	pub edge_types : Option<EdgeTypeVocabulary>,
	pub name : S,
	pub weights : Option<Vec<WeightT>>,
	pub node_types : Option<NodeTypeVocabulary>,
	pub not_singleton_nodes : Option<BitVec<Lsb0, u8>>,
	pub singleton_nodes_with_self_loops : Option<RoaringBitmap>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Fast_u32_modulo_Params {
	pub val : u32,
	pub n : u32,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Not_one_Params {
	pub weight : WeightT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
	pub list_name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_column_number_Params {
	pub column_name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_sources_column_Params {
	pub sources_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_sources_column_number_Params {
	pub sources_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_destinations_column_Params {
	pub destinations_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_destinations_column_number_Params {
	pub destinations_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_edge_types_column_Params {
	pub edge_type_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_edge_types_column_number_Params {
	pub edge_types_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_weights_column_Params {
	pub weights_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_weights_column_number_Params {
	pub weights_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_skip_weights_if_unavailable_Params {
	pub skip_weights_if_unavailable : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_skip_edge_types_if_unavailable_Params {
	pub skip_edge_types_if_unavailable : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_default_weight_Params {
	pub default_weight : Option<WeightT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_graph_name_Params {
	pub graph_name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_default_edge_type_Params {
	pub default_edge_type : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_skip_self_loops_Params {
	pub skip_self_loops : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_csv_is_correct_Params {
	pub csv_is_correct : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_comment_symbol_Params {
	pub comment_symbol : Option<String>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_verbose_Params {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_might_have_singletons_with_selfloops_Params {
	pub might_have_singletons_with_selfloops : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_might_have_trap_nodes_Params {
	pub might_have_trap_nodes : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_edge_type_ids_Params {
	pub numeric_edge_type_ids : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_node_ids_Params {
	pub numeric_node_ids : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_ignore_duplicates_Params {
	pub ignore_duplicates : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_separator_Params {
	pub separator : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_header_Params {
	pub header : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_rows_to_skip_Params {
	pub rows_to_skip : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_max_rows_number_Params {
	pub max_rows_number : Option<u64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Parse_edge_line_Params {
	pub vals : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_with_self_loops_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Has_edge_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_node_trap_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Random_string_Params {
	pub len : usize,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Load_ppi_Params {
	pub load_nodes : bool,
	pub load_edge_types : bool,
	pub load_weights : bool,
	pub directed : bool,
	pub verbose : bool,
	pub skip_self_loops : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Load_empty_graph_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_nodes_column_Params {
	pub nodes_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_node_types_column_Params {
	pub nodes_type_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_nodes_column_number_Params {
	pub nodes_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_node_types_column_number_Params {
	pub node_types_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_verbose_Params {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_separator_Params {
	pub separator : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_header_Params {
	pub header : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_node_ids_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_ids_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_edge_id_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_id_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_unique_source_node_id_Params {
	pub source_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_nodes_ids_Params {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_node_degree_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_degree_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_node_names_Params {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_edge_type_id_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_weight_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_weight_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_node_name_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_name_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_destination_node_id_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destination_node_id_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_neighbours_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_minmax_edge_ids_from_source_node_id_Params {
	pub src : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_minmax_edge_ids_from_source_node_id_Params {
	pub src : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct From_usize_Params {
	pub v : usize,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub val : T,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Validate_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Validate_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Degrees_product_Params {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Jaccard_index_Params {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Adamic_adar_index_Params {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Resource_allocation_index_Params {
	pub one : NodeT,
	pub two : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_components_number_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Textual_report_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_sources_column_Params {
	pub sources_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_sources_column_number_Params {
	pub sources_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_destinations_column_Params {
	pub destinations_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_destinations_column_number_Params {
	pub destinations_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_edge_types_column_Params {
	pub edge_type_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_edge_types_column_number_Params {
	pub edge_type_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_weights_column_Params {
	pub weights_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_weights_column_number_Params {
	pub weights_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_verbose_Params {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_node_ids_Params {
	pub numeric_node_ids : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_separator_Params {
	pub separator : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_header_Params {
	pub header : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_directed_Params {
	pub directed : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_name_Params {
	pub name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_all_edge_types_Params {
	pub edge_type : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_all_node_types_Params {
	pub node_type : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Extract_uniform_node_Params {
	pub node : NodeT,
	pub random_state : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Uniform_walk_Params {
	pub node : NodeT,
	pub random_state : u64,
	pub walk_length : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Encode_edge_Params {
	pub src : NodeT,
	pub dst : NodeT,
	pub node_bits : u8,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Encode_max_edge_Params {
	pub node : NodeT,
	pub node_bits : u8,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Decode_edge_Params {
	pub edge : u64,
	pub node_bits : u8,
	pub node_bit_mask : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_bits_Params {
	pub top_node : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Encode_edge_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Decode_edge_Params {
	pub edge : u64,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_sources_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_source_names_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destinations_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destination_names_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_names_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_components_vector_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Unchecked_insert_Params {
	pub value : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Insert_Params {
	pub value : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Unchecked_translate_Params {
	pub id : IndexT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_Params {
	pub id : IndexT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_ids_Params {
	pub numeric_ids : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct From_structs_Params {
	pub ids : Vec<Option<Vec<NodeTypeT>>>,
	pub vocabulary : Option<Vocabulary<NodeTypeT>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Unchecked_translate_Params {
	pub id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_Params {
	pub id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_vector_Params {
	pub ids : Vec<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_ids_Params {
	pub numeric_ids : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct New_Params {
	pub path : S,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_nodes_column_Params {
	pub nodes_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_nodes_column_number_Params {
	pub nodes_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_graph_name_Params {
	pub graph_name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_node_types_column_Params {
	pub nodes_type_column : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_node_types_column_number_Params {
	pub node_types_column_number : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_skip_node_types_if_unavailable_Params {
	pub skip_node_types_if_unavailable : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_might_have_singletons_Params {
	pub might_have_singletons : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_comment_symbol_Params {
	pub comment_symbol : Option<String>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_default_node_type_Params {
	pub default_node_type : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_verbose_Params {
	pub verbose : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_node_type_ids_Params {
	pub numeric_node_type_ids : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_node_ids_Params {
	pub numeric_node_ids : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_ignore_duplicates_Params {
	pub ignore_duplicates : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_separator_Params {
	pub separator : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_node_types_separator_Params {
	pub node_types_separator : Option<S>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_header_Params {
	pub header : Option<bool>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_rows_to_skip_Params {
	pub rows_to_skip : Option<usize>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_max_rows_number_Params {
	pub max_rows_number : Option<u64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct From_structs_Params {
	pub ids : Vec<Option<EdgeTypeT>>,
	pub vocabulary : Vocabulary<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct From_option_structs_Params {
	pub ids : Option<Vec<Option<EdgeTypeT>>>,
	pub vocabulary : Option<Vocabulary<EdgeTypeT>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Unchecked_translate_Params {
	pub id : EdgeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_Params {
	pub id : EdgeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_numeric_ids_Params {
	pub numeric_ids : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_node_neighbours_Params {
	pub src : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_sources_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_sources_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_destinations_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Check_numeric_ids_compatibility_Params {
	pub has_nodes_list : bool,
	pub numeric_node_ids : bool,
	pub numeric_edge_node_ids : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Parse_unsorted_quadruples_Params {
	pub edges : Vec<Quadruple>,
	pub verbose : bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
	pub get_unchecked_weight_from_edge_id: Get_unchecked_weight_from_edge_id_Params,
	pub get_unchecked_edge_degreee_from_node_ids: Get_unchecked_edge_degreee_from_node_ids_Params,
	pub new: New_Params,
	pub new: New_Params,
	pub set_iterations: Set_iterations_Params,
	pub set_max_neighbours: Set_max_neighbours_Params,
	pub set_random_state: Set_random_state_Params,
	pub set_dense_node_mapping: Set_dense_node_mapping_Params,
	pub set_return_weight: Set_return_weight_Params,
	pub set_explore_weight: Set_explore_weight_Params,
	pub set_change_node_type_weight: Set_change_node_type_weight_Params,
	pub set_change_edge_type_weight: Set_change_edge_type_weight_Params,
	pub new: New_Params,
	pub compose_lines: Compose_lines_Params,
	pub new: New_Params,
	pub fast_u32_modulo: Fast_u32_modulo_Params,
	pub not_one: Not_one_Params,
	pub new: New_Params,
	pub get_column_number: Get_column_number_Params,
	pub new: New_Params,
	pub set_sources_column: Set_sources_column_Params,
	pub set_sources_column_number: Set_sources_column_number_Params,
	pub set_destinations_column: Set_destinations_column_Params,
	pub set_destinations_column_number: Set_destinations_column_number_Params,
	pub set_edge_types_column: Set_edge_types_column_Params,
	pub set_edge_types_column_number: Set_edge_types_column_number_Params,
	pub set_weights_column: Set_weights_column_Params,
	pub set_weights_column_number: Set_weights_column_number_Params,
	pub set_skip_weights_if_unavailable: Set_skip_weights_if_unavailable_Params,
	pub set_skip_edge_types_if_unavailable: Set_skip_edge_types_if_unavailable_Params,
	pub set_default_weight: Set_default_weight_Params,
	pub set_graph_name: Set_graph_name_Params,
	pub set_default_edge_type: Set_default_edge_type_Params,
	pub set_skip_self_loops: Set_skip_self_loops_Params,
	pub set_csv_is_correct: Set_csv_is_correct_Params,
	pub set_comment_symbol: Set_comment_symbol_Params,
	pub set_verbose: Set_verbose_Params,
	pub set_might_have_singletons_with_selfloops: Set_might_have_singletons_with_selfloops_Params,
	pub set_might_have_trap_nodes: Set_might_have_trap_nodes_Params,
	pub set_numeric_edge_type_ids: Set_numeric_edge_type_ids_Params,
	pub set_numeric_node_ids: Set_numeric_node_ids_Params,
	pub set_ignore_duplicates: Set_ignore_duplicates_Params,
	pub set_separator: Set_separator_Params,
	pub set_header: Set_header_Params,
	pub set_rows_to_skip: Set_rows_to_skip_Params,
	pub set_max_rows_number: Set_max_rows_number_Params,
	pub parse_edge_line: Parse_edge_line_Params,
	pub is_singleton_from_node_id: Is_singleton_from_node_id_Params,
	pub is_singleton_with_self_loops_from_node_id: Is_singleton_with_self_loops_from_node_id_Params,
	pub has_edge_from_node_ids: Has_edge_from_node_ids_Params,
	pub is_node_trap_from_node_id: Is_node_trap_from_node_id_Params,
	pub random_string: Random_string_Params,
	pub load_ppi: Load_ppi_Params,
	pub load_empty_graph: Load_empty_graph_Params,
	pub new: New_Params,
	pub set_nodes_column: Set_nodes_column_Params,
	pub set_node_types_column: Set_node_types_column_Params,
	pub set_nodes_column_number: Set_nodes_column_number_Params,
	pub set_node_types_column_number: Set_node_types_column_number_Params,
	pub set_verbose: Set_verbose_Params,
	pub set_separator: Set_separator_Params,
	pub set_header: Set_header_Params,
	pub get_unchecked_node_ids_from_edge_id: Get_unchecked_node_ids_from_edge_id_Params,
	pub get_node_ids_from_edge_id: Get_node_ids_from_edge_id_Params,
	pub get_unchecked_edge_id_from_node_ids: Get_unchecked_edge_id_from_node_ids_Params,
	pub get_edge_id_from_node_ids: Get_edge_id_from_node_ids_Params,
	pub get_unchecked_unique_source_node_id: Get_unchecked_unique_source_node_id_Params,
	pub get_top_k_central_nodes_ids: Get_top_k_central_nodes_ids_Params,
	pub get_unchecked_node_degree_from_node_id: Get_unchecked_node_degree_from_node_id_Params,
	pub get_node_degree_from_node_id: Get_node_degree_from_node_id_Params,
	pub get_top_k_central_node_names: Get_top_k_central_node_names_Params,
	pub get_unchecked_edge_type_id_from_edge_id: Get_unchecked_edge_type_id_from_edge_id_Params,
	pub get_weight_from_edge_id: Get_weight_from_edge_id_Params,
	pub get_weight_from_node_ids: Get_weight_from_node_ids_Params,
	pub get_unchecked_node_name_from_node_id: Get_unchecked_node_name_from_node_id_Params,
	pub get_node_name_from_node_id: Get_node_name_from_node_id_Params,
	pub get_unchecked_destination_node_id_from_edge_id: Get_unchecked_destination_node_id_from_edge_id_Params,
	pub get_destination_node_id_from_edge_id: Get_destination_node_id_from_edge_id_Params,
	pub get_node_neighbours_from_node_id: Get_node_neighbours_from_node_id_Params,
	pub get_unchecked_minmax_edge_ids_from_source_node_id: Get_unchecked_minmax_edge_ids_from_source_node_id_Params,
	pub get_minmax_edge_ids_from_source_node_id: Get_minmax_edge_ids_from_source_node_id_Params,
	pub from_usize: From_usize_Params,
	pub new: New_Params,
	pub validate_node_id: Validate_node_id_Params,
	pub validate_edge_id: Validate_edge_id_Params,
	pub degrees_product: Degrees_product_Params,
	pub jaccard_index: Jaccard_index_Params,
	pub adamic_adar_index: Adamic_adar_index_Params,
	pub resource_allocation_index: Resource_allocation_index_Params,
	pub connected_components_number: Connected_components_number_Params,
	pub textual_report: Textual_report_Params,
	pub new: New_Params,
	pub set_sources_column: Set_sources_column_Params,
	pub set_sources_column_number: Set_sources_column_number_Params,
	pub set_destinations_column: Set_destinations_column_Params,
	pub set_destinations_column_number: Set_destinations_column_number_Params,
	pub set_edge_types_column: Set_edge_types_column_Params,
	pub set_edge_types_column_number: Set_edge_types_column_number_Params,
	pub set_weights_column: Set_weights_column_Params,
	pub set_weights_column_number: Set_weights_column_number_Params,
	pub set_verbose: Set_verbose_Params,
	pub set_numeric_node_ids: Set_numeric_node_ids_Params,
	pub set_separator: Set_separator_Params,
	pub set_header: Set_header_Params,
	pub set_directed: Set_directed_Params,
	pub set_name: Set_name_Params,
	pub set_all_edge_types: Set_all_edge_types_Params,
	pub set_all_node_types: Set_all_node_types_Params,
	pub extract_uniform_node: Extract_uniform_node_Params,
	pub uniform_walk: Uniform_walk_Params,
	pub encode_edge: Encode_edge_Params,
	pub encode_max_edge: Encode_max_edge_Params,
	pub decode_edge: Decode_edge_Params,
	pub get_node_bits: Get_node_bits_Params,
	pub encode_edge: Encode_edge_Params,
	pub decode_edge: Decode_edge_Params,
	pub get_sources: Get_sources_Params,
	pub get_source_names: Get_source_names_Params,
	pub get_destinations: Get_destinations_Params,
	pub get_destination_names: Get_destination_names_Params,
	pub get_edges: Get_edges_Params,
	pub get_edge_names: Get_edge_names_Params,
	pub get_node_components_vector: Get_node_components_vector_Params,
	pub unchecked_insert: Unchecked_insert_Params,
	pub insert: Insert_Params,
	pub unchecked_translate: Unchecked_translate_Params,
	pub translate: Translate_Params,
	pub set_numeric_ids: Set_numeric_ids_Params,
	pub from_structs: From_structs_Params,
	pub unchecked_translate: Unchecked_translate_Params,
	pub translate: Translate_Params,
	pub translate_vector: Translate_vector_Params,
	pub set_numeric_ids: Set_numeric_ids_Params,
	pub new: New_Params,
	pub set_nodes_column: Set_nodes_column_Params,
	pub set_nodes_column_number: Set_nodes_column_number_Params,
	pub set_graph_name: Set_graph_name_Params,
	pub set_node_types_column: Set_node_types_column_Params,
	pub set_node_types_column_number: Set_node_types_column_number_Params,
	pub set_skip_node_types_if_unavailable: Set_skip_node_types_if_unavailable_Params,
	pub set_might_have_singletons: Set_might_have_singletons_Params,
	pub set_comment_symbol: Set_comment_symbol_Params,
	pub set_default_node_type: Set_default_node_type_Params,
	pub set_verbose: Set_verbose_Params,
	pub set_numeric_node_type_ids: Set_numeric_node_type_ids_Params,
	pub set_numeric_node_ids: Set_numeric_node_ids_Params,
	pub set_ignore_duplicates: Set_ignore_duplicates_Params,
	pub set_separator: Set_separator_Params,
	pub set_node_types_separator: Set_node_types_separator_Params,
	pub set_header: Set_header_Params,
	pub set_rows_to_skip: Set_rows_to_skip_Params,
	pub set_max_rows_number: Set_max_rows_number_Params,
	pub from_structs: From_structs_Params,
	pub from_option_structs: From_option_structs_Params,
	pub unchecked_translate: Unchecked_translate_Params,
	pub translate: Translate_Params,
	pub set_numeric_ids: Set_numeric_ids_Params,
	pub iter_node_neighbours: Iter_node_neighbours_Params,
	pub iter_sources_ids: Iter_sources_ids_Params,
	pub par_iter_sources_ids: Par_iter_sources_ids_Params,
	pub iter_destinations_ids: Iter_destinations_ids_Params,
	pub check_numeric_ids_compatibility: Check_numeric_ids_compatibility_Params,
	pub parse_unsorted_quadruples: Parse_unsorted_quadruples_Params,
    pub from_vec: FromVecHarnessParams,
}

pub fn meta_test(data: MetaParams) -> Result<(), String> {
    let data_copy = data.clone();
    let data_copy2 = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_mega_test(info, data_copy.clone());
    }));

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        data.from_vec.name,
        data.from_vec.ignore_duplicated_nodes,
        data.from_vec.ignore_duplicated_edges,
        data.from_vec.verbose,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_weights,
    )?;
    
    

    let g_copy = graph.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_mega_test_once_loaded(info, data_copy2.clone(), g_copy.clone());
    }));
    
    graph.get_unchecked_weight_from_edge_id(data.get_unchecked_weight_from_edge_id.edge_id);
	graph.get_unchecked_edge_degreee_from_node_ids(data.get_unchecked_edge_degreee_from_node_ids.src, data.get_unchecked_edge_degreee_from_node_ids.dst);
	graph.is_first_order_walk();
	graph.new(data.new.walk_length)?;
	graph.is_first_order_walk();
	graph.new(data.new.walk_length)?;
	graph.set_iterations(data.set_iterations.iterations)?;
	graph.get_iterations();
	graph.set_max_neighbours(data.set_max_neighbours.max_neighbours)?;
	graph.set_random_state(data.set_random_state.random_state);
	graph.set_dense_node_mapping(data.set_dense_node_mapping.dense_node_mapping);
	graph.set_return_weight(data.set_return_weight.return_weight)?;
	graph.set_explore_weight(data.set_explore_weight.explore_weight)?;
	graph.set_change_node_type_weight(data.set_change_node_type_weight.change_node_type_weight)?;
	graph.set_change_edge_type_weight(data.set_change_edge_type_weight.change_edge_type_weight)?;
	graph.is_first_order_walk();
	graph.new(data.new.path);
	graph.compose_lines(data.compose_lines.number_of_columns, data.compose_lines.pairs);
	graph.new(data.new.directed, data.new.unique_self_loop_number, data.new.self_loop_number, data.new.not_singleton_nodes_number, data.new.singleton_nodes_with_self_loops_number, data.new.unique_edges_number, data.new.edges, data.new.unique_sources, data.new.nodes, data.new.node_bit_mask, data.new.node_bits, data.new.edge_types, data.new.name, data.new.weights, data.new.node_types, data.new.not_singleton_nodes, data.new.singleton_nodes_with_self_loops);
	graph.fast_u32_modulo(data.fast_u32_modulo.val, data.fast_u32_modulo.n);
	graph.not_one(data.not_one.weight);
	graph.new(data.new.path, data.new.list_name)?;
	graph.count_rows();
	graph.get_header()?;
	graph.get_elements_per_line()?;
	graph.get_column_number(data.get_column_number.column_name)?;
	graph.new(data.new.path)?;
	graph.set_sources_column(data.set_sources_column.sources_column)?;
	graph.set_sources_column_number(data.set_sources_column_number.sources_column_number)?;
	graph.set_destinations_column(data.set_destinations_column.destinations_column)?;
	graph.set_destinations_column_number(data.set_destinations_column_number.destinations_column_number)?;
	graph.set_edge_types_column(data.set_edge_types_column.edge_type_column)?;
	graph.set_edge_types_column_number(data.set_edge_types_column_number.edge_types_column_number)?;
	graph.set_weights_column(data.set_weights_column.weights_column)?;
	graph.set_weights_column_number(data.set_weights_column_number.weights_column_number)?;
	graph.set_skip_weights_if_unavailable(data.set_skip_weights_if_unavailable.skip_weights_if_unavailable);
	graph.set_skip_edge_types_if_unavailable(data.set_skip_edge_types_if_unavailable.skip_edge_types_if_unavailable);
	graph.set_default_weight(data.set_default_weight.default_weight);
	graph.set_graph_name(data.set_graph_name.graph_name);
	graph.set_default_edge_type(data.set_default_edge_type.default_edge_type);
	graph.set_skip_self_loops(data.set_skip_self_loops.skip_self_loops);
	graph.set_csv_is_correct(data.set_csv_is_correct.csv_is_correct);
	graph.set_comment_symbol(data.set_comment_symbol.comment_symbol)?;
	graph.set_verbose(data.set_verbose.verbose);
	graph.set_might_have_singletons_with_selfloops(data.set_might_have_singletons_with_selfloops.might_have_singletons_with_selfloops);
	graph.set_might_have_trap_nodes(data.set_might_have_trap_nodes.might_have_trap_nodes);
	graph.set_numeric_edge_type_ids(data.set_numeric_edge_type_ids.numeric_edge_type_ids);
	graph.set_numeric_node_ids(data.set_numeric_node_ids.numeric_node_ids);
	graph.set_ignore_duplicates(data.set_ignore_duplicates.ignore_duplicates);
	graph.set_separator(data.set_separator.separator)?;
	graph.set_header(data.set_header.header);
	graph.set_rows_to_skip(data.set_rows_to_skip.rows_to_skip);
	graph.set_max_rows_number(data.set_max_rows_number.max_rows_number);
	graph.has_edge_types();
	graph.has_weights();
	graph.parse_edge_line(data.parse_edge_line.vals)?;
	graph.is_singleton_from_node_id(data.is_singleton_from_node_id.node_id)?;
	graph.is_singleton_with_self_loops_from_node_id(data.is_singleton_with_self_loops_from_node_id.node_id);
	graph.has_edge_from_node_ids(data.has_edge_from_node_ids.src, data.has_edge_from_node_ids.dst);
	graph.is_node_trap_from_node_id(data.is_node_trap_from_node_id.node_id)?;
	graph.random_string(data.random_string.len);
	graph.load_ppi(data.load_ppi.load_nodes, data.load_ppi.load_edge_types, data.load_ppi.load_weights, data.load_ppi.directed, data.load_ppi.verbose, data.load_ppi.skip_self_loops)?;
	graph.load_empty_graph(data.load_empty_graph.directed);
	graph.disable_all();
	graph.new(data.new.path);
	graph.set_nodes_column(data.set_nodes_column.nodes_column);
	graph.set_node_types_column(data.set_node_types_column.nodes_type_column);
	graph.set_nodes_column_number(data.set_nodes_column_number.nodes_column_number);
	graph.set_node_types_column_number(data.set_node_types_column_number.node_types_column_number);
	graph.set_verbose(data.set_verbose.verbose);
	graph.set_separator(data.set_separator.separator);
	graph.set_header(data.set_header.header);
	graph.strongly_connected_components();
	graph.get_unchecked_node_ids_from_edge_id(data.get_unchecked_node_ids_from_edge_id.edge_id);
	graph.get_node_ids_from_edge_id(data.get_node_ids_from_edge_id.edge_id)?;
	graph.get_unchecked_edge_id_from_node_ids(data.get_unchecked_edge_id_from_node_ids.src, data.get_unchecked_edge_id_from_node_ids.dst);
	graph.get_edge_id_from_node_ids(data.get_edge_id_from_node_ids.src, data.get_edge_id_from_node_ids.dst)?;
	graph.get_unchecked_unique_source_node_id(data.get_unchecked_unique_source_node_id.source_id);
	graph.get_top_k_central_nodes_ids(data.get_top_k_central_nodes_ids.k);
	graph.get_unchecked_node_degree_from_node_id(data.get_unchecked_node_degree_from_node_id.node_id);
	graph.get_node_degree_from_node_id(data.get_node_degree_from_node_id.node_id)?;
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
	graph.get_unchecked_edge_type_id_from_edge_id(data.get_unchecked_edge_type_id_from_edge_id.edge_id);
	graph.get_weight_from_edge_id(data.get_weight_from_edge_id.edge_id)?;
	graph.get_weight_from_node_ids(data.get_weight_from_node_ids.src, data.get_weight_from_node_ids.dst)?;
	graph.get_unchecked_node_name_from_node_id(data.get_unchecked_node_name_from_node_id.node_id);
	graph.get_node_name_from_node_id(data.get_node_name_from_node_id.node_id)?;
	graph.get_unchecked_destination_node_id_from_edge_id(data.get_unchecked_destination_node_id_from_edge_id.edge_id);
	graph.get_destination_node_id_from_edge_id(data.get_destination_node_id_from_edge_id.edge_id)?;
	graph.get_node_neighbours_from_node_id(data.get_node_neighbours_from_node_id.node_id)?;
	graph.get_unchecked_minmax_edge_ids_from_source_node_id(data.get_unchecked_minmax_edge_ids_from_source_node_id.src);
	graph.get_minmax_edge_ids_from_source_node_id(data.get_minmax_edge_ids_from_source_node_id.src)?;
	graph.from_usize(data.from_usize.v);
	graph.new(data.new.val);
	graph.read();
	graph.write();
	graph.clone();
	graph.validate_node_id(data.validate_node_id.node_id)?;
	graph.validate_edge_id(data.validate_edge_id.edge_id)?;
	graph.must_have_node_types()?;
	graph.must_have_edge_types()?;
	graph.must_have_weights()?;
	graph.degrees_product(data.degrees_product.one, data.degrees_product.two)?;
	graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two)?;
	graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two)?;
	graph.resource_allocation_index(data.resource_allocation_index.one, data.resource_allocation_index.two)?;
	graph.traps_rate();
	graph.get_node_degrees_mean()?;
	graph.get_undirected_edges_number();
	graph.get_unique_undirected_edges_number();
	graph.get_edges_number();
	graph.get_unique_edges_number();
	graph.get_node_degrees_median()?;
	graph.get_max_node_degree()?;
	graph.get_min_node_degree()?;
	graph.get_node_degrees_mode()?;
	graph.get_self_loop_number();
	graph.get_unique_self_loop_number();
	graph.get_self_loop_rate()?;
	graph.connected_components_number(data.connected_components_number.verbose);
	graph.get_singleton_nodes_number();
	graph.get_singleton_nodes_with_self_loops_number();
	graph.get_not_singleton_nodes_number();
	graph.get_density()?;
	graph.report();
	graph.textual_report(data.textual_report.verbose)?;
	graph.new(data.new.path);
	graph.set_sources_column(data.set_sources_column.sources_column);
	graph.set_sources_column_number(data.set_sources_column_number.sources_column_number);
	graph.set_destinations_column(data.set_destinations_column.destinations_column);
	graph.set_destinations_column_number(data.set_destinations_column_number.destinations_column_number);
	graph.set_edge_types_column(data.set_edge_types_column.edge_type_column);
	graph.set_edge_types_column_number(data.set_edge_types_column_number.edge_type_column_number);
	graph.set_weights_column(data.set_weights_column.weights_column);
	graph.set_weights_column_number(data.set_weights_column_number.weights_column_number);
	graph.set_verbose(data.set_verbose.verbose);
	graph.set_numeric_node_ids(data.set_numeric_node_ids.numeric_node_ids);
	graph.set_separator(data.set_separator.separator);
	graph.set_header(data.set_header.header);
	graph.set_directed(data.set_directed.directed);
	graph.set_name(data.set_name.name);
	graph.invalidate_report();
	graph.set_all_edge_types(data.set_all_edge_types.edge_type);
	graph.set_all_node_types(data.set_all_node_types.node_type);
	graph.extract_uniform_node(data.extract_uniform_node.node, data.extract_uniform_node.random_state);
	graph.uniform_walk(data.uniform_walk.node, data.uniform_walk.random_state, data.uniform_walk.walk_length);
	graph.encode_edge(data.encode_edge.src, data.encode_edge.dst, data.encode_edge.node_bits);
	graph.encode_max_edge(data.encode_max_edge.node, data.encode_max_edge.node_bits);
	graph.decode_edge(data.decode_edge.edge, data.decode_edge.node_bits, data.decode_edge.node_bit_mask);
	graph.get_node_bits(data.get_node_bits.top_node);
	graph.encode_edge(data.encode_edge.src, data.encode_edge.dst);
	graph.decode_edge(data.decode_edge.edge);
	graph.has_nodes();
	graph.has_edges();
	graph.get_name();
	graph.get_trap_nodes_number();
	graph.has_trap_nodes();
	graph.is_directed();
	graph.has_weights();
	graph.has_edge_types();
	graph.has_selfloops();
	graph.has_singletons();
	graph.has_singleton_nodes_with_self_loops();
	graph.get_sources(data.get_sources.directed);
	graph.get_source_names(data.get_source_names.directed);
	graph.get_destinations(data.get_destinations.directed);
	graph.get_destination_names(data.get_destination_names.directed);
	graph.get_node_names();
	graph.get_nodes();
	graph.get_edge_types()?;
	graph.get_edge_type_names();
	graph.get_node_types()?;
	graph.get_weights()?;
	graph.get_min_weight()?;
	graph.get_max_weight()?;
	graph.get_node_type_names();
	graph.get_unique_directed_edges_number();
	graph.get_max_encodable_edge_number();
	graph.get_nodes_mapping();
	graph.get_edges(data.get_edges.directed);
	graph.get_edge_names(data.get_edge_names.directed);
	graph.has_node_types();
	graph.has_multilabel_node_types();
	graph.get_unknown_node_types_number();
	graph.get_minimum_node_types_number();
	graph.has_unknown_node_types();
	graph.get_unknown_edge_types_number();
	graph.get_minimum_edge_types_number();
	graph.has_unknown_edge_types();
	graph.get_nodes_number();
	graph.get_node_components_vector(data.get_node_components_vector.verbose);
	graph.get_directed_edges_number();
	graph.get_edge_types_number();
	graph.get_node_types_number();
	graph.get_node_degrees();
	graph.get_not_singletons();
	graph.get_dense_node_mapping();
	graph.is_multigraph();
	graph.get_multigraph_edges_number();
	graph.get_outbounds();
	graph.get_unique_source_nodes_number();
	graph.get_edge_type_counts()?;
	graph.get_edge_type_counts_hashmap()?;
	graph.get_node_type_counts()?;
	graph.get_node_type_counts_hashmap()?;
	graph.unchecked_insert(data.unchecked_insert.value);
	graph.insert(data.insert.value)?;
	graph.build_reverse_mapping()?;
	graph.is_empty();
	graph.unchecked_translate(data.unchecked_translate.id);
	graph.translate(data.translate.id)?;
	graph.keys();
	graph.len();
	graph.set_numeric_ids(data.set_numeric_ids.numeric_ids);
	graph.compute_hash();
	graph.from_structs(data.from_structs.ids, data.from_structs.vocabulary);
	graph.build_counts();
	graph.build_reverse_mapping()?;
	graph.is_empty();
	graph.is_multilabel();
	graph.min_node_type_count();
	graph.get_unknown_count();
	graph.unchecked_translate(data.unchecked_translate.id);
	graph.translate(data.translate.id)?;
	graph.translate_vector(data.translate_vector.ids)?;
	graph.keys();
	graph.len();
	graph.set_numeric_ids(data.set_numeric_ids.numeric_ids);
	graph.new(data.new.path)?;
	graph.set_nodes_column(data.set_nodes_column.nodes_column)?;
	graph.set_nodes_column_number(data.set_nodes_column_number.nodes_column_number);
	graph.set_graph_name(data.set_graph_name.graph_name);
	graph.set_node_types_column(data.set_node_types_column.nodes_type_column)?;
	graph.set_node_types_column_number(data.set_node_types_column_number.node_types_column_number);
	graph.set_skip_node_types_if_unavailable(data.set_skip_node_types_if_unavailable.skip_node_types_if_unavailable)?;
	graph.set_might_have_singletons(data.set_might_have_singletons.might_have_singletons)?;
	graph.set_comment_symbol(data.set_comment_symbol.comment_symbol)?;
	graph.set_default_node_type(data.set_default_node_type.default_node_type);
	graph.set_verbose(data.set_verbose.verbose);
	graph.set_numeric_node_type_ids(data.set_numeric_node_type_ids.numeric_node_type_ids);
	graph.set_numeric_node_ids(data.set_numeric_node_ids.numeric_node_ids);
	graph.set_ignore_duplicates(data.set_ignore_duplicates.ignore_duplicates);
	graph.set_separator(data.set_separator.separator)?;
	graph.set_node_types_separator(data.set_node_types_separator.node_types_separator)?;
	graph.set_header(data.set_header.header);
	graph.set_rows_to_skip(data.set_rows_to_skip.rows_to_skip);
	graph.set_max_rows_number(data.set_max_rows_number.max_rows_number);
	graph.has_node_types();
	graph.compute_hash();
	graph.from_structs(data.from_structs.ids, data.from_structs.vocabulary);
	graph.from_option_structs(data.from_option_structs.ids, data.from_option_structs.vocabulary);
	graph.build_counts();
	graph.is_empty();
	graph.unchecked_translate(data.unchecked_translate.id);
	graph.translate(data.translate.id)?;
	graph.keys();
	graph.len();
	graph.set_numeric_ids(data.set_numeric_ids.numeric_ids);
	graph.get_unknown_count();
	graph.min_edge_type_count();
	graph.iter_node_ids();
	graph.par_iter_node_ids();
	graph.iter_node_degrees();
	graph.par_iter_node_degrees();
	graph.iter_singleton_node_ids();
	graph.iter_singleton_with_selfloops_node_ids();
	graph.iter_node_neighbours(data.iter_node_neighbours.src);
	graph.iter_sources_ids(data.iter_sources_ids.directed);
	graph.par_iter_sources_ids(data.par_iter_sources_ids.directed);
	graph.iter_destinations_ids(data.iter_destinations_ids.directed);
	graph.iter_unique_sources();
	graph.compute_hash();
	graph.check_numeric_ids_compatibility(data.check_numeric_ids_compatibility.has_nodes_list, data.check_numeric_ids_compatibility.numeric_node_ids, data.check_numeric_ids_compatibility.numeric_edge_node_ids)?;
	graph.parse_unsorted_quadruples(data.parse_unsorted_quadruples.edges, data.parse_unsorted_quadruples.verbose);

    Ok(())
}