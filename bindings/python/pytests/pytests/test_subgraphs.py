from ensmallen.datasets.linqs import Cora
import pytest


def test_subgraph_sampling_node_labels():
    cora = Cora().drop_singleton_nodes()

    with pytest.raises(ValueError):
        # Should raise because the list of metrics is empty
        cora.get_subgraphs(
            number_of_nodes_to_sample=256,
            random_state=42,
            node_sampling_method="breadth_first_search",
            edge_weighting_methods=[]
        )

    with pytest.raises(ValueError):
        # Should raise because the provided node sampling method is invalid
        cora.get_subgraphs(
            number_of_nodes_to_sample=256,
            random_state=42,
            node_sampling_method="invalid_node_sampling",
            edge_weighting_methods=["jaccard_coefficient", "adamic_adar_index"]
        )

    with pytest.raises(ValueError):
        # Should raise because the one of the provided metrics is invalid
        cora.get_subgraphs(
            number_of_nodes_to_sample=256,
            random_state=42,
            node_sampling_method="breadth_first_search",
            edge_weighting_methods=["jaccard_coefficient", "invalid_metric"]
        )

    with pytest.raises(ValueError):
        # Should raise because the nodes to be sampled should be a strictly positive value
        cora.get_subgraphs(
            number_of_nodes_to_sample=0,
            random_state=42,
            node_sampling_method="breadth_first_search",
            edge_weighting_methods=["jaccard_coefficient", "invalid_metric"]
        )

    for node_sampling_method in cora.get_node_sampling_methods():
        for edge_weighting_method in cora.get_edge_weighting_methods():
            number_of_nodes_to_sample = 256
            nodes, kernels = cora.get_subgraphs(
                number_of_nodes_to_sample=number_of_nodes_to_sample,
                random_state=42,
                node_sampling_method=node_sampling_method,
                edge_weighting_methods=[edge_weighting_method]
            )
            assert len(nodes) <= number_of_nodes_to_sample
            assert len(kernels) == 1
            assert all(
                kernel.shape == (len(nodes), len(nodes))
                for kernel in kernels
            )

            source_nodes, source_kernels, destination_nodes, destination_kernels, labels = cora.get_edge_prediction_subgraphs(
                number_of_nodes_to_sample=number_of_nodes_to_sample,
                random_state=42,
                node_sampling_method=node_sampling_method,
                edge_weighting_methods=[edge_weighting_method]
            ) 
            assert set(source_nodes) == set(destination_nodes)
            assert len(source_nodes) == len(destination_nodes)
            assert len(labels) == len(source_nodes)
            assert all(
                kernel.shape == (len(nodes), len(nodes))
                for kernels in (source_kernels, destination_kernels)
                for kernel in kernels
            )