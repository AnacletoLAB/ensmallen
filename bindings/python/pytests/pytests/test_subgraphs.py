from ensmallen_graph.datasets.linqs import Cora
import pytest


def test_subgraph_sampling_node_labels():
    cora = Cora()

    with pytest.raises(ValueError):
        # Should raise because the list of metrics is empty
        cora.get_subgraphs(
            nodes_to_sample_number=256,
            random_state=42,
            node_sampling_method="breath_first_search",
            metrics=[]
        )

    with pytest.raises(ValueError):
        # Should raise because the provided node sampling method is invalid
        cora.get_subgraphs(
            nodes_to_sample_number=256,
            random_state=42,
            node_sampling_method="invalid_node_sampling",
            metrics=["jaccard_coefficient", "adamic_adar_index"]
        )

    with pytest.raises(ValueError):
        # Should raise because the one of the provided metrics is invalid
        cora.get_subgraphs(
            nodes_to_sample_number=256,
            random_state=42,
            node_sampling_method="breath_first_search",
            metrics=["jaccard_coefficient", "invalid_metric"]
        )

    with pytest.raises(ValueError):
        # Should raise because the nodes to be sampled should be a strictly positive value
        cora.get_subgraphs(
            nodes_to_sample_number=0,
            random_state=42,
            node_sampling_method="breath_first_search",
            metrics=["jaccard_coefficient", "invalid_metric"]
        )

    nodes_to_sample_number = 256
    metrics_to_compute = ["jaccard_coefficient", "adamic_adar_index"]
    nodes, kernels = cora.get_subgraphs(
        nodes_to_sample_number=nodes_to_sample_number,
        random_state=42,
        node_sampling_method="breath_first_search",
        metrics=metrics_to_compute
    )
    assert len(nodes) <= nodes_to_sample_number
    assert len(kernels) == len(metrics_to_compute)
    assert all(
        kernel.shape == (len(nodes), len(nodes))
        for kernel in kernels
    )
