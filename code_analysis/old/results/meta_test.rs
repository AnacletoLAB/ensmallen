use super::*;
use arbitrary::Arbitrary;
use std::collections::HashSet;
use rayon::iter::ParallelIterator;

#[derive(Arbitrary, Debug, Clone)]
pub struct Random_spanning_arborescence_kruskal_Params {
	pub random_state : EdgeT,
	pub unwanted_edge_types : Option<HashSet<Option<EdgeTypeT>>>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Spanning_arborescence_kruskal_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Spanning_arborescence_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_components_Params {
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_label_prediction_tuple_from_node_ids_Params {
	pub node_ids : Vec<NodeT>,
	pub random_state : u64,
	pub include_central_node : bool,
	pub offset : NodeT,
	pub max_neighbours : Option<NodeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_with_selfloops_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Has_edge_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Has_edge_with_type_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Is_trap_node_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Enable_Params {
	pub vector_sources : bool,
	pub vector_destinations : bool,
	pub vector_outbounds : bool,
	pub cache_size : Option<f64>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_ids_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_id_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_ids_and_type_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_ids_type_and_weight_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_nodes_ids_Params {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct get_unweighted_node_degree_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_node_names_Params {
	pub k : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_id_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_id_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_name_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_name_from_edge_id_Params {
	pub edge_id : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_name_from_edge_type_id_Params {
	pub edge_type_id : EdgeTypeT,
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
pub struct Get_weight_with_type_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_name_from_node_id_Params {
	pub node_id : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_count_from_edge_type_id_Params {
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_count_from_node_type_id_Params {
	pub node_type : Option<NodeTypeT>,
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
pub struct Get_minmax_edge_ids_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_id_with_type_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
	pub edge_type : Option<EdgeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_ids_from_edge_type_names_Params {
	pub edge_types : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_ids_from_node_type_names_Params {
	pub node_types : Vec<Option<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_minmax_edge_ids_from_source_node_id_Params {
	pub src : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_name_from_node_type_id_Params {
	pub node_type_id : NodeTypeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_names_from_node_type_ids_Params {
	pub node_type_ids : Vec<NodeTypeT>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Filter_Params {
	pub node_names : Option<Vec<String>>,
	pub node_types : Option<Vec<Option<String>>>,
	pub edge_types : Option<Vec<Option<String>>>,
	pub min_weight : Option<WeightT>,
	pub max_weight : Option<WeightT>,
	pub verbose : bool,
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
pub struct Remove_Params {
	pub allow_nodes_set : Option<HashSet<String>>,
	pub deny_nodes_set : Option<HashSet<String>>,
	pub allow_node_types_set : Option<HashSet<String>>,
	pub deny_node_types_set : Option<HashSet<String>>,
	pub allow_edge_set : Option<HashSet<EdgeT>>,
	pub deny_edge_set : Option<HashSet<EdgeT>>,
	pub allow_edge_types_set : Option<HashSet<String>>,
	pub deny_edge_types_set : Option<HashSet<String>>,
	pub weights : bool,
	pub node_types : bool,
	pub edge_types : bool,
	pub singletons : bool,
	pub selfloops : bool,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Remove_components_Params {
	pub node_names : Option<Vec<String>>,
	pub node_types : Option<Vec<Option<String>>>,
	pub edge_types : Option<Vec<Option<String>>>,
	pub minimum_component_size : Option<NodeT>,
	pub top_k_components : Option<NodeT>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_name_Params {
	pub name : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_all_edge_types_Params {
	pub edge_type : String,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Set_all_node_types_Params {
	pub node_type : String,
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
pub struct Get_bipartite_edges_Params {
	pub removed_existing_edges : Option<bool>,
	pub first_nodes_set : Option<HashSet<String>>,
	pub second_nodes_set : Option<HashSet<String>>,
	pub first_node_types_set : Option<HashSet<String>>,
	pub second_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_bipartite_edge_names_Params {
	pub removed_existing_edges : Option<bool>,
	pub first_nodes_set : Option<HashSet<String>>,
	pub second_nodes_set : Option<HashSet<String>>,
	pub first_node_types_set : Option<HashSet<String>>,
	pub second_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_star_edges_Params {
	pub central_node : String,
	pub removed_existing_edges : Option<bool>,
	pub star_points_nodes_set : Option<HashSet<String>>,
	pub star_points_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_star_edge_names_Params {
	pub central_node : String,
	pub removed_existing_edges : Option<bool>,
	pub star_points_nodes_set : Option<HashSet<String>>,
	pub star_points_node_types_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_clique_edges_Params {
	pub directed : Option<bool>,
	pub allow_selfloops : Option<bool>,
	pub removed_existing_edges : Option<bool>,
	pub allow_node_type_set : Option<HashSet<String>>,
	pub allow_node_set : Option<HashSet<String>>,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Get_clique_edge_names_Params {
	pub directed : Option<bool>,
	pub allow_selfloops : Option<bool>,
	pub removed_existing_edges : Option<bool>,
	pub allow_node_type_set : Option<HashSet<String>>,
	pub allow_node_set : Option<HashSet<String>>,
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
pub struct Par_iter_destinations_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edge_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edges_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edge_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edges_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edges_with_type_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edges_with_type_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edge_with_type_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edge_with_type_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edge_with_type_and_weight_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edge_with_type_and_weight_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Par_iter_edge_with_type_and_weight_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edge_with_type_and_weight_ids_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_unique_edges_Params {
	pub directed : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Iter_edge_ids_from_node_ids_Params {
	pub src : NodeT,
	pub dst : NodeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_holdout_Params {
	pub random_state : EdgeT,
	pub train_size : f64,
	pub edge_types : Option<Vec<Option<String>>>,
	pub include_all_edge_types : bool,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Random_holdout_Params {
	pub random_state : EdgeT,
	pub train_size : f64,
	pub include_all_edge_types : bool,
	pub edge_types : Option<Vec<Option<String>>>,
	pub min_number_overlaps : Option<EdgeT>,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Node_label_holdout_Params {
	pub train_size : f64,
	pub use_stratification : bool,
	pub random_state : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Edge_label_holdout_Params {
	pub train_size : f64,
	pub use_stratification : bool,
	pub random_state : EdgeT,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Random_subgraph_Params {
	pub random_state : usize,
	pub nodes_number : NodeT,
	pub verbose : bool,
}
#[derive(Arbitrary, Debug, Clone)]
pub struct Kfold_Params {
	pub k : EdgeT,
	pub k_index : u64,
	pub edge_types : Option<Vec<Option<String>>>,
	pub random_state : EdgeT,
	pub verbose : bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {
	pub random_spanning_arborescence_kruskal: Random_spanning_arborescence_kruskal_Params,
	pub spanning_arborescence_kruskal: Spanning_arborescence_kruskal_Params,
	pub spanning_arborescence: Spanning_arborescence_Params,
	pub connected_components: Connected_components_Params,
	pub get_node_label_prediction_tuple_from_node_ids: Get_node_label_prediction_tuple_from_node_ids_Params,
	pub is_singleton_from_node_id: Is_singleton_from_node_id_Params,
	pub is_singleton_with_selfloops_from_node_id: Is_singleton_with_selfloops_from_node_id_Params,
	pub has_edge_from_node_ids: Has_edge_from_node_ids_Params,
	pub has_edge_with_type_from_node_ids: Has_edge_with_type_from_node_ids_Params,
	pub is_trap_node_from_node_id: Is_trap_node_from_node_id_Params,
	pub enable: Enable_Params,
	pub get_node_ids_from_edge_id: Get_node_ids_from_edge_id_Params,
	pub get_edge_id_from_node_ids: Get_edge_id_from_node_ids_Params,
	pub get_node_ids_and_type_from_edge_id: Get_node_ids_and_type_from_edge_id_Params,
	pub get_node_ids_type_and_weight_from_edge_id: Get_node_ids_type_and_weight_from_edge_id_Params,
	pub get_top_k_central_nodes_ids: Get_top_k_central_nodes_ids_Params,
	pub get_unweighted_node_degree_from_node_id: get_unweighted_node_degree_from_node_id_Params,
	pub get_top_k_central_node_names: Get_top_k_central_node_names_Params,
	pub get_node_type_id_from_node_id: Get_node_type_id_from_node_id_Params,
	pub get_edge_type_id_from_edge_id: Get_edge_type_id_from_edge_id_Params,
	pub get_node_type_name_from_node_id: Get_node_type_name_from_node_id_Params,
	pub get_edge_type_name_from_edge_id: Get_edge_type_name_from_edge_id_Params,
	pub get_edge_type_name_from_edge_type_id: Get_edge_type_name_from_edge_type_id_Params,
	pub get_weight_from_edge_id: Get_weight_from_edge_id_Params,
	pub get_weight_from_node_ids: Get_weight_from_node_ids_Params,
	pub get_weight_with_type_from_node_ids: Get_weight_with_type_from_node_ids_Params,
	pub get_node_name_from_node_id: Get_node_name_from_node_id_Params,
	pub get_edge_count_from_edge_type_id: Get_edge_count_from_edge_type_id_Params,
	pub get_node_count_from_node_type_id: Get_node_count_from_node_type_id_Params,
	pub get_destination_node_id_from_edge_id: Get_destination_node_id_from_edge_id_Params,
	pub get_node_neighbours_from_node_id: Get_node_neighbours_from_node_id_Params,
	pub get_minmax_edge_ids_from_node_ids: Get_minmax_edge_ids_from_node_ids_Params,
	pub get_edge_id_with_type_from_node_ids: Get_edge_id_with_type_from_node_ids_Params,
	pub get_edge_type_ids_from_edge_type_names: Get_edge_type_ids_from_edge_type_names_Params,
	pub get_node_type_ids_from_node_type_names: Get_node_type_ids_from_node_type_names_Params,
	pub get_minmax_edge_ids_from_source_node_id: Get_minmax_edge_ids_from_source_node_id_Params,
	pub get_node_type_name_from_node_type_id: Get_node_type_name_from_node_type_id_Params,
	pub get_node_type_names_from_node_type_ids: Get_node_type_names_from_node_type_ids_Params,
	pub filter: Filter_Params,
	pub validate_node_id: Validate_node_id_Params,
	pub validate_edge_id: Validate_edge_id_Params,
	pub degrees_product: Degrees_product_Params,
	pub jaccard_index: Jaccard_index_Params,
	pub adamic_adar_index: Adamic_adar_index_Params,
	pub resource_allocation_index: Resource_allocation_index_Params,
	pub connected_components_number: Connected_components_number_Params,
	pub textual_report: Textual_report_Params,
	pub remove: Remove_Params,
	pub remove_components: Remove_components_Params,
	pub set_name: Set_name_Params,
	pub set_all_edge_types: Set_all_edge_types_Params,
	pub set_all_node_types: Set_all_node_types_Params,
	pub encode_edge: Encode_edge_Params,
	pub decode_edge: Decode_edge_Params,
	pub get_bipartite_edges: Get_bipartite_edges_Params,
	pub get_bipartite_edge_names: Get_bipartite_edge_names_Params,
	pub get_star_edges: Get_star_edges_Params,
	pub get_star_edge_names: Get_star_edge_names_Params,
	pub get_clique_edges: Get_clique_edges_Params,
	pub get_clique_edge_names: Get_clique_edge_names_Params,
	pub get_sources: Get_sources_Params,
	pub get_source_names: Get_source_names_Params,
	pub get_destinations: Get_destinations_Params,
	pub get_destination_names: Get_destination_names_Params,
	pub get_edges: Get_edges_Params,
	pub get_edge_names: Get_edge_names_Params,
	pub get_node_components_vector: Get_node_components_vector_Params,
	pub iter_sources_ids: Iter_sources_ids_Params,
	pub par_iter_sources_ids: Par_iter_sources_ids_Params,
	pub iter_destinations_ids: Iter_destinations_ids_Params,
	pub par_iter_destinations_ids: Par_iter_destinations_ids_Params,
	pub iter_edge_ids: Iter_edge_ids_Params,
	pub iter_edges: Iter_edges_Params,
	pub par_iter_edge_ids: Par_iter_edge_ids_Params,
	pub par_iter_edges: Par_iter_edges_Params,
	pub iter_edges_with_type_ids: Iter_edges_with_type_ids_Params,
	pub iter_edges_with_type: Iter_edges_with_type_Params,
	pub par_iter_edge_with_type: Par_iter_edge_with_type_Params,
	pub par_iter_edge_with_type_ids: Par_iter_edge_with_type_ids_Params,
	pub par_iter_edge_with_type_and_weight: Par_iter_edge_with_type_and_weight_Params,
	pub iter_edge_with_type_and_weight: Iter_edge_with_type_and_weight_Params,
	pub par_iter_edge_with_type_and_weight_ids: Par_iter_edge_with_type_and_weight_ids_Params,
	pub iter_edge_with_type_and_weight_ids: Iter_edge_with_type_and_weight_ids_Params,
	pub iter_unique_edges: Iter_unique_edges_Params,
	pub iter_edge_ids_from_node_ids: Iter_edge_ids_from_node_ids_Params,
	pub connected_holdout: Connected_holdout_Params,
	pub random_holdout: Random_holdout_Params,
	pub node_label_holdout: Node_label_holdout_Params,
	pub edge_label_holdout: Edge_label_holdout_Params,
	pub random_subgraph: Random_subgraph_Params,
	pub kfold: Kfold_Params,
    pub from_vec: FromVecHarnessParams,
}

pub fn meta_test(data: MetaParams) -> Result<(), String> {
    let data_copy = data.clone();
    let data_copy2 = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test(Some(info), data_copy.clone(), None);
    }));

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        "MetaTest",
        data.from_vec.ignore_duplicated_nodes,
        false,
        data.from_vec.ignore_duplicated_edges,
        false,
        data.from_vec.verbose,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_edge_weights,
        true,
        true,
        true,
    )?;
    
    

    let g_copy = graph.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test_once_loaded(Some(info), data_copy2.clone(), g_copy.clone());
    }));
    
	let _ = graph.iter_destinations_ids(data.iter_destinations_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_ids(data.iter_edge_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_with_type_and_weight(data.iter_edge_with_type_and_weight.directed).collect::<Vec<_>>();
	let _ = graph.iter_edge_with_type_and_weight_ids(data.iter_edge_with_type_and_weight_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_edges(data.iter_edges.directed).collect::<Vec<_>>();
	let _ = graph.iter_edges_with_type(data.iter_edges_with_type.directed).collect::<Vec<_>>();
	let _ = graph.iter_edges_with_type_ids(data.iter_edges_with_type_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_unweighted_node_degrees().collect::<Vec<_>>();
	let _ = graph.iter_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_node_names_and_node_type_names().collect::<Vec<_>>();
	let _ = graph.iter_nodes_with_type_ids().collect::<Vec<_>>();
	let _ = graph.iter_connected_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_singleton_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_singleton_with_selfloops_node_ids().collect::<Vec<_>>();
	let _ = graph.iter_sources_ids(data.iter_sources_ids.directed).collect::<Vec<_>>();
	let _ = graph.iter_unique_edges(data.iter_unique_edges.directed).collect::<Vec<_>>();
	let _ = graph.iter_unique_sources().collect::<Vec<_>>();
	let _ = graph.par_iter_destinations_ids(data.par_iter_destinations_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_ids(data.par_iter_edge_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_with_type(data.par_iter_edge_with_type.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_with_type_and_weight(data.par_iter_edge_with_type_and_weight.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_with_type_and_weight_ids(data.par_iter_edge_with_type_and_weight_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edge_with_type_ids(data.par_iter_edge_with_type_ids.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_edges(data.par_iter_edges.directed).collect::<Vec<_>>();
	let _ = graph.par_iter_unweighted_node_degrees().collect::<Vec<_>>();
	let _ = graph.par_iter_node_ids().collect::<Vec<_>>();
	let _ = graph.par_iter_sources_ids(data.par_iter_sources_ids.directed).collect::<Vec<_>>();
	graph.compute_hash();
	graph.connected_components_number(data.connected_components_number.verbose);
	graph.decode_edge(data.decode_edge.edge);
	graph.disable_all();
	graph.encode_edge(data.encode_edge.src, data.encode_edge.dst);
	graph.get_clique_edge_names(data.get_clique_edge_names.directed, data.get_clique_edge_names.allow_selfloops, data.get_clique_edge_names.removed_existing_edges, data.get_clique_edge_names.allow_node_type_set, data.get_clique_edge_names.allow_node_set);
	graph.get_clique_edges(data.get_clique_edges.directed, data.get_clique_edges.allow_selfloops, data.get_clique_edges.removed_existing_edges, data.get_clique_edges.allow_node_type_set, data.get_clique_edges.allow_node_set);
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
	graph.get_unweighted_node_degrees();
	graph.get_node_names();
	graph.get_node_types_number();
	graph.get_nodes();
	graph.get_nodes_mapping();
	graph.get_nodes_number();
	graph.get_not_singleton_nodes_number();
	graph.get_not_singleton_nodes();
	graph.get_outbounds();
	graph.get_selfloop_number();
	graph.get_singleton_nodes_number();
	graph.get_singleton_nodes_with_selfloops_number();
	graph.get_source_names(data.get_source_names.directed);
	graph.get_sources(data.get_sources.directed);
	graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
	graph.get_top_k_central_nodes_ids(data.get_top_k_central_nodes_ids.k);
	graph.get_trap_nodes_number();
	graph.get_undirected_edges_number();
	graph.get_unique_directed_edges_number();
	graph.get_unique_edges_number();
	graph.get_unique_selfloop_number();
	graph.get_unique_source_nodes_number();
	graph.get_unique_undirected_edges_number();
	graph.get_unknown_edge_types_number();
	graph.get_unknown_node_types_number();
	graph.has_edge_from_node_ids(data.has_edge_from_node_ids.src, data.has_edge_from_node_ids.dst);
	graph.has_edge_types();
	graph.has_edge_with_type_from_node_ids(data.has_edge_with_type_from_node_ids.src, data.has_edge_with_type_from_node_ids.dst, data.has_edge_with_type_from_node_ids.edge_type);
	graph.has_edges();
	graph.has_multilabel_node_types();
	graph.has_node_types();
	graph.has_nodes();
	graph.has_selfloops();
	graph.has_singleton_nodes_with_selfloops();
	graph.has_singleton_nodes();
	graph.has_trap_nodes();
	graph.has_unknown_edge_types();
	graph.has_unknown_node_types();
	graph.has_edge_weights();
	graph.is_directed();
	graph.is_multigraph();
	graph.is_singleton_with_selfloops_from_node_id(data.is_singleton_with_selfloops_from_node_id.node_id);
	graph.random_spanning_arborescence_kruskal(data.random_spanning_arborescence_kruskal.random_state, &data.random_spanning_arborescence_kruskal.unwanted_edge_types, data.random_spanning_arborescence_kruskal.verbose);
	graph.report();
	graph.set_name(data.set_name.name);
	graph.spanning_arborescence_kruskal(data.spanning_arborescence_kruskal.verbose);
	graph.strongly_connected_components();
	graph.get_traps_rate();
	let _ = graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two);
	let _ = graph.connected_components(data.connected_components.verbose);
	let _ = graph.connected_holdout(data.connected_holdout.random_state, data.connected_holdout.train_size, data.connected_holdout.edge_types, data.connected_holdout.include_all_edge_types, data.connected_holdout.verbose);
	let _ = graph.degrees_product(data.degrees_product.one, data.degrees_product.two);
	let _ = graph.edge_label_holdout(data.edge_label_holdout.train_size, data.edge_label_holdout.use_stratification, data.edge_label_holdout.random_state);
	let _ = graph.enable(data.enable.vector_sources, data.enable.vector_destinations, data.enable.vector_outbounds, data.enable.cache_size);
	let _ = graph.filter(data.filter.node_names, data.filter.node_types, data.filter.edge_types, data.filter.min_weight, data.filter.max_weight, data.filter.verbose);
	let _ = graph.get_bipartite_edge_names(data.get_bipartite_edge_names.removed_existing_edges, data.get_bipartite_edge_names.first_nodes_set, data.get_bipartite_edge_names.second_nodes_set, data.get_bipartite_edge_names.first_node_types_set, data.get_bipartite_edge_names.second_node_types_set);
	let _ = graph.get_bipartite_edges(data.get_bipartite_edges.removed_existing_edges, data.get_bipartite_edges.first_nodes_set, data.get_bipartite_edges.second_nodes_set, data.get_bipartite_edges.first_node_types_set, data.get_bipartite_edges.second_node_types_set);
	let _ = graph.get_density();
	let _ = graph.get_destination_node_id_from_edge_id(data.get_destination_node_id_from_edge_id.edge_id);
	let _ = graph.get_edge_count_from_edge_type_id(data.get_edge_count_from_edge_type_id.edge_type);
	let _ = graph.get_edge_id_from_node_ids(data.get_edge_id_from_node_ids.src, data.get_edge_id_from_node_ids.dst);
	let _ = graph.get_edge_id_with_type_from_node_ids(data.get_edge_id_with_type_from_node_ids.src, data.get_edge_id_with_type_from_node_ids.dst, data.get_edge_id_with_type_from_node_ids.edge_type);
	let _ = graph.get_edge_type_counts();
	let _ = graph.get_edge_type_id_counts_hashmap();
	let _ = graph.get_edge_type_id_from_edge_id(data.get_edge_type_id_from_edge_id.edge_id);
	let _ = graph.get_edge_type_ids_from_edge_type_names(data.get_edge_type_ids_from_edge_type_names.edge_types);
	let _ = graph.get_edge_type_name_from_edge_id(data.get_edge_type_name_from_edge_id.edge_id);
	let _ = graph.get_edge_type_name_from_edge_type_id(data.get_edge_type_name_from_edge_type_id.edge_type_id);
	let _ = graph.get_edge_types();
	let _ = graph.get_unweighted_max_node_degree();
	let _ = graph.get_max_weight();
	let _ = graph.get_min_node_degree();
	let _ = graph.get_min_weight();
	let _ = graph.get_minmax_edge_ids_from_node_ids(data.get_minmax_edge_ids_from_node_ids.src, data.get_minmax_edge_ids_from_node_ids.dst);
	let _ = graph.get_minmax_edge_ids_from_source_node_id(data.get_minmax_edge_ids_from_source_node_id.src);
	let _ = graph.get_node_count_from_node_type_id(data.get_node_count_from_node_type_id.node_type);
	let _ = graph.get_unweighted_node_degree_from_node_id(data.get_unweighted_node_degree_from_node_id.node_id);
	let _ = graph.get_unweighted_node_degrees_mean();
	let _ = graph.get_unweighted_node_degrees_median();
	let _ = graph.get_unweighted_node_degrees_mode();
	let _ = graph.get_node_ids_and_type_from_edge_id(data.get_node_ids_and_type_from_edge_id.edge_id);
	let _ = graph.get_node_ids_from_edge_id(data.get_node_ids_from_edge_id.edge_id);
	let _ = graph.get_node_ids_type_and_weight_from_edge_id(data.get_node_ids_type_and_weight_from_edge_id.edge_id);
	let _ = graph.get_node_label_prediction_tuple_from_node_ids(data.get_node_label_prediction_tuple_from_node_ids.node_ids, data.get_node_label_prediction_tuple_from_node_ids.random_state, data.get_node_label_prediction_tuple_from_node_ids.include_central_node, data.get_node_label_prediction_tuple_from_node_ids.offset, data.get_node_label_prediction_tuple_from_node_ids.max_neighbours);
	let _ = graph.get_node_name_from_node_id(data.get_node_name_from_node_id.node_id);
	let _ = graph.get_node_neighbours_from_node_id(data.get_node_neighbours_from_node_id.node_id);
	let _ = graph.get_node_type_counts();
	let _ = graph.get_node_type_counts_hashmap();
	let _ = graph.get_node_type_id_from_node_id(data.get_node_type_id_from_node_id.node_id);
	let _ = graph.get_node_type_ids_from_node_type_names(data.get_node_type_ids_from_node_type_names.node_types);
	let _ = graph.get_node_type_name_from_node_id(data.get_node_type_name_from_node_id.node_id);
	let _ = graph.get_node_type_name_from_node_type_id(data.get_node_type_name_from_node_type_id.node_type_id);
	let _ = graph.get_node_type_names();
	let _ = graph.get_node_type_names_from_node_type_ids(data.get_node_type_names_from_node_type_ids.node_type_ids);
	let _ = graph.get_node_types_ids();
	let _ = graph.get_selfloop_rate();
	let _ = graph.get_star_edge_names(data.get_star_edge_names.central_node, data.get_star_edge_names.removed_existing_edges, data.get_star_edge_names.star_points_nodes_set, data.get_star_edge_names.star_points_node_types_set);
	let _ = graph.get_star_edges(data.get_star_edges.central_node, data.get_star_edges.removed_existing_edges, data.get_star_edges.star_points_nodes_set, data.get_star_edges.star_points_node_types_set);
	let _ = graph.get_weight_from_edge_id(data.get_weight_from_edge_id.edge_id);
	let _ = graph.get_weight_from_node_ids(data.get_weight_from_node_ids.src, data.get_weight_from_node_ids.dst);
	let _ = graph.get_weight_with_type_from_node_ids(data.get_weight_with_type_from_node_ids.src, data.get_weight_with_type_from_node_ids.dst, data.get_weight_with_type_from_node_ids.edge_type);
	let _ = graph.get_weights();
	let _ = graph.is_trap_node_from_node_id(data.is_trap_node_from_node_id.node_id);
	let _ = graph.is_singleton_from_node_id(data.is_singleton_from_node_id.node_id);
	let _ = graph.iter_edge_ids_from_node_ids(data.iter_edge_ids_from_node_ids.src, data.iter_edge_ids_from_node_ids.dst);
	let _ = graph.iter_weights();
	let _ = graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two);
	let _ = graph.kfold(data.kfold.k, data.kfold.k_index, data.kfold.edge_types, data.kfold.random_state, data.kfold.verbose);
	let _ = graph.must_have_edge_types();
	let _ = graph.must_have_node_types();
	let _ = graph.must_have_edge_weights();
	let _ = graph.node_label_holdout(data.node_label_holdout.train_size, data.node_label_holdout.use_stratification, data.node_label_holdout.random_state);
	let _ = graph.par_iter_weights();
	let _ = graph.random_holdout(data.random_holdout.random_state, data.random_holdout.train_size, data.random_holdout.include_all_edge_types, data.random_holdout.edge_types, data.random_holdout.min_number_overlaps, data.random_holdout.verbose);
	let _ = graph.random_subgraph(data.random_subgraph.random_state, data.random_subgraph.nodes_number, data.random_subgraph.verbose);
	let _ = graph.remove(data.remove.allow_nodes_set, data.remove.deny_nodes_set, data.remove.allow_node_types_set, data.remove.deny_node_types_set, data.remove.allow_edge_set, data.remove.deny_edge_set, data.remove.allow_edge_types_set, data.remove.deny_edge_types_set, data.remove.weights, data.remove.node_types, data.remove.edge_types, data.remove.singletons, data.remove.selfloops, data.remove.verbose);
	let _ = graph.remove_components(data.remove_components.node_names, data.remove_components.node_types, data.remove_components.edge_types, data.remove_components.minimum_component_size, data.remove_components.top_k_components, data.remove_components.verbose);
	let _ = graph.resource_allocation_index(data.resource_allocation_index.one, data.resource_allocation_index.two);
	let _ = graph.spanning_arborescence(data.spanning_arborescence.verbose);
	let _ = graph.textual_report(data.textual_report.verbose);
	let _ = graph.validate_edge_id(data.validate_edge_id.edge_id);
	let _ = graph.validate_node_id(data.validate_node_id.node_id);
	let mut graph = graph.set_all_edge_types(data.set_all_edge_types.edge_type)?;
	let mut graph = graph.set_all_node_types(data.set_all_node_types.node_type)?;

    Ok(())
}