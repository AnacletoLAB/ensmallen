
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
struct Spanning_arborescence_kruskal_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Spanning_arborescence_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Connected_components_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Validate_weight_Params {
	weight : WeightT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Parse_weight_Params {
	weight : String,
}


#[derive(Arbitrary, Debug, Clone)]
struct Set_all_edge_types_Params {
	edge_type : S,
}


#[derive(Arbitrary, Debug, Clone)]
struct Set_all_node_types_Params {
	node_type : S,
}


#[derive(Arbitrary, Debug, Clone)]
struct Enable_Params {
	vector_sources : bool,
	vector_destinations : bool,
	vector_outbounds : bool,
	cache_size : Option<f64>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Filter_Params {
	node_names : Option<Vec<String>>,
	node_types : Option<Vec<Option<String>>>,
	edge_types : Option<Vec<Option<String>>>,
	min_weight : Option<WeightT>,
	max_weight : Option<WeightT>,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_filtered_neighbours_range_Params {
	src : NodeT,
	node_names : Option<Vec<String>>,
	node_types : Option<Vec<Option<String>>>,
	edge_types : Option<Vec<Option<String>>>,
	min_weight : Option<WeightT>,
	max_weight : Option<WeightT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Degrees_product_Params {
	one : NodeT,
	two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Jaccard_index_Params {
	one : NodeT,
	two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Adamic_adar_index_Params {
	one : NodeT,
	two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Resource_allocation_index_Params {
	one : NodeT,
	two : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Traps_rate_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Textual_report_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Remove_Params {
	allow_nodes_set : Option<HashSet<String>>,
	deny_nodes_set : Option<HashSet<String>>,
	allow_node_types_set : Option<HashSet<String>>,
	deny_node_types_set : Option<HashSet<String>>,
	allow_edge_set : Option<HashSet<EdgeT>>,
	deny_edge_set : Option<HashSet<EdgeT>>,
	allow_edge_types_set : Option<HashSet<String>>,
	deny_edge_types_set : Option<HashSet<String>>,
	weights : bool,
	node_types : bool,
	edge_types : bool,
	singletons : bool,
	selfloops : bool,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Remove_components_Params {
	node_names : Option<Vec<String>>,
	node_types : Option<Vec<Option<String>>>,
	edge_types : Option<Vec<Option<String>>>,
	minimum_component_size : Option<NodeT>,
	top_k_components : Option<NodeT>,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Extract_uniform_node_Params {
	node : NodeT,
	random_state : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_bipartite_edges_Params {
	removed_existing_edges : Option<bool>,
	first_nodes_set : Option<HashSet<String>>,
	second_nodes_set : Option<HashSet<String>>,
	first_node_types_set : Option<HashSet<String>>,
	second_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_bipartite_edge_names_Params {
	removed_existing_edges : Option<bool>,
	first_nodes_set : Option<HashSet<String>>,
	second_nodes_set : Option<HashSet<String>>,
	first_node_types_set : Option<HashSet<String>>,
	second_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_star_edges_Params {
	central_node : String,
	removed_existing_edges : Option<bool>,
	star_points_nodes_set : Option<HashSet<String>>,
	star_points_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_star_edge_names_Params {
	central_node : String,
	removed_existing_edges : Option<bool>,
	star_points_nodes_set : Option<HashSet<String>>,
	star_points_node_types_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_clique_edges_Params {
	directed : Option<bool>,
	allow_self_loops : Option<bool>,
	removed_existing_edges : Option<bool>,
	allow_node_type_set : Option<HashSet<String>>,
	allow_node_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_clique_edge_names_Params {
	directed : Option<bool>,
	allow_self_loops : Option<bool>,
	removed_existing_edges : Option<bool>,
	allow_node_type_set : Option<HashSet<String>>,
	allow_node_set : Option<HashSet<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_name_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_singleton_with_self_loops_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_singletons_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_source_names_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destination_names_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_names_Params {
	k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_top_k_central_node_names_Params {
	k : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_types_Params {
	edge_type_id : EdgeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_names_Params {
	node_type_id : Vec<NodeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_node_type_id_Params {
	node_type_id : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_weights_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_names_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_type_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_weight_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_node_type_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_id_by_node_id_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_type_name_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_name_by_edge_id_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_name_Params {
	node_id : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_weight_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_node_types_Params {
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_directed_edges_number_Params {
	edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_count_by_edge_type_Params {
	edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_node_count_by_node_type_Params {
	node_type : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_count_by_node_type_Params {
	node_type : NodeTypeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_multigraph_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_range_Params {
	min_edge_id : EdgeT,
	max_edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_neighbours_iter_Params {
	src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unique_sources_number_Params {
	src : NodeT,
	dst : NodeT,
	edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_id_Params {
	src : NodeT,
	dst : NodeT,
	edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Has_edge_Params {
	src : NodeT,
	dst : NodeT,
	edge_type : Option<EdgeTypeT>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_type_counts_Params {
	edge_types : Vec<Option<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Translate_node_types_Params {
	node_types : Vec<Option<String>>,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_node_degree_Params {
	node : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_edge_ids_range_Params {
	src : NodeT,
	dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_destinations_range_Params {
	src : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_ids_Params {
	src : NodeT,
	dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_link_edge_types_Params {
	src : NodeT,
	dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unchecked_link_weights_Params {
	src : NodeT,
	dst : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_node_trap_Params {
	node : NodeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Is_edge_trap_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_sources_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_sources_par_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_destinations_par_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_triples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_triples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_triples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_string_quadruples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_string_quadruples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_triples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_quadruples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edges_par_quadruples_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_quadruple_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_edge_triple_Params {
	edge_id : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Get_unique_edges_iter_Params {
	directed : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Connected_holdout_Params {
	random_state : EdgeT,
	train_size : f64,
	edge_types : Option<Vec<Option<String>>>,
	include_all_edge_types : bool,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_holdout_Params {
	random_state : EdgeT,
	train_size : f64,
	include_all_edge_types : bool,
	edge_types : Option<Vec<Option<String>>>,
	min_number_overlaps : Option<EdgeT>,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Node_label_holdout_Params {
	train_size : f64,
	use_stratification : bool,
	random_state : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Edge_label_holdout_Params {
	train_size : f64,
	use_stratification : bool,
	random_state : EdgeT,
}


#[derive(Arbitrary, Debug, Clone)]
struct Random_subgraph_Params {
	random_state : usize,
	nodes_number : NodeT,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct Kfold_Params {
	k : EdgeT,
	k_index : u64,
	edge_types : Option<Vec<Option<String>>>,
	random_state : EdgeT,
	verbose : bool,
}


#[derive(Arbitrary, Debug, Clone)]
struct TheUltimateFuzzer {
	spanning_arborescence_kruskal : Spanning_arborescence_kruskal_Params,
	spanning_arborescence : Spanning_arborescence_Params,
	connected_components : Connected_components_Params,
	validate_weight : Validate_weight_Params,
	parse_weight : Parse_weight_Params,
	set_all_edge_types : Set_all_edge_types_Params,
	set_all_node_types : Set_all_node_types_Params,
	enable : Enable_Params,
	filter : Filter_Params,
	get_filtered_neighbours_range : Get_filtered_neighbours_range_Params,
	degrees_product : Degrees_product_Params,
	jaccard_index : Jaccard_index_Params,
	adamic_adar_index : Adamic_adar_index_Params,
	resource_allocation_index : Resource_allocation_index_Params,
	traps_rate : Traps_rate_Params,
	textual_report : Textual_report_Params,
	remove : Remove_Params,
	remove_components : Remove_components_Params,
	extract_uniform_node : Extract_uniform_node_Params,
	get_bipartite_edges : Get_bipartite_edges_Params,
	get_bipartite_edge_names : Get_bipartite_edge_names_Params,
	get_star_edges : Get_star_edges_Params,
	get_star_edge_names : Get_star_edge_names_Params,
	get_clique_edges : Get_clique_edges_Params,
	get_clique_edge_names : Get_clique_edge_names_Params,
	get_name : Get_name_Params,
	is_singleton_with_self_loops : Is_singleton_with_self_loops_Params,
	has_singletons : Has_singletons_Params,
	get_source_names : Get_source_names_Params,
	get_destinations : Get_destinations_Params,
	get_destination_names : Get_destination_names_Params,
	get_node_names : Get_node_names_Params,
	get_top_k_central_node_names : Get_top_k_central_node_names_Params,
	get_edge_types : Get_edge_types_Params,
	get_edge_type_names : Get_edge_type_names_Params,
	translate_node_type_id : Translate_node_type_id_Params,
	get_weights : Get_weights_Params,
	get_edge_names : Get_edge_names_Params,
	get_unchecked_edge_type : Get_unchecked_edge_type_Params,
	get_unchecked_edge_weight : Get_unchecked_edge_weight_Params,
	get_unchecked_node_type : Get_unchecked_node_type_Params,
	get_node_type_id_by_node_id : Get_node_type_id_by_node_id_Params,
	get_edge_type : Get_edge_type_Params,
	get_node_type_name : Get_node_type_name_Params,
	get_edge_type_name_by_edge_id : Get_edge_type_name_by_edge_id_Params,
	get_node_name : Get_node_name_Params,
	get_edge_weight : Get_edge_weight_Params,
	has_node_types : Has_node_types_Params,
	get_directed_edges_number : Get_directed_edges_number_Params,
	get_edge_count_by_edge_type : Get_edge_count_by_edge_type_Params,
	get_unchecked_node_count_by_node_type : Get_unchecked_node_count_by_node_type_Params,
	get_node_count_by_node_type : Get_node_count_by_node_type_Params,
	is_multigraph : Is_multigraph_Params,
	get_destinations_range : Get_destinations_range_Params,
	get_neighbours_iter : Get_neighbours_iter_Params,
	get_unique_sources_number : Get_unique_sources_number_Params,
	get_edge_id : Get_edge_id_Params,
	has_edge : Has_edge_Params,
	get_edge_type_counts : Get_edge_type_counts_Params,
	translate_node_types : Translate_node_types_Params,
	get_node_degree : Get_node_degree_Params,
	get_unchecked_edge_ids_range : Get_unchecked_edge_ids_range_Params,
	get_unchecked_destinations_range : Get_unchecked_destinations_range_Params,
	get_edge_ids : Get_edge_ids_Params,
	get_unchecked_link_edge_types : Get_unchecked_link_edge_types_Params,
	get_unchecked_link_weights : Get_unchecked_link_weights_Params,
	is_node_trap : Is_node_trap_Params,
	is_edge_trap : Is_edge_trap_Params,
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
    
    


	graph.spanning_arborescence_kruskal(data.spanning_arborescence_kruskal.verbose);
	graph.spanning_arborescence(data.spanning_arborescence.verbose)?;
	graph.connected_components(data.connected_components.verbose)?;






	graph.validate_weight(data.validate_weight.weight)?;
	graph.parse_weight(data.parse_weight.weight)?;


	graph.set_all_edge_types(data.set_all_edge_types.edge_type)?;
	graph.set_all_node_types(data.set_all_node_types.node_type);
	graph.enable(data.enable.vector_sources, data.enable.vector_destinations, data.enable.vector_outbounds, data.enable.cache_size)?;
	graph.strongly_connected_components();
	graph.filter(data.filter.node_names, data.filter.node_types, data.filter.edge_types, data.filter.min_weight, data.filter.max_weight, data.filter.verbose)?;
	graph.get_filtered_neighbours_range(data.get_filtered_neighbours_range.src, data.get_filtered_neighbours_range.node_names, data.get_filtered_neighbours_range.node_types, data.get_filtered_neighbours_range.edge_types, data.get_filtered_neighbours_range.min_weight, data.get_filtered_neighbours_range.max_weight)?;
	graph.degrees_product(data.degrees_product.one, data.degrees_product.two)?;
	graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two)?;
	graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two)?;
	graph.resource_allocation_index(data.resource_allocation_index.one, data.resource_allocation_index.two)?;
	graph.traps_rate(data.traps_rate.verbose);


	graph.textual_report(data.textual_report.verbose)?;
	graph.remove(data.remove.allow_nodes_set, data.remove.deny_nodes_set, data.remove.allow_node_types_set, data.remove.deny_node_types_set, data.remove.allow_edge_set, data.remove.deny_edge_set, data.remove.allow_edge_types_set, data.remove.deny_edge_types_set, data.remove.weights, data.remove.node_types, data.remove.edge_types, data.remove.singletons, data.remove.selfloops, data.remove.verbose)?;
	graph.remove_components(data.remove_components.node_names, data.remove_components.node_types, data.remove_components.edge_types, data.remove_components.minimum_component_size, data.remove_components.top_k_components, data.remove_components.verbose)?;
	graph.extract_uniform_node(data.extract_uniform_node.node, data.extract_uniform_node.random_state);






	graph.get_bipartite_edges(data.get_bipartite_edges.removed_existing_edges, data.get_bipartite_edges.first_nodes_set, data.get_bipartite_edges.second_nodes_set, data.get_bipartite_edges.first_node_types_set, data.get_bipartite_edges.second_node_types_set)?;
	graph.get_bipartite_edge_names(data.get_bipartite_edge_names.removed_existing_edges, data.get_bipartite_edge_names.first_nodes_set, data.get_bipartite_edge_names.second_nodes_set, data.get_bipartite_edge_names.first_node_types_set, data.get_bipartite_edge_names.second_node_types_set)?;
	graph.get_star_edges(data.get_star_edges.central_node, data.get_star_edges.removed_existing_edges, data.get_star_edges.star_points_nodes_set, data.get_star_edges.star_points_node_types_set)?;
	graph.get_star_edge_names(data.get_star_edge_names.central_node, data.get_star_edge_names.removed_existing_edges, data.get_star_edge_names.star_points_nodes_set, data.get_star_edge_names.star_points_node_types_set)?;
	graph.get_clique_edges(data.get_clique_edges.directed, data.get_clique_edges.allow_self_loops, data.get_clique_edges.removed_existing_edges, data.get_clique_edges.allow_node_type_set, data.get_clique_edges.allow_node_set);
	graph.get_clique_edge_names(data.get_clique_edge_names.directed, data.get_clique_edge_names.allow_self_loops, data.get_clique_edge_names.removed_existing_edges, data.get_clique_edge_names.allow_node_type_set, data.get_clique_edge_names.allow_node_set);
	graph.get_name(data.get_name.node_id);
	graph.is_singleton_with_self_loops(data.is_singleton_with_self_loops.node_id);

	graph.has_singletons(data.has_singletons.directed);
	graph.get_source_names(data.get_source_names.directed);
	graph.get_destinations(data.get_destinations.directed);
	graph.get_destination_names(data.get_destination_names.directed);
	graph.get_node_names(data.get_node_names.k);
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
	graph.get_edge_types(data.get_edge_types.edge_type_id);
	graph.get_edge_type_names(data.get_edge_type_names.node_type_id);
	graph.translate_node_type_id(data.translate_node_type_id.node_type_id);
	graph.get_weights(data.get_weights.directed);
	graph.get_edge_names(data.get_edge_names.directed);
	graph.get_unchecked_edge_type(data.get_unchecked_edge_type.edge_id);
	graph.get_unchecked_edge_weight(data.get_unchecked_edge_weight.edge_id);
	graph.get_unchecked_node_type(data.get_unchecked_node_type.node_id);
	graph.get_node_type_id_by_node_id(data.get_node_type_id_by_node_id.node_id)?;
	graph.get_edge_type(data.get_edge_type.edge_id)?;
	graph.get_node_type_name(data.get_node_type_name.node_id);
	graph.get_edge_type_name_by_edge_id(data.get_edge_type_name_by_edge_id.edge_id);
	graph.get_node_name(data.get_node_name.node_id)?;






	graph.get_edge_weight(data.get_edge_weight.edge_id);
	graph.has_node_types(data.has_node_types.verbose);
	graph.get_directed_edges_number(data.get_directed_edges_number.edge_type);
	graph.get_edge_count_by_edge_type(data.get_edge_count_by_edge_type.edge_type)?;


	graph.get_unchecked_node_count_by_node_type(data.get_unchecked_node_count_by_node_type.node_type);

	graph.get_node_count_by_node_type(data.get_node_count_by_node_type.node_type)?;

	graph.is_multigraph(data.is_multigraph.edge_id);
	graph.get_destinations_range(data.get_destinations_range.min_edge_id, data.get_destinations_range.max_edge_id);
	graph.get_neighbours_iter(data.get_neighbours_iter.src);
	graph.get_unique_sources_number(data.get_unique_sources_number.src, data.get_unique_sources_number.dst, data.get_unique_sources_number.edge_type);
	graph.get_edge_id(data.get_edge_id.src, data.get_edge_id.dst, data.get_edge_id.edge_type)?;
	graph.has_edge(data.has_edge.src, data.has_edge.dst, data.has_edge.edge_type);



	graph.get_edge_type_counts(data.get_edge_type_counts.edge_types)?;
	graph.translate_node_types(data.translate_node_types.node_types)?;


	graph.get_node_degree(data.get_node_degree.node);
	graph.get_unchecked_edge_ids_range(data.get_unchecked_edge_ids_range.src, data.get_unchecked_edge_ids_range.dst);
	graph.get_unchecked_destinations_range(data.get_unchecked_destinations_range.src);
	graph.get_edge_ids(data.get_edge_ids.src, data.get_edge_ids.dst);
	graph.get_unchecked_link_edge_types(data.get_unchecked_link_edge_types.src, data.get_unchecked_link_edge_types.dst);
	graph.get_unchecked_link_weights(data.get_unchecked_link_weights.src, data.get_unchecked_link_weights.dst);
	graph.is_node_trap(data.is_node_trap.node);
	graph.is_edge_trap(data.is_edge_trap.edge_id);
	graph.get_nodes_iter();
	graph.get_edges_iter(data.get_edges_iter.directed);
	graph.get_sources_iter(data.get_sources_iter.directed);
	graph.get_sources_par_iter(data.get_sources_par_iter.directed);
	graph.get_destinations_iter(data.get_destinations_iter.directed);
	graph.get_destinations_par_iter(data.get_destinations_par_iter.directed);
	graph.get_edges_string_iter(data.get_edges_string_iter.directed);
	graph.get_edges_par_iter(data.get_edges_par_iter.directed);
	graph.get_edges_par_string_iter(data.get_edges_par_string_iter.directed);
	graph.get_edges_triples(data.get_edges_triples.directed);
	graph.get_edges_string_triples(data.get_edges_string_triples.directed);
	graph.get_edges_par_string_triples(data.get_edges_par_string_triples.directed);
	graph.get_edges_par_string_quadruples(data.get_edges_par_string_quadruples.directed);
	graph.get_edges_string_quadruples(data.get_edges_string_quadruples.directed);
	graph.get_edges_par_triples(data.get_edges_par_triples.directed);
	graph.get_edges_quadruples(data.get_edges_quadruples.directed);
	graph.get_edges_par_quadruples(data.get_edges_par_quadruples.directed);
	graph.get_edge_quadruple(data.get_edge_quadruple.edge_id);
	graph.get_edge_triple(data.get_edge_triple.edge_id);
	graph.get_unique_edges_iter(data.get_unique_edges_iter.directed);
	graph.get_unique_sources_iter();
	graph.get_unique_sources_par_iter();







	graph.connected_holdout(data.connected_holdout.random_state, data.connected_holdout.train_size, data.connected_holdout.edge_types, data.connected_holdout.include_all_edge_types, data.connected_holdout.verbose)?;
	graph.random_holdout(data.random_holdout.random_state, data.random_holdout.train_size, data.random_holdout.include_all_edge_types, data.random_holdout.edge_types, data.random_holdout.min_number_overlaps, data.random_holdout.verbose)?;
	graph.node_label_holdout(data.node_label_holdout.train_size, data.node_label_holdout.use_stratification, data.node_label_holdout.random_state)?;
	graph.edge_label_holdout(data.edge_label_holdout.train_size, data.edge_label_holdout.use_stratification, data.edge_label_holdout.random_state)?;
	graph.random_subgraph(data.random_subgraph.random_state, data.random_subgraph.nodes_number, data.random_subgraph.verbose)?;
	graph.kfold(data.kfold.k, data.kfold.k_index, data.kfold.edge_types, data.kfold.random_state, data.kfold.verbose)?;

    Ok(())
}
