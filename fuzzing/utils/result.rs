
use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromVecHarnessParams {
    pub directed: bool,
    pub directed_edge_list: bool,
    pub ignore_duplicated_nodes: bool,
    pub ignore_duplicated_edges: bool,
    pub verbose: bool,
    pub numeric_edge_types_ids: bool,
    pub numeric_node_ids: bool,
    pub numeric_edge_node_ids: bool,
    pub numeric_node_types_ids: bool,
    pub has_node_types: bool,
    pub has_edge_types: bool,
    pub has_weights: bool,
    pub name: String,
    pub edges: Vec<Result<StringQuadruple, String>>,
    pub nodes: Option<Vec<Result<(String, Option<Vec<String>>), String>>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_spanning_arborescence_kruskal_Params {
	&self : random_state,
	EdgeT : unwanted_edge_types,
	&Option<HashSet<Option<EdgeTypeT>>> : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Spanning_arborescence_kruskal_Params {
	&self : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Spanning_arborescence_Params {
	&self : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Connected_components_Params {
	&self : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct New<s: into<string>>_Params {
	directed : bool,
	unique_self_loop_number : NodeT,
	self_loop_number : EdgeT,
	not_singleton_nodes_number : NodeT,
	singleton_nodes_with_self_loops_number : NodeT,
	unique_edges_number : EdgeT,
	edges : EliasFano,
	unique_sources : EliasFano,
	nodes : Vocabulary<NodeT>,
	node_bit_mask : EdgeT,
	node_bits : u8,
	edge_types : Option<EdgeTypeVocabulary>,
	name : S,
	weights : Option<Vec<WeightT>>,
	node_types : Option<NodeTypeVocabulary>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Overlaps_Params {
	 : other,
}


#[derive(Arbitrary, Debug, Clone)]
struct Contains_Params {
	 : other,
}


#[derive(Arbitrary, Debug, Clone)]
struct Node2vec<'a>_Params {
	&'a self : walk_parameters,
	&'a WalksParameters : quantity,
	NodeT : window_size,
	usize : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Cooccurence_matrix_Params {
	&self : walks_parameters,
	&WalksParameters : window_size,
	usize : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Link_prediction_degrees<'a>_Params {
	&'a self : idx,
	u64 : batch_size,
	usize : normalize,
	bool : negative_samples,
	f64 : avoid_false_negatives,
	bool : maximal_sampling_attempts,
	usize : graph_to_avoid,
	&'a Option<&Graph> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Link_prediction_ids<'a>_Params {
	&'a self : idx,
	u64 : batch_size,
	usize : negative_samples,
	f64 : avoid_false_negatives,
	bool : maximal_sampling_attempts,
	usize : graph_to_avoid,
	&'a Option<&Graph> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Are_nodes_remappable_Params {
	 : other,
}


#[derive(Arbitrary, Debug, Clone)]
struct Remap_Params {
	 : other,
	&Graph : verbose,
}


#[derive(Arbitrary, Debug, Clone)]
struct Set_all_edge_types<s: into<string>>_Params {
	self : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Set_all_node_types<s: into<string>>_Params {
	self : node_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Enable_Params {
	&mut self : vector_sources,
	bool : vector_destinations,
	bool : vector_outbounds,
	bool : cache_size,
	Option<f64> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Filter_Params {
	&self : node_names,
	Option<Vec<String>> : node_types,
	Option<Vec<Option<String>>> : edge_types,
	Option<Vec<Option<String>>> : min_weight,
	Option<WeightT> : max_weight,
	Option<WeightT> : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_filtered_neighbours_range_Params {
	&self : src,
	NodeT : node_names,
	Option<Vec<String>> : node_types,
	Option<Vec<Option<String>>> : edge_types,
	Option<Vec<Option<String>>> : min_weight,
	Option<WeightT> : max_weight,
	Option<WeightT> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Degrees_product_Params {
	 : one,
	NodeT : two,
}


#[derive(Arbitrary, Debug, Clone)]
struct Jaccard_index_Params {
	 : one,
	NodeT : two,
}


#[derive(Arbitrary, Debug, Clone)]
struct Adamic_adar_index_Params {
	 : one,
	NodeT : two,
}


#[derive(Arbitrary, Debug, Clone)]
struct Resource_allocation_index_Params {
	 : one,
	NodeT : two,
}


#[derive(Arbitrary, Debug, Clone)]
struct Connected_components_number_Params {
	 : verbose,
}


#[derive(Arbitrary, Debug, Clone)]
struct Overlap_textual_report_Params {
	 : other,
	&Graph : verbose,
}


#[derive(Arbitrary, Debug, Clone)]
struct Textual_report_Params {
	 : verbose,
}


#[derive(Arbitrary, Debug, Clone)]
struct Remove_Params {
	&self : allow_nodes_set,
	Option<HashSet<String>> : deny_nodes_set,
	Option<HashSet<String>> : allow_node_types_set,
	Option<HashSet<String>> : deny_node_types_set,
	Option<HashSet<String>> : allow_edge_set,
	Option<HashSet<EdgeT>> : deny_edge_set,
	Option<HashSet<EdgeT>> : allow_edge_types_set,
	Option<HashSet<String>> : deny_edge_types_set,
	Option<HashSet<String>> : weights,
	bool : node_types,
	bool : edge_types,
	bool : singletons,
	bool : selfloops,
	bool : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Remove_components_Params {
	&self : node_names,
	Option<Vec<String>> : node_types,
	Option<Vec<Option<String>>> : edge_types,
	Option<Vec<Option<String>>> : minimum_component_size,
	Option<NodeT> : top_k_components,
	Option<NodeT> : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Extract_uniform_node_Params {
	 : node,
	NodeT : random_state,
}


#[derive(Arbitrary, Debug, Clone)]
struct Extract_node_Params {
	&self : node,
	NodeT : random_state,
	NodeT : walk_weights,
	&WalkWeights : min_edge_id,
	EdgeT : max_edge_id,
	EdgeT : destinations,
	&[NodeT] : probabilistic_indices,
	&Option<Vec<u64>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Extract_edge_Params {
	&self : src,
	NodeT : dst,
	NodeT : edge,
	EdgeT : random_state,
	NodeT : walk_weights,
	&WalkWeights : min_edge_id,
	EdgeT : max_edge_id,
	EdgeT : destinations,
	&[NodeT] : previous_destinations,
	&[NodeT] : probabilistic_indices,
	&Option<Vec<u64>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_walks_iter<'a>_Params {
	&'a self : quantity,
	NodeT : parameters,
	&'a WalksParameters : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Complete_walks_iter<'a>_Params {
	&'a self : parameters,
	&'a WalksParameters : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Walk_iter<'a>_Params {
	&'a self : quantity,
	NodeT : to_node,
}


#[derive(Arbitrary, Debug, Clone)]
struct Single_walk_Params {
	&self : node,
	NodeT : random_state,
	NodeT : parameters,
	&SingleWalkParameters : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_bipartite_edges_Params {
	&self : removed_existing_edges,
	Option<bool> : first_nodes_set,
	Option<HashSet<String>> : second_nodes_set,
	Option<HashSet<String>> : first_node_types_set,
	Option<HashSet<String>> : second_node_types_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_bipartite_edge_names_Params {
	&self : removed_existing_edges,
	Option<bool> : first_nodes_set,
	Option<HashSet<String>> : second_nodes_set,
	Option<HashSet<String>> : first_node_types_set,
	Option<HashSet<String>> : second_node_types_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_star_edges_Params {
	&self : central_node,
	String : removed_existing_edges,
	Option<bool> : star_points_nodes_set,
	Option<HashSet<String>> : star_points_node_types_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_star_edge_names_Params {
	&self : central_node,
	String : removed_existing_edges,
	Option<bool> : star_points_nodes_set,
	Option<HashSet<String>> : star_points_node_types_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_clique_edges_Params {
	&self : directed,
	Option<bool> : allow_self_loops,
	Option<bool> : removed_existing_edges,
	Option<bool> : allow_node_type_set,
	Option<HashSet<String>> : allow_node_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_clique_edge_names_Params {
	&self : directed,
	Option<bool> : allow_self_loops,
	Option<bool> : removed_existing_edges,
	Option<bool> : allow_node_type_set,
	Option<HashSet<String>> : allow_node_set,
	Option<HashSet<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_singleton_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_singleton_with_self_loops_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct is_singleton_by_nide_name_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_sources_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_source_names_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destination_names_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_top_k_central_nodes_Params {
	 : k,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_top_k_central_node_names_Params {
	 : k,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_name_Params {
	 : edge_type_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_node_type_id_vector_Params {
	&self : node_type_id,
	Vec<NodeTypeT> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_node_type_id_Params {
	 : node_type_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_names_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_type_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_weight_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_node_type_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_id_by_node_id_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_name_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_name_by_edge_id_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_name_Params {
	 : node_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_id_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_id_by_node_name_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_name_by_node_name_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_node_by_name_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_node_id_Params {
	 : node_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_type_id_Params {
	 : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_weight_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_components_vector_Params {
	 : verbose,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_count_by_edge_type_Params {
	 : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_count_by_edge_type_Params {
	 : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_id_Params {
	 : edge_type_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_count_by_edge_type_name_Params {
	 : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_node_count_by_node_type_Params {
	 : node_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_id_Params {
	 : node_type_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_count_by_node_type_Params {
	 : node_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_count_by_node_type_name_Params {
	 : node_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destination_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_range_Params {
	&self : min_edge_id,
	EdgeT : max_edge_id,
	EdgeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_neighbours_iter_Params {
	 : src,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_id_Params {
	&self : src,
	NodeT : dst,
	NodeT : edge_type,
	Option<EdgeTypeT> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_id_Params {
	&self : src,
	NodeT : dst,
	NodeT : edge_type,
	Option<EdgeTypeT> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_edge_Params {
	 : src,
	NodeT : dst,
	NodeT : edge_type,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_id_by_node_names_Params {
	&self : src_name,
	&str : dst_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_edge_by_node_names_Params {
	&self : src_name,
	&str : dst_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_id_with_type_by_node_names_Params {
	&self : src_name,
	&str : dst_name,
	&str : edge_type_name,
	Option<&String> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_edge_types_Params {
	&self : edge_types,
	Vec<Option<String>> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_node_types_Params {
	 : node_types,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_edge_with_type_by_node_names_Params {
	&self : src_name,
	&str : dst_name,
	&str : edge_type_name,
	Option<&String> : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_node_with_type_by_name_Params {
	 : node_name,
	&str : node_type_name,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_degree_Params {
	 : node,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_ids_range_Params {
	&self : src,
	NodeT : dst,
	NodeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_destinations_range_Params {
	 : src,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_ids_Params {
	 : src,
	NodeT : dst,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_link_edge_types_Params {
	&self : src,
	NodeT : dst,
	NodeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_link_weights_Params {
	 : src,
	NodeT : dst,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_node_trap_Params {
	 : node,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_edge_trap_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_nodes_names_iter_Params {
	&self : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_sources_iter_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_sources_par_iter_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_iter_Params {
	 : directed,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_par_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_triples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_triples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_triples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_quadruples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_quadruples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_triples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_quadruples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_quadruples_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_quadruple_Params {
	&self : edge_id,
	EdgeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_triple_Params {
	 : edge_id,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unique_edges_iter_Params {
	&self : directed,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Sample_negatives_Params {
	&self : random_state,
	EdgeT : negatives_number,
	EdgeT : seed_graph,
	Option<&Graph> : only_from_same_component,
	bool : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Connected_holdout_Params {
	&self : random_state,
	EdgeT : train_size,
	f64 : edge_types,
	Option<Vec<Option<String>>> : include_all_edge_types,
	bool : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_holdout_Params {
	&self : random_state,
	EdgeT : train_size,
	f64 : include_all_edge_types,
	bool : edge_types,
	Option<Vec<Option<String>>> : min_number_overlaps,
	Option<EdgeT> : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Node_label_holdout_Params {
	&self : train_size,
	f64 : use_stratification,
	bool : random_state,
	EdgeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Edge_label_holdout_Params {
	&self : train_size,
	f64 : use_stratification,
	bool : random_state,
	EdgeT : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_subgraph_Params {
	&self : random_state,
	usize : nodes_number,
	NodeT : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct Kfold_Params {
	&self : k,
	EdgeT : k_index,
	u64 : edge_types,
	Option<Vec<Option<String>>> : random_state,
	EdgeT : verbose,
	bool : ,
}


#[derive(Arbitrary, Debug, Clone)]
struct TheUltimateFuzzer {
	random_spanning_arborescence_kruskal : Random_spanning_arborescence_kruskal_Params,
	spanning_arborescence_kruskal : Spanning_arborescence_kruskal_Params,
	spanning_arborescence : Spanning_arborescence_Params,
	connected_components : Connected_components_Params,
	new<S: Into<String>> : New<s: into<string>>_Params,
	overlaps : Overlaps_Params,
	contains : Contains_Params,
	node2vec<'a> : Node2vec<'a>_Params,
	cooccurence_matrix : Cooccurence_matrix_Params,
	link_prediction_degrees<'a> : Link_prediction_degrees<'a>_Params,
	link_prediction_ids<'a> : Link_prediction_ids<'a>_Params,
	are_nodes_remappable : Are_nodes_remappable_Params,
	remap : Remap_Params,
	set_all_edge_types<S: Into<String>> : Set_all_edge_types<s: into<string>>_Params,
	set_all_node_types<S: Into<String>> : Set_all_node_types<s: into<string>>_Params,
	enable : Enable_Params,
	filter : Filter_Params,
	get_filtered_neighbours_range : Get_filtered_neighbours_range_Params,
	degrees_product : Degrees_product_Params,
	jaccard_index : Jaccard_index_Params,
	adamic_adar_index : Adamic_adar_index_Params,
	resource_allocation_index : Resource_allocation_index_Params,
	connected_components_number : Connected_components_number_Params,
	overlap_textual_report : Overlap_textual_report_Params,
	textual_report : Textual_report_Params,
	remove : Remove_Params,
	remove_components : Remove_components_Params,
	extract_uniform_node : Extract_uniform_node_Params,
	extract_node : Extract_node_Params,
	extract_edge : Extract_edge_Params,
	random_walks_iter<'a> : Random_walks_iter<'a>_Params,
	complete_walks_iter<'a> : Complete_walks_iter<'a>_Params,
	walk_iter<'a> : Walk_iter<'a>_Params,
	single_walk : Single_walk_Params,
	get_bipartite_edges : Get_bipartite_edges_Params,
	get_bipartite_edge_names : Get_bipartite_edge_names_Params,
	get_star_edges : Get_star_edges_Params,
	get_star_edge_names : Get_star_edge_names_Params,
	get_clique_edges : Get_clique_edges_Params,
	get_clique_edge_names : Get_clique_edge_names_Params,
	is_singleton : Is_singleton_Params,
	is_singleton_with_self_loops : Is_singleton_with_self_loops_Params,
	is_singleton_by_nide_name : is_singleton_by_nide_name_Params,
	get_sources : Get_sources_Params,
	get_source_names : Get_source_names_Params,
	get_destinations : Get_destinations_Params,
	get_destination_names : Get_destination_names_Params,
	get_top_k_central_nodes : Get_top_k_central_nodes_Params,
	get_top_k_central_node_names : Get_top_k_central_node_names_Params,
	get_edge_type_name : Get_edge_type_name_Params,
	translate_node_type_id_vector : Translate_node_type_id_vector_Params,
	translate_node_type_id : Translate_node_type_id_Params,
	get_edges : Get_edges_Params,
	get_edge_names : Get_edge_names_Params,
	get_unchecked_edge_type : Get_unchecked_edge_type_Params,
	get_unchecked_edge_weight : Get_unchecked_edge_weight_Params,
	get_unchecked_node_type : Get_unchecked_node_type_Params,
	get_node_type_id_by_node_id : Get_node_type_id_by_node_id_Params,
	get_edge_type : Get_edge_type_Params,
	get_node_type_name : Get_node_type_name_Params,
	get_edge_type_name_by_edge_id : Get_edge_type_name_by_edge_id_Params,
	get_node_name : Get_node_name_Params,
	get_node_id : Get_node_id_Params,
	get_node_type_id_by_node_name : Get_node_type_id_by_node_name_Params,
	get_node_type_name_by_node_name : Get_node_type_name_by_node_name_Params,
	has_node_by_name : Has_node_by_name_Params,
	get_unchecked_node_id : Get_unchecked_node_id_Params,
	get_unchecked_edge_type_id : Get_unchecked_edge_type_id_Params,
	get_edge_weight : Get_edge_weight_Params,
	get_node_components_vector : Get_node_components_vector_Params,
	get_unchecked_edge_count_by_edge_type : Get_unchecked_edge_count_by_edge_type_Params,
	get_edge_count_by_edge_type : Get_edge_count_by_edge_type_Params,
	get_edge_type_id : Get_edge_type_id_Params,
	get_edge_count_by_edge_type_name : Get_edge_count_by_edge_type_name_Params,
	get_unchecked_node_count_by_node_type : Get_unchecked_node_count_by_node_type_Params,
	get_node_type_id : Get_node_type_id_Params,
	get_node_count_by_node_type : Get_node_count_by_node_type_Params,
	get_node_count_by_node_type_name : Get_node_count_by_node_type_name_Params,
	get_destination : Get_destination_Params,
	get_destinations_range : Get_destinations_range_Params,
	get_neighbours_iter : Get_neighbours_iter_Params,
	get_unchecked_edge_id : Get_unchecked_edge_id_Params,
	get_edge_id : Get_edge_id_Params,
	has_edge : Has_edge_Params,
	get_edge_id_by_node_names : Get_edge_id_by_node_names_Params,
	has_edge_by_node_names : Has_edge_by_node_names_Params,
	get_edge_id_with_type_by_node_names : Get_edge_id_with_type_by_node_names_Params,
	translate_edge_types : Translate_edge_types_Params,
	translate_node_types : Translate_node_types_Params,
	has_edge_with_type_by_node_names : Has_edge_with_type_by_node_names_Params,
	has_node_with_type_by_name : Has_node_with_type_by_name_Params,
	get_node_degree : Get_node_degree_Params,
	get_unchecked_edge_ids_range : Get_unchecked_edge_ids_range_Params,
	get_unchecked_destinations_range : Get_unchecked_destinations_range_Params,
	get_edge_ids : Get_edge_ids_Params,
	get_unchecked_link_edge_types : Get_unchecked_link_edge_types_Params,
	get_unchecked_link_weights : Get_unchecked_link_weights_Params,
	is_node_trap : Is_node_trap_Params,
	is_edge_trap : Is_edge_trap_Params,
	get_nodes_names_iter : Get_nodes_names_iter_Params,
	get_edges_iter : Get_edges_iter_Params,
	get_sources_iter : Get_sources_iter_Params,
	get_sources_par_iter : Get_sources_par_iter_Params,
	get_destinations_iter : Get_destinations_iter_Params,
	get_destinations_par_iter : Get_destinations_par_iter_Params,
	get_edges_string_iter : Get_edges_string_iter_Params,
	get_edges_par_iter : Get_edges_par_iter_Params,
	get_edges_par_string_iter : Get_edges_par_string_iter_Params,
	get_edges_triples : Get_edges_triples_Params,
	get_edges_string_triples : Get_edges_string_triples_Params,
	get_edges_par_string_triples : Get_edges_par_string_triples_Params,
	get_edges_par_string_quadruples : Get_edges_par_string_quadruples_Params,
	get_edges_string_quadruples : Get_edges_string_quadruples_Params,
	get_edges_par_triples : Get_edges_par_triples_Params,
	get_edges_quadruples : Get_edges_quadruples_Params,
	get_edges_par_quadruples : Get_edges_par_quadruples_Params,
	get_edge_quadruple : Get_edge_quadruple_Params,
	get_edge_triple : Get_edge_triple_Params,
	get_unique_edges_iter : Get_unique_edges_iter_Params,
	sample_negatives : Sample_negatives_Params,
	connected_holdout : Connected_holdout_Params,
	random_holdout : Random_holdout_Params,
	node_label_holdout : Node_label_holdout_Params,
	edge_label_holdout : Edge_label_holdout_Params,
	random_subgraph : Random_subgraph_Params,
	kfold : Kfold_Params,
	from_vec: FromVecHarnessParams,
}

pub fn mega_test(data: TheUltimateFuzzer) -> Result<(), String> {
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
    
    
	graph.random_spanning_arborescence_kruskal(data.random_spanning_arborescence_kruskal.&self, data.random_spanning_arborescence_kruskal.EdgeT, data.random_spanning_arborescence_kruskal.&Option<HashSet<Option<EdgeTypeT>>>, data.random_spanning_arborescence_kruskal.bool);
	graph.spanning_arborescence_kruskal(data.spanning_arborescence_kruskal.&self, data.spanning_arborescence_kruskal.bool);
	graph.spanning_arborescence(data.spanning_arborescence.&self, data.spanning_arborescence.bool)?;
	graph.connected_components(data.connected_components.&self, data.connected_components.bool)?;
	graph.new<S: Into<String>>(data.new<S: Into<String>>.directed, data.new<S: Into<String>>.unique_self_loop_number, data.new<S: Into<String>>.self_loop_number, data.new<S: Into<String>>.not_singleton_nodes_number, data.new<S: Into<String>>.singleton_nodes_with_self_loops_number, data.new<S: Into<String>>.unique_edges_number, data.new<S: Into<String>>.edges, data.new<S: Into<String>>.unique_sources, data.new<S: Into<String>>.nodes, data.new<S: Into<String>>.node_bit_mask, data.new<S: Into<String>>.node_bits, data.new<S: Into<String>>.edge_types, data.new<S: Into<String>>.name, data.new<S: Into<String>>.weights, data.new<S: Into<String>>.node_types);
	graph.overlaps(data.overlaps.)?;
	graph.contains(data.contains.)?;
	graph.node2vec<'a>(data.node2vec<'a>.&'a self, data.node2vec<'a>.&'a WalksParameters, data.node2vec<'a>.NodeT, data.node2vec<'a>.usize)?;
	graph.cooccurence_matrix(data.cooccurence_matrix.&self, data.cooccurence_matrix.&WalksParameters, data.cooccurence_matrix.usize, data.cooccurence_matrix.bool)?;
	graph.link_prediction_degrees<'a>(data.link_prediction_degrees<'a>.&'a self, data.link_prediction_degrees<'a>.u64, data.link_prediction_degrees<'a>.usize, data.link_prediction_degrees<'a>.bool, data.link_prediction_degrees<'a>.f64, data.link_prediction_degrees<'a>.bool, data.link_prediction_degrees<'a>.usize, data.link_prediction_degrees<'a>.&'a Option<&Graph>)?;
	graph.link_prediction_ids<'a>(data.link_prediction_ids<'a>.&'a self, data.link_prediction_ids<'a>.u64, data.link_prediction_ids<'a>.usize, data.link_prediction_ids<'a>.f64, data.link_prediction_ids<'a>.bool, data.link_prediction_ids<'a>.usize, data.link_prediction_ids<'a>.&'a Option<&Graph>)?;


	graph.are_nodes_remappable(data.are_nodes_remappable.);
	graph.remap(data.remap., data.remap.&Graph)?;
	graph.set_all_edge_types<S: Into<String>>(data.set_all_edge_types<S: Into<String>>.self)?;
	graph.set_all_node_types<S: Into<String>>(data.set_all_node_types<S: Into<String>>.self);
	graph.enable(data.enable.&mut self, data.enable.bool, data.enable.bool, data.enable.bool, data.enable.Option<f64>)?;
	graph.strongly_connected_components();
	graph.filter(data.filter.&self, data.filter.Option<Vec<String>>, data.filter.Option<Vec<Option<String>>>, data.filter.Option<Vec<Option<String>>>, data.filter.Option<WeightT>, data.filter.Option<WeightT>, data.filter.bool)?;
	graph.get_filtered_neighbours_range(data.get_filtered_neighbours_range.&self, data.get_filtered_neighbours_range.NodeT, data.get_filtered_neighbours_range.Option<Vec<String>>, data.get_filtered_neighbours_range.Option<Vec<Option<String>>>, data.get_filtered_neighbours_range.Option<Vec<Option<String>>>, data.get_filtered_neighbours_range.Option<WeightT>, data.get_filtered_neighbours_range.Option<WeightT>)?;
	graph.degrees_product(data.degrees_product., data.degrees_product.NodeT)?;
	graph.jaccard_index(data.jaccard_index., data.jaccard_index.NodeT)?;
	graph.adamic_adar_index(data.adamic_adar_index., data.adamic_adar_index.NodeT)?;
	graph.resource_allocation_index(data.resource_allocation_index., data.resource_allocation_index.NodeT)?;
	graph.traps_rate();
	graph.degrees_mean();
	graph.get_undirected_edges_number();
	graph.get_edges_number();
	graph.degrees_median();
	graph.max_degree();
	graph.min_degree();
	graph.degrees_mode();
	graph.get_self_loop_number();
	graph.get_unique_self_loop_number();
	graph.get_self_loop_rate();
	graph.connected_components_number(data.connected_components_number.);
	graph.get_singleton_nodes_number();
	graph.get_singleton_nodes_with_self_loops_number();
	graph.get_not_singleton_nodes_number();
	graph.density();
	graph.report();
	graph.overlap_textual_report(data.overlap_textual_report., data.overlap_textual_report.&Graph)?;
	graph.textual_report(data.textual_report.)?;
	graph.remove(data.remove.&self, data.remove.Option<HashSet<String>>, data.remove.Option<HashSet<String>>, data.remove.Option<HashSet<String>>, data.remove.Option<HashSet<String>>, data.remove.Option<HashSet<EdgeT>>, data.remove.Option<HashSet<EdgeT>>, data.remove.Option<HashSet<String>>, data.remove.Option<HashSet<String>>, data.remove.bool, data.remove.bool, data.remove.bool, data.remove.bool, data.remove.bool, data.remove.bool)?;
	graph.remove_components(data.remove_components.&self, data.remove_components.Option<Vec<String>>, data.remove_components.Option<Vec<Option<String>>>, data.remove_components.Option<Vec<Option<String>>>, data.remove_components.Option<NodeT>, data.remove_components.Option<NodeT>, data.remove_components.bool)?;
	graph.extract_uniform_node(data.extract_uniform_node., data.extract_uniform_node.NodeT);
	graph.extract_node(data.extract_node.&self, data.extract_node.NodeT, data.extract_node.NodeT, data.extract_node.&WalkWeights, data.extract_node.EdgeT, data.extract_node.EdgeT, data.extract_node.&[NodeT], data.extract_node.&Option<Vec<u64>>);
	graph.extract_edge(data.extract_edge.&self, data.extract_edge.NodeT, data.extract_edge.NodeT, data.extract_edge.EdgeT, data.extract_edge.NodeT, data.extract_edge.&WalkWeights, data.extract_edge.EdgeT, data.extract_edge.EdgeT, data.extract_edge.&[NodeT], data.extract_edge.&[NodeT], data.extract_edge.&Option<Vec<u64>>);
	graph.random_walks_iter<'a>(data.random_walks_iter<'a>.&'a self, data.random_walks_iter<'a>.NodeT, data.random_walks_iter<'a>.&'a WalksParameters)?;
	graph.complete_walks_iter<'a>(data.complete_walks_iter<'a>.&'a self, data.complete_walks_iter<'a>.&'a WalksParameters)?;
	graph.walk_iter<'a>(data.walk_iter<'a>.&'a self, data.walk_iter<'a>.NodeT);
	graph.single_walk(data.single_walk.&self, data.single_walk.NodeT, data.single_walk.NodeT, data.single_walk.&SingleWalkParameters);
	graph.get_bipartite_edges(data.get_bipartite_edges.&self, data.get_bipartite_edges.Option<bool>, data.get_bipartite_edges.Option<HashSet<String>>, data.get_bipartite_edges.Option<HashSet<String>>, data.get_bipartite_edges.Option<HashSet<String>>, data.get_bipartite_edges.Option<HashSet<String>>)?;
	graph.get_bipartite_edge_names(data.get_bipartite_edge_names.&self, data.get_bipartite_edge_names.Option<bool>, data.get_bipartite_edge_names.Option<HashSet<String>>, data.get_bipartite_edge_names.Option<HashSet<String>>, data.get_bipartite_edge_names.Option<HashSet<String>>, data.get_bipartite_edge_names.Option<HashSet<String>>)?;
	graph.get_star_edges(data.get_star_edges.&self, data.get_star_edges.String, data.get_star_edges.Option<bool>, data.get_star_edges.Option<HashSet<String>>, data.get_star_edges.Option<HashSet<String>>)?;
	graph.get_star_edge_names(data.get_star_edge_names.&self, data.get_star_edge_names.String, data.get_star_edge_names.Option<bool>, data.get_star_edge_names.Option<HashSet<String>>, data.get_star_edge_names.Option<HashSet<String>>)?;
	graph.get_clique_edges(data.get_clique_edges.&self, data.get_clique_edges.Option<bool>, data.get_clique_edges.Option<bool>, data.get_clique_edges.Option<bool>, data.get_clique_edges.Option<HashSet<String>>, data.get_clique_edges.Option<HashSet<String>>);
	graph.get_clique_edge_names(data.get_clique_edge_names.&self, data.get_clique_edge_names.Option<bool>, data.get_clique_edge_names.Option<bool>, data.get_clique_edge_names.Option<bool>, data.get_clique_edge_names.Option<HashSet<String>>, data.get_clique_edge_names.Option<HashSet<String>>);
	graph.get_name();
	graph.get_traps_number();
	graph.has_traps();
	graph.is_directed();
	graph.has_weights();
	graph.has_edge_types();
	graph.has_selfloops();
	graph.is_singleton(data.is_singleton.);
	graph.is_singleton_with_self_loops(data.is_singleton_with_self_loops.);
	graph.is_singleton_by_nide_name(data.is_singleton_by_nide_name.)?;
	graph.has_singletons();
	graph.has_singleton_nodes_with_self_loops_number();
	graph.get_sources(data.get_sources.);
	graph.get_source_names(data.get_source_names.);
	graph.get_destinations(data.get_destinations.);
	graph.get_destination_names(data.get_destination_names.);
	graph.get_node_names();
	graph.get_nodes();
	graph.get_top_k_central_nodes(data.get_top_k_central_nodes.);
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.);
	graph.get_edge_types();
	graph.get_edge_type_name(data.get_edge_type_name.);
	graph.get_edge_type_names();
	graph.get_node_types();
	graph.translate_node_type_id_vector(data.translate_node_type_id_vector.&self, data.translate_node_type_id_vector.Vec<NodeTypeT>);
	graph.translate_node_type_id(data.translate_node_type_id.);
	graph.get_weights();
	graph.get_node_type_names();
	graph.get_unique_edges_number();
	graph.get_max_encodable_edge_number();
	graph.get_nodes_mapping();
	graph.get_edges(data.get_edges.);
	graph.get_edge_names(data.get_edge_names.);
	graph.get_unchecked_edge_type(data.get_unchecked_edge_type.);
	graph.get_unchecked_edge_weight(data.get_unchecked_edge_weight.);
	graph.get_unchecked_node_type(data.get_unchecked_node_type.);
	graph.get_node_type_id_by_node_id(data.get_node_type_id_by_node_id.)?;
	graph.get_edge_type(data.get_edge_type.)?;
	graph.get_node_type_name(data.get_node_type_name.);
	graph.get_edge_type_name_by_edge_id(data.get_edge_type_name_by_edge_id.);
	graph.get_node_name(data.get_node_name.)?;
	graph.get_node_id(data.get_node_id.)?;
	graph.get_node_type_id_by_node_name(data.get_node_type_id_by_node_name.)?;
	graph.get_node_type_name_by_node_name(data.get_node_type_name_by_node_name.)?;
	graph.has_node_by_name(data.has_node_by_name.);
	graph.get_unchecked_node_id(data.get_unchecked_node_id.);
	graph.get_unchecked_edge_type_id(data.get_unchecked_edge_type_id.);
	graph.get_edge_weight(data.get_edge_weight.);
	graph.has_node_types();
	graph.has_multilabel_node_types();
	graph.get_unknown_node_types_number();
	graph.get_minimum_node_types_number();
	graph.has_unknown_node_types();
	graph.get_unknown_edge_types_number();
	graph.get_minimum_edge_types_number();
	graph.has_unknown_edge_types();
	graph.get_nodes_number();
	graph.get_node_components_vector(data.get_node_components_vector.);
	graph.get_directed_edges_number();
	graph.get_edge_types_number();
	graph.get_node_types_number();
	graph.get_node_degrees();
	graph.get_not_singletons();
	graph.get_dense_node_mapping();
	graph.get_unchecked_edge_count_by_edge_type(data.get_unchecked_edge_count_by_edge_type.);
	graph.get_edge_count_by_edge_type(data.get_edge_count_by_edge_type.)?;
	graph.get_edge_type_id(data.get_edge_type_id.)?;
	graph.get_edge_count_by_edge_type_name(data.get_edge_count_by_edge_type_name.)?;
	graph.get_unchecked_node_count_by_node_type(data.get_unchecked_node_count_by_node_type.);
	graph.get_node_type_id(data.get_node_type_id.)?;
	graph.get_node_count_by_node_type(data.get_node_count_by_node_type.)?;
	graph.get_node_count_by_node_type_name(data.get_node_count_by_node_type_name.)?;
	graph.is_multigraph();
	graph.get_multigraph_edges_number();
	graph.get_outbounds();
	graph.get_destination(data.get_destination.);
	graph.get_destinations_range(data.get_destinations_range.&self, data.get_destinations_range.EdgeT, data.get_destinations_range.EdgeT);
	graph.get_neighbours_iter(data.get_neighbours_iter.);
	graph.get_unique_sources_number();
	graph.get_source_nodes_number();
	graph.get_unchecked_edge_id(data.get_unchecked_edge_id.&self, data.get_unchecked_edge_id.NodeT, data.get_unchecked_edge_id.NodeT, data.get_unchecked_edge_id.Option<EdgeTypeT>);
	graph.get_edge_id(data.get_edge_id.&self, data.get_edge_id.NodeT, data.get_edge_id.NodeT, data.get_edge_id.Option<EdgeTypeT>)?;
	graph.has_edge(data.has_edge., data.has_edge.NodeT, data.has_edge.NodeT);
	graph.get_edge_id_by_node_names(data.get_edge_id_by_node_names.&self, data.get_edge_id_by_node_names.&str)?;
	graph.has_edge_by_node_names(data.has_edge_by_node_names.&self, data.has_edge_by_node_names.&str);
	graph.get_edge_id_with_type_by_node_names(data.get_edge_id_with_type_by_node_names.&self, data.get_edge_id_with_type_by_node_names.&str, data.get_edge_id_with_type_by_node_names.&str, data.get_edge_id_with_type_by_node_names.Option<&String>)?;
	graph.get_edge_type_counts()?;
	graph.get_edge_type_counts_hashmap()?;
	graph.translate_edge_types(data.translate_edge_types.&self, data.translate_edge_types.Vec<Option<String>>)?;
	graph.translate_node_types(data.translate_node_types.)?;
	graph.get_node_type_counts()?;
	graph.get_node_type_counts_hashmap()?;
	graph.has_edge_with_type_by_node_names(data.has_edge_with_type_by_node_names.&self, data.has_edge_with_type_by_node_names.&str, data.has_edge_with_type_by_node_names.&str, data.has_edge_with_type_by_node_names.Option<&String>);
	graph.has_node_with_type_by_name(data.has_node_with_type_by_name., data.has_node_with_type_by_name.&str);
	graph.get_node_degree(data.get_node_degree.);
	graph.get_unchecked_edge_ids_range(data.get_unchecked_edge_ids_range.&self, data.get_unchecked_edge_ids_range.NodeT, data.get_unchecked_edge_ids_range.NodeT);
	graph.get_unchecked_destinations_range(data.get_unchecked_destinations_range.);
	graph.get_edge_ids(data.get_edge_ids., data.get_edge_ids.NodeT);
	graph.get_unchecked_link_edge_types(data.get_unchecked_link_edge_types.&self, data.get_unchecked_link_edge_types.NodeT, data.get_unchecked_link_edge_types.NodeT);
	graph.get_unchecked_link_weights(data.get_unchecked_link_weights., data.get_unchecked_link_weights.NodeT);
	graph.is_node_trap(data.is_node_trap.);
	graph.is_edge_trap(data.is_edge_trap.);
	graph.get_nodes_iter();
	graph.get_node_degrees_iter();
	graph.get_node_degrees_par_iter();
	graph.get_nodes_names_iter(data.get_nodes_names_iter.&self);
	graph.get_edges_iter(data.get_edges_iter.&self, data.get_edges_iter.bool);
	graph.get_sources_iter(data.get_sources_iter.);
	graph.get_sources_par_iter(data.get_sources_par_iter.);
	graph.get_destinations_iter(data.get_destinations_iter.);
	graph.get_destinations_par_iter(data.get_destinations_par_iter.&self, data.get_destinations_par_iter.bool);
	graph.get_edges_string_iter(data.get_edges_string_iter.&self, data.get_edges_string_iter.bool);
	graph.get_edges_par_iter(data.get_edges_par_iter.&self, data.get_edges_par_iter.bool);
	graph.get_edges_par_string_iter(data.get_edges_par_string_iter.&self, data.get_edges_par_string_iter.bool);
	graph.get_edges_triples(data.get_edges_triples.&self, data.get_edges_triples.bool);
	graph.get_edges_string_triples(data.get_edges_string_triples.&self, data.get_edges_string_triples.bool);
	graph.get_edges_par_string_triples(data.get_edges_par_string_triples.&self, data.get_edges_par_string_triples.bool);
	graph.get_edges_par_string_quadruples(data.get_edges_par_string_quadruples.&self, data.get_edges_par_string_quadruples.bool);
	graph.get_edges_string_quadruples(data.get_edges_string_quadruples.&self, data.get_edges_string_quadruples.bool);
	graph.get_edges_par_triples(data.get_edges_par_triples.&self, data.get_edges_par_triples.bool);
	graph.get_edges_quadruples(data.get_edges_quadruples.&self, data.get_edges_quadruples.bool);
	graph.get_edges_par_quadruples(data.get_edges_par_quadruples.&self, data.get_edges_par_quadruples.bool);
	graph.get_edge_quadruple(data.get_edge_quadruple.&self, data.get_edge_quadruple.EdgeT);
	graph.get_edge_triple(data.get_edge_triple.);
	graph.get_unique_edges_iter(data.get_unique_edges_iter.&self, data.get_unique_edges_iter.bool);
	graph.get_unique_sources_iter();
	graph.get_unique_sources_par_iter();


	graph.compute_hash();
	graph.sample_negatives(data.sample_negatives.&self, data.sample_negatives.EdgeT, data.sample_negatives.EdgeT, data.sample_negatives.Option<&Graph>, data.sample_negatives.bool, data.sample_negatives.bool)?;
	graph.connected_holdout(data.connected_holdout.&self, data.connected_holdout.EdgeT, data.connected_holdout.f64, data.connected_holdout.Option<Vec<Option<String>>>, data.connected_holdout.bool, data.connected_holdout.bool)?;
	graph.random_holdout(data.random_holdout.&self, data.random_holdout.EdgeT, data.random_holdout.f64, data.random_holdout.bool, data.random_holdout.Option<Vec<Option<String>>>, data.random_holdout.Option<EdgeT>, data.random_holdout.bool)?;
	graph.node_label_holdout(data.node_label_holdout.&self, data.node_label_holdout.f64, data.node_label_holdout.bool, data.node_label_holdout.EdgeT)?;
	graph.edge_label_holdout(data.edge_label_holdout.&self, data.edge_label_holdout.f64, data.edge_label_holdout.bool, data.edge_label_holdout.EdgeT)?;
	graph.random_subgraph(data.random_subgraph.&self, data.random_subgraph.usize, data.random_subgraph.NodeT, data.random_subgraph.bool)?;
	graph.kfold(data.kfold.&self, data.kfold.EdgeT, data.kfold.u64, data.kfold.Option<Vec<Option<String>>>, data.kfold.EdgeT, data.kfold.bool)?;

    Ok(())
}
