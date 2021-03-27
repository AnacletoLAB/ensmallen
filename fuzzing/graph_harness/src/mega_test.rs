use super::*;
use arbitrary::Arbitrary;
use std::collections::HashSet;

#[derive(Arbitrary, Debug, Clone)]
pub struct Remove_Params {
    pub allow_nodes_set: Option<HashSet<String>>,
    pub deny_nodes_set: Option<HashSet<String>>,
    pub allow_node_types_set: Option<HashSet<String>>,
    pub deny_node_types_set: Option<HashSet<String>>,
    pub allow_edge_set: Option<HashSet<EdgeT>>,
    pub deny_edge_set: Option<HashSet<EdgeT>>,
    pub allow_edge_types_set: Option<HashSet<String>>,
    pub deny_edge_types_set: Option<HashSet<String>>,
    pub weights: bool,
    pub node_types: bool,
    pub edge_types: bool,
    pub singletons: bool,
    pub selfloops: bool,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Remove_components_Params {
    pub node_names: Option<Vec<String>>,
    pub node_types: Option<Vec<Option<String>>>,
    pub edge_types: Option<Vec<Option<String>>>,
    pub minimum_component_size: Option<NodeT>,
    pub top_k_components: Option<NodeT>,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_bipartite_edges_Params {
    pub removed_existing_edges: Option<bool>,
    pub first_nodes_set: Option<HashSet<String>>,
    pub second_nodes_set: Option<HashSet<String>>,
    pub first_node_types_set: Option<HashSet<String>>,
    pub second_node_types_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_bipartite_edge_names_Params {
    pub removed_existing_edges: Option<bool>,
    pub first_nodes_set: Option<HashSet<String>>,
    pub second_nodes_set: Option<HashSet<String>>,
    pub first_node_types_set: Option<HashSet<String>>,
    pub second_node_types_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_star_edges_Params {
    pub central_node: String,
    pub removed_existing_edges: Option<bool>,
    pub star_points_nodes_set: Option<HashSet<String>>,
    pub star_points_node_types_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_star_edge_names_Params {
    pub central_node: String,
    pub removed_existing_edges: Option<bool>,
    pub star_points_nodes_set: Option<HashSet<String>>,
    pub star_points_node_types_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_clique_edges_Params {
    pub directed: Option<bool>,
    pub allow_self_loops: Option<bool>,
    pub removed_existing_edges: Option<bool>,
    pub allow_node_type_set: Option<HashSet<String>>,
    pub allow_node_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_clique_edge_names_Params {
    pub directed: Option<bool>,
    pub allow_self_loops: Option<bool>,
    pub removed_existing_edges: Option<bool>,
    pub allow_node_type_set: Option<HashSet<String>>,
    pub allow_node_set: Option<HashSet<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_id_by_node_ids_Params {
    pub src: NodeT,
    pub dst: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_sources_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_sources_par_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destinations_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destinations_par_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_string_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_string_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_triples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_string_triples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_string_triples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_string_quadruples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_string_quadruples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_triples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_quadruples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_par_quadruples_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_quadruple_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_triple_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unique_edges_iter_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_holdout_Params {
    pub random_state: EdgeT,
    pub train_size: f64,
    pub edge_types: Option<Vec<Option<String>>>,
    pub include_all_edge_types: bool,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Random_holdout_Params {
    pub random_state: EdgeT,
    pub train_size: f64,
    pub include_all_edge_types: bool,
    pub edge_types: Option<Vec<Option<String>>>,
    pub min_number_overlaps: Option<EdgeT>,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Node_label_holdout_Params {
    pub train_size: f64,
    pub use_stratification: bool,
    pub random_state: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Edge_label_holdout_Params {
    pub train_size: f64,
    pub use_stratification: bool,
    pub random_state: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Random_subgraph_Params {
    pub random_state: usize,
    pub nodes_number: NodeT,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Kfold_Params {
    pub k: EdgeT,
    pub k_index: u64,
    pub edge_types: Option<Vec<Option<String>>>,
    pub random_state: EdgeT,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Is_singleton_with_self_loops_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_sources_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_source_names_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destinations_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destination_names_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_nodes_Params {
    pub k: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_top_k_central_node_names_Params {
    pub k: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_name_Params {
    pub edge_type_id: EdgeTypeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_node_type_id_Params {
    pub node_type_id: NodeTypeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_node_type_id_vector_Params {
    pub node_type_id: Vec<NodeTypeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edges_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_names_Params {
    pub directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_id_by_node_id_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_type_name_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_type_name_by_edge_id_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_name_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_weight_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_components_vector_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_count_by_edge_type_Params {
    pub edge_type: Option<EdgeTypeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_count_by_node_type_Params {
    pub node_type: Option<NodeTypeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_destination_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_id_with_type_by_node_ids_Params {
    pub src: NodeT,
    pub dst: NodeT,
    pub edge_type: Option<EdgeTypeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Has_edge_with_type_Params {
    pub src: NodeT,
    pub dst: NodeT,
    pub edge_type: Option<EdgeTypeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_edge_types_Params {
    pub edge_types: Vec<Option<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Translate_node_types_Params {
    pub node_types: Vec<Option<String>>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_degree_Params {
    pub node_id: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_edge_ids_Params {
    pub src: NodeT,
    pub dst: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_link_edge_types_Params {
    pub src: NodeT,
    pub dst: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_unchecked_link_weights_Params {
    pub src: NodeT,
    pub dst: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Is_node_trap_Params {
    pub node: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Is_edge_trap_Params {
    pub edge_id: EdgeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Degrees_product_Params {
    pub one: NodeT,
    pub two: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Jaccard_index_Params {
    pub one: NodeT,
    pub two: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Adamic_adar_index_Params {
    pub one: NodeT,
    pub two: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Resource_allocation_index_Params {
    pub one: NodeT,
    pub two: NodeT,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_components_number_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Textual_report_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Filter_Params {
    pub node_names: Option<Vec<String>>,
    pub node_types: Option<Vec<Option<String>>>,
    pub edge_types: Option<Vec<Option<String>>>,
    pub min_weight: Option<WeightT>,
    pub max_weight: Option<WeightT>,
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Spanning_arborescence_kruskal_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Spanning_arborescence_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Connected_components_Params {
    pub verbose: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Enable_Params {
    pub vector_sources: bool,
    pub vector_destinations: bool,
    pub vector_outbounds: bool,
    pub cache_size: Option<f64>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct Get_node_label_prediction_tuple_by_node_ids_Params {
    pub node_ids: Vec<NodeT>,
    pub random_state: u64,
    pub include_central_node: bool,
    pub offset: NodeT,
    pub max_neighbours: Option<NodeT>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct TheUltimateFuzzer {
    pub remove: Remove_Params,
    pub remove_components: Remove_components_Params,
    pub get_bipartite_edges: Get_bipartite_edges_Params,
    pub get_bipartite_edge_names: Get_bipartite_edge_names_Params,
    pub get_star_edges: Get_star_edges_Params,
    pub get_star_edge_names: Get_star_edge_names_Params,
    pub get_clique_edges: Get_clique_edges_Params,
    pub get_clique_edge_names: Get_clique_edge_names_Params,
    pub get_edge_id_by_node_ids: Get_edge_id_by_node_ids_Params,
    pub get_edges_iter: Get_edges_iter_Params,
    pub get_sources_iter: Get_sources_iter_Params,
    pub get_sources_par_iter: Get_sources_par_iter_Params,
    pub get_destinations_iter: Get_destinations_iter_Params,
    pub get_destinations_par_iter: Get_destinations_par_iter_Params,
    pub get_edges_string_iter: Get_edges_string_iter_Params,
    pub get_edges_par_iter: Get_edges_par_iter_Params,
    pub get_edges_par_string_iter: Get_edges_par_string_iter_Params,
    pub get_edges_triples: Get_edges_triples_Params,
    pub get_edges_string_triples: Get_edges_string_triples_Params,
    pub get_edges_par_string_triples: Get_edges_par_string_triples_Params,
    pub get_edges_par_string_quadruples: Get_edges_par_string_quadruples_Params,
    pub get_edges_string_quadruples: Get_edges_string_quadruples_Params,
    pub get_edges_par_triples: Get_edges_par_triples_Params,
    pub get_edges_quadruples: Get_edges_quadruples_Params,
    pub get_edges_par_quadruples: Get_edges_par_quadruples_Params,
    pub get_edge_quadruple: Get_edge_quadruple_Params,
    pub get_edge_triple: Get_edge_triple_Params,
    pub get_unique_edges_iter: Get_unique_edges_iter_Params,
    pub connected_holdout: Connected_holdout_Params,
    pub random_holdout: Random_holdout_Params,
    pub node_label_holdout: Node_label_holdout_Params,
    pub edge_label_holdout: Edge_label_holdout_Params,
    pub random_subgraph: Random_subgraph_Params,
    pub kfold: Kfold_Params,
    pub is_singleton: Is_singleton_Params,
    pub is_singleton_with_self_loops: Is_singleton_with_self_loops_Params,
    pub get_sources: Get_sources_Params,
    pub get_source_names: Get_source_names_Params,
    pub get_destinations: Get_destinations_Params,
    pub get_destination_names: Get_destination_names_Params,
    pub get_top_k_central_nodes: Get_top_k_central_nodes_Params,
    pub get_top_k_central_node_names: Get_top_k_central_node_names_Params,
    pub get_edge_type_name: Get_edge_type_name_Params,
    pub translate_node_type_id: Translate_node_type_id_Params,
    pub translate_node_type_id_vector: Translate_node_type_id_vector_Params,
    pub get_edges: Get_edges_Params,
    pub get_edge_names: Get_edge_names_Params,
    pub get_node_type_id_by_node_id: Get_node_type_id_by_node_id_Params,
    pub get_edge_type: Get_edge_type_Params,
    pub get_node_type_name: Get_node_type_name_Params,
    pub get_edge_type_name_by_edge_id: Get_edge_type_name_by_edge_id_Params,
    pub get_node_name: Get_node_name_Params,
    pub get_edge_weight: Get_edge_weight_Params,
    pub get_node_components_vector: Get_node_components_vector_Params,
    pub get_edge_count_by_edge_type: Get_edge_count_by_edge_type_Params,
    pub get_node_count_by_node_type: Get_node_count_by_node_type_Params,
    pub get_destination: Get_destination_Params,
    pub get_edge_id_with_type_by_node_ids: Get_edge_id_with_type_by_node_ids_Params,
    pub has_edge_with_type: Has_edge_with_type_Params,
    pub translate_edge_types: Translate_edge_types_Params,
    pub translate_node_types: Translate_node_types_Params,
    pub get_node_degree: Get_node_degree_Params,
    pub get_edge_ids: Get_edge_ids_Params,
    pub get_unchecked_link_edge_types: Get_unchecked_link_edge_types_Params,
    pub get_unchecked_link_weights: Get_unchecked_link_weights_Params,
    pub is_node_trap: Is_node_trap_Params,
    pub is_edge_trap: Is_edge_trap_Params,
    pub degrees_product: Degrees_product_Params,
    pub jaccard_index: Jaccard_index_Params,
    pub adamic_adar_index: Adamic_adar_index_Params,
    pub resource_allocation_index: Resource_allocation_index_Params,
    pub connected_components_number: Connected_components_number_Params,
    pub textual_report: Textual_report_Params,
    pub filter: Filter_Params,
    pub spanning_arborescence_kruskal: Spanning_arborescence_kruskal_Params,
    pub spanning_arborescence: Spanning_arborescence_Params,
    pub connected_components: Connected_components_Params,
    pub enable: Enable_Params,
    pub get_node_label_prediction_tuple_by_node_ids:
        Get_node_label_prediction_tuple_by_node_ids_Params,
    pub from_vec: FromVecHarnessParams,
}

pub fn mega_test(data: TheUltimateFuzzer) -> Result<(), String> {
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

    graph.remove(
        data.remove.allow_nodes_set,
        data.remove.deny_nodes_set,
        data.remove.allow_node_types_set,
        data.remove.deny_node_types_set,
        data.remove.allow_edge_set,
        data.remove.deny_edge_set,
        data.remove.allow_edge_types_set,
        data.remove.deny_edge_types_set,
        data.remove.weights,
        data.remove.node_types,
        data.remove.edge_types,
        data.remove.singletons,
        data.remove.selfloops,
        data.remove.verbose,
    )?;
    graph.remove_components(
        data.remove_components.node_names,
        data.remove_components.node_types,
        data.remove_components.edge_types,
        data.remove_components.minimum_component_size,
        data.remove_components.top_k_components,
        data.remove_components.verbose,
    )?;

    graph.get_bipartite_edges(
        data.get_bipartite_edges.removed_existing_edges,
        data.get_bipartite_edges.first_nodes_set,
        data.get_bipartite_edges.second_nodes_set,
        data.get_bipartite_edges.first_node_types_set,
        data.get_bipartite_edges.second_node_types_set,
    )?;
    graph.get_bipartite_edge_names(
        data.get_bipartite_edge_names.removed_existing_edges,
        data.get_bipartite_edge_names.first_nodes_set,
        data.get_bipartite_edge_names.second_nodes_set,
        data.get_bipartite_edge_names.first_node_types_set,
        data.get_bipartite_edge_names.second_node_types_set,
    )?;
    graph.get_star_edges(
        data.get_star_edges.central_node,
        data.get_star_edges.removed_existing_edges,
        data.get_star_edges.star_points_nodes_set,
        data.get_star_edges.star_points_node_types_set,
    )?;
    graph.get_star_edge_names(
        data.get_star_edge_names.central_node,
        data.get_star_edge_names.removed_existing_edges,
        data.get_star_edge_names.star_points_nodes_set,
        data.get_star_edge_names.star_points_node_types_set,
    )?;
    graph.get_clique_edges(
        data.get_clique_edges.directed,
        data.get_clique_edges.allow_self_loops,
        data.get_clique_edges.removed_existing_edges,
        data.get_clique_edges.allow_node_type_set,
        data.get_clique_edges.allow_node_set,
    );
    graph.get_clique_edge_names(
        data.get_clique_edge_names.directed,
        data.get_clique_edge_names.allow_self_loops,
        data.get_clique_edge_names.removed_existing_edges,
        data.get_clique_edge_names.allow_node_type_set,
        data.get_clique_edge_names.allow_node_set,
    );
    graph.get_edge_id_by_node_ids(
        data.get_edge_id_by_node_ids.src,
        data.get_edge_id_by_node_ids.dst,
    )?;
    graph.get_nodes_names_iter().for_each(drop);
    graph
        .get_edges_iter(data.get_edges_iter.directed)
        .for_each(drop);
    graph
        .get_sources_iter(data.get_sources_iter.directed)
        .for_each(drop);
    graph.get_sources_par_iter(data.get_sources_par_iter.directed);
    graph
        .get_destinations_iter(data.get_destinations_iter.directed)
        .for_each(drop);
    graph.get_destinations_par_iter(data.get_destinations_par_iter.directed);
    graph
        .get_edges_string_iter(data.get_edges_string_iter.directed)
        .for_each(drop);
    graph.get_edges_par_iter(data.get_edges_par_iter.directed);
    graph.get_edges_par_string_iter(data.get_edges_par_string_iter.directed);
    graph
        .get_edges_triples(data.get_edges_triples.directed)
        .for_each(drop);
    graph
        .get_edges_string_triples(data.get_edges_string_triples.directed)
        .for_each(drop);
    graph.get_edges_par_string_triples(data.get_edges_par_string_triples.directed);
    graph.get_edges_par_string_quadruples(data.get_edges_par_string_quadruples.directed);
    graph.get_edges_string_quadruples(data.get_edges_string_quadruples.directed);
    graph
        .get_edges_par_triples(data.get_edges_par_triples.directed)
        .for_each(drop);
    graph
        .get_edges_quadruples(data.get_edges_quadruples.directed)
        .for_each(drop);
    graph.get_edges_par_quadruples(data.get_edges_par_quadruples.directed);
    graph.get_edge_quadruple(data.get_edge_quadruple.edge_id);
    graph.get_edge_triple(data.get_edge_triple.edge_id);
    graph
        .get_unique_edges_iter(data.get_unique_edges_iter.directed)
        .for_each(drop);

    graph.connected_holdout(
        data.connected_holdout.random_state,
        data.connected_holdout.train_size,
        data.connected_holdout.edge_types,
        data.connected_holdout.include_all_edge_types,
        data.connected_holdout.verbose,
    )?;
    graph.random_holdout(
        data.random_holdout.random_state,
        data.random_holdout.train_size,
        data.random_holdout.include_all_edge_types,
        data.random_holdout.edge_types,
        data.random_holdout.min_number_overlaps,
        data.random_holdout.verbose,
    )?;
    graph.node_label_holdout(
        data.node_label_holdout.train_size,
        data.node_label_holdout.use_stratification,
        data.node_label_holdout.random_state,
    )?;
    graph.edge_label_holdout(
        data.edge_label_holdout.train_size,
        data.edge_label_holdout.use_stratification,
        data.edge_label_holdout.random_state,
    )?;
    graph.random_subgraph(
        data.random_subgraph.random_state,
        data.random_subgraph.nodes_number,
        data.random_subgraph.verbose,
    )?;
    graph.kfold(
        data.kfold.k,
        data.kfold.k_index,
        data.kfold.edge_types,
        data.kfold.random_state,
        data.kfold.verbose,
    )?;
    graph.is_singleton(data.is_singleton.node_id)?;
    graph.is_singleton_with_self_loops(data.is_singleton_with_self_loops.node_id);

    graph.get_sources(data.get_sources.directed);
    graph.get_source_names(data.get_source_names.directed);
    graph.get_destinations(data.get_destinations.directed);
    graph.get_destination_names(data.get_destination_names.directed);
    graph.get_top_k_central_nodes(data.get_top_k_central_nodes.k);
    graph.get_top_k_central_node_names(data.get_top_k_central_node_names.k);
    graph.get_edge_type_name(data.get_edge_type_name.edge_type_id)?;
    graph.translate_node_type_id(data.translate_node_type_id.node_type_id)?;
    graph.translate_node_type_id_vector(data.translate_node_type_id_vector.node_type_id)?;
    graph.get_edges(data.get_edges.directed);
    graph.get_edge_names(data.get_edge_names.directed);
    graph.get_node_type_id_by_node_id(data.get_node_type_id_by_node_id.node_id)?;
    graph.get_edge_type(data.get_edge_type.edge_id)?;
    graph.get_node_type_name(data.get_node_type_name.node_id)?;
    graph.get_edge_type_name_by_edge_id(data.get_edge_type_name_by_edge_id.edge_id);
    graph.get_node_name(data.get_node_name.node_id)?;

    graph.get_edge_weight(data.get_edge_weight.edge_id);
    graph.get_node_components_vector(data.get_node_components_vector.verbose);
    graph.get_edge_count_by_edge_type(data.get_edge_count_by_edge_type.edge_type)?;

    graph.get_node_count_by_node_type(data.get_node_count_by_node_type.node_type)?;

    graph.get_destination(data.get_destination.edge_id)?;
    graph.get_edge_id_with_type_by_node_ids(
        data.get_edge_id_with_type_by_node_ids.src,
        data.get_edge_id_with_type_by_node_ids.dst,
        data.get_edge_id_with_type_by_node_ids.edge_type,
    )?;

    graph.has_edge_with_type(
        data.has_edge_with_type.src,
        data.has_edge_with_type.dst,
        data.has_edge_with_type.edge_type,
    );

    graph.translate_edge_types(data.translate_edge_types.edge_types)?;
    graph.translate_node_types(data.translate_node_types.node_types)?;

    graph.get_node_degree(data.get_node_degree.node_id)?;
    graph.get_edge_ids(data.get_edge_ids.src, data.get_edge_ids.dst);
    graph.get_unchecked_link_edge_types(
        data.get_unchecked_link_edge_types.src,
        data.get_unchecked_link_edge_types.dst,
    );
    graph.get_unchecked_link_weights(
        data.get_unchecked_link_weights.src,
        data.get_unchecked_link_weights.dst,
    );
    graph.is_node_trap(data.is_node_trap.node)?;
    graph.is_edge_trap(data.is_edge_trap.edge_id)?;
    graph.degrees_product(data.degrees_product.one, data.degrees_product.two)?;
    graph.jaccard_index(data.jaccard_index.one, data.jaccard_index.two)?;
    graph.adamic_adar_index(data.adamic_adar_index.one, data.adamic_adar_index.two)?;
    graph.resource_allocation_index(
        data.resource_allocation_index.one,
        data.resource_allocation_index.two,
    )?;
    graph.connected_components_number(data.connected_components_number.verbose);

    graph.textual_report(data.textual_report.verbose)?;
    graph.filter(
        data.filter.node_names,
        data.filter.node_types,
        data.filter.edge_types,
        data.filter.min_weight,
        data.filter.max_weight,
        data.filter.verbose,
    )?;

    graph.spanning_arborescence_kruskal(data.spanning_arborescence_kruskal.verbose);
    graph.spanning_arborescence(data.spanning_arborescence.verbose)?;
    graph.connected_components(data.connected_components.verbose)?;
    graph.enable(
        data.enable.vector_sources,
        data.enable.vector_destinations,
        data.enable.vector_outbounds,
        data.enable.cache_size,
    )?;

    graph.get_node_label_prediction_tuple_by_node_ids(
        data.get_node_label_prediction_tuple_by_node_ids.node_ids,
        data.get_node_label_prediction_tuple_by_node_ids
            .random_state,
        data.get_node_label_prediction_tuple_by_node_ids
            .include_central_node,
        data.get_node_label_prediction_tuple_by_node_ids.offset,
        data.get_node_label_prediction_tuple_by_node_ids
            .max_neighbours,
    )?;

    Ok(())
}
