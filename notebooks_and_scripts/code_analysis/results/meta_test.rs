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
pub struct MetaParams {
	pub get_unchecked_weight_from_edge_id: Get_unchecked_weight_from_edge_id_Params,
	pub get_unchecked_edge_degreee_from_node_ids: Get_unchecked_edge_degreee_from_node_ids_Params,
	pub is_singleton_from_node_id: Is_singleton_from_node_id_Params,
	pub is_singleton_with_self_loops_from_node_id: Is_singleton_with_self_loops_from_node_id_Params,
	pub has_edge_from_node_ids: Has_edge_from_node_ids_Params,
	pub is_node_trap_from_node_id: Is_node_trap_from_node_id_Params,
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
	pub validate_node_id: Validate_node_id_Params,
	pub validate_edge_id: Validate_edge_id_Params,
	pub degrees_product: Degrees_product_Params,
	pub jaccard_index: Jaccard_index_Params,
	pub adamic_adar_index: Adamic_adar_index_Params,
	pub resource_allocation_index: Resource_allocation_index_Params,
	pub connected_components_number: Connected_components_number_Params,
	pub textual_report: Textual_report_Params,
	pub set_name: Set_name_Params,
	pub set_all_edge_types: Set_all_edge_types_Params,
	pub set_all_node_types: Set_all_node_types_Params,
	pub extract_uniform_node: Extract_uniform_node_Params,
	pub uniform_walk: Uniform_walk_Params,
	pub encode_edge: Encode_edge_Params,
	pub decode_edge: Decode_edge_Params,
	pub get_sources: Get_sources_Params,
	pub get_source_names: Get_source_names_Params,
	pub get_destinations: Get_destinations_Params,
	pub get_destination_names: Get_destination_names_Params,
	pub get_edges: Get_edges_Params,
	pub get_edge_names: Get_edge_names_Params,
	pub get_node_components_vector: Get_node_components_vector_Params,
	pub iter_node_neighbours: Iter_node_neighbours_Params,
	pub iter_sources_ids: Iter_sources_ids_Params,
	pub par_iter_sources_ids: Par_iter_sources_ids_Params,
	pub iter_destinations_ids: Iter_destinations_ids_Params,
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
    
	let _ = graph.iter_destinations_ids(data.iter_destinations_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_node_degrees().collect::<Vec<_>>();
	let _ = graph.iter_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_node_neighbours(data.iter_node_neighbours.src).collect::<Vec<_>>();
	let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_sources_ids(data.iter_sources_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_unique_sources().collect::<Vec<_>>();
	let _ = graph.par_iter_node_degrees().collect::<Vec<_>>();
	let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
	let _ = graph.par_iter_sources_ids(data.par_iter_sources_ids.directed).collect::<Vec<_>>();
	graph.compute_hash();
	graph.connected_components_number(data.connected_components_number.verbose);
	graph.decode_edge(data.decode_edge.edge);
	graph.disable_all();
	graph.encode_edge(data.encode_edge.src, data.encode_edge.dst);
	graph.extract_uniform_node(data.extract_uniform_node.node, data.extract_uniform_node.random_state);
	graph.get_dense_node_mapping();
	graph.get_destination_names(data.get_destination_names.directed);
	graph.get_destinations(data.get_destinations.directed);
	graph.get_directed_edges_number();
	graph.get_edge_names(data.get_edge_names.directed);
	graph.get_edge_type_names();
	graph.get_edge_types_number();
	graph.get_edges(data.get_edges.directed);
	graph.get_edges_number();
	graph.get_max_encodable_edge_number();
	graph.get_minimum_edge_types_number();
	graph.get_minimum_node_types_number();
	graph.get_multigraph_edges_number();
	graph.get_name();
	graph.get_node_components_vector(data.get_node_components_vector.verbose);
	graph.get_node_degrees();
	graph.get_node_names();
	graph.get_node_type_names();
	graph.get_node_types_number();
	graph.get_nodes();
	graph.get_nodes_mapping();
	graph.get_nodes_number();
	graph.get_not_singleton_nodes_number();
	graph.get_not_singletons();
	graph.get_outbounds();
	graph.get_self_loop_number();
	graph.get_singleton_nodes_number();
	graph.get_singleton_nodes_with_self_loops_number();
	graph.get_source_names(data.get_source_names.directed);
	graph.get_sources(data.get_sources.directed);
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
	graph.get_top_k_central_nodes_ids(data.get_top_k_central_nodes_ids.k);
	graph.get_trap_nodes_number();
	graph.get_unchecked_destination_node_id_from_edge_id(data.get_unchecked_destination_node_id_from_edge_id.edge_id);
	graph.get_unchecked_edge_degreee_from_node_ids(data.get_unchecked_edge_degreee_from_node_ids.src, data.get_unchecked_edge_degreee_from_node_ids.dst);
	graph.get_unchecked_edge_id_from_node_ids(data.get_unchecked_edge_id_from_node_ids.src, data.get_unchecked_edge_id_from_node_ids.dst);
	graph.get_unchecked_edge_type_id_from_edge_id(data.get_unchecked_edge_type_id_from_edge_id.edge_id);
	graph.get_unchecked_minmax_edge_ids_from_source_node_id(data.get_unchecked_minmax_edge_ids_from_source_node_id.src);
	graph.get_unchecked_node_degree_from_node_id(data.get_unchecked_node_degree_from_node_id.node_id);
	graph.get_unchecked_node_ids_from_edge_id(data.get_unchecked_node_ids_from_edge_id.edge_id);
	graph.get_unchecked_node_name_from_node_id(data.get_unchecked_node_name_from_node_id.node_id);
	graph.get_unchecked_unique_source_node_id(data.get_unchecked_unique_source_node_id.source_id);
	graph.get_unchecked_weight_from_edge_id(data.get_unchecked_weight_from_edge_id.edge_id);
	graph.get_undirected_edges_number();
	graph.get_unique_directed_edges_number();
	graph.get_unique_edges_number();
	graph.get_unique_self_loop_number();
	graph.get_unique_source_nodes_number();
	graph.get_unique_undirected_edges_number();
	graph.get_unknown_edge_types_number();
	graph.get_unknown_node_types_number();
	graph.has_edge_from_node_ids(data.has_edge_from_node_ids.src, data.has_edge_from_node_ids.dst);
	graph.has_edge_types();
	graph.has_edges();
	graph.has_multilabel_node_types();
	graph.has_node_types();
	graph.has_nodes();
	graph.has_selfloops();
	graph.has_singleton_nodes_with_self_loops();
	graph.has_singletons();
	graph.has_trap_nodes();
	graph.has_unknown_edge_types();
	graph.has_unknown_node_types();
	graph.has_weights();
	graph.invalidate_report();
	graph.is_directed();
	graph.is_multigraph();
	graph.is_singleton_with_self_loops_from_node_id(data.is_singleton_with_self_loops_from_node_id.node_id);
	graph.report();
	graph.set_all_edge_types(data.set_all_edge_types.edge_type);
	graph.set_all_node_types(data.set_all_node_types.node_type);
	graph.set_name(data.set_name.name);
	graph.strongly_connected_components();
	graph.traps_rate();
	graph.uniform_walk(data.uniform_walk.node, data.uniform_walk.random_state, data.uniform_walk.walk_length);
	let _ = graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two);
	let _ = graph.degrees_product(data.degrees_product.one, data.degrees_product.two);
	let _ = graph.get_density();
	let _ = graph.get_destination_node_id_from_edge_id(data.get_destination_node_id_from_edge_id.edge_id);
	let _ = graph.get_edge_id_from_node_ids(data.get_edge_id_from_node_ids.src, data.get_edge_id_from_node_ids.dst);
	let _ = graph.get_edge_type_counts();
	let _ = graph.get_edge_type_counts_hashmap();
	let _ = graph.get_edge_types();
	let _ = graph.get_max_node_degree();
	let _ = graph.get_max_weight();
	let _ = graph.get_min_node_degree();
	let _ = graph.get_min_weight();
	let _ = graph.get_minmax_edge_ids_from_source_node_id(data.get_minmax_edge_ids_from_source_node_id.src);
	let _ = graph.get_node_degree_from_node_id(data.get_node_degree_from_node_id.node_id);
	let _ = graph.get_node_degrees_mean();
	let _ = graph.get_node_degrees_median();
	let _ = graph.get_node_degrees_mode();
	let _ = graph.get_node_ids_from_edge_id(data.get_node_ids_from_edge_id.edge_id);
	let _ = graph.get_node_name_from_node_id(data.get_node_name_from_node_id.node_id);
	let _ = graph.get_node_neighbours_from_node_id(data.get_node_neighbours_from_node_id.node_id);
	let _ = graph.get_node_type_counts();
	let _ = graph.get_node_type_counts_hashmap();
	let _ = graph.get_node_types();
	let _ = graph.get_self_loop_rate();
	let _ = graph.get_weight_from_edge_id(data.get_weight_from_edge_id.edge_id);
	let _ = graph.get_weight_from_node_ids(data.get_weight_from_node_ids.src, data.get_weight_from_node_ids.dst);
	let _ = graph.get_weights();
	let _ = graph.is_node_trap_from_node_id(data.is_node_trap_from_node_id.node_id);
	let _ = graph.is_singleton_from_node_id(data.is_singleton_from_node_id.node_id);
	let _ = graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two);
	let _ = graph.must_have_edge_types();
	let _ = graph.must_have_node_types();
	let _ = graph.must_have_weights();
	let _ = graph.resource_allocation_index(data.resource_allocation_index.one, data.resource_allocation_index.two);
	let _ = graph.textual_report(data.textual_report.verbose);
	let _ = graph.validate_edge_id(data.validate_edge_id.edge_id);
	let _ = graph.validate_node_id(data.validate_node_id.node_id);

    Ok(())
}