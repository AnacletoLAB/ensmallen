use crate::{Graph, NodeT, NodeTypeT};
use std::collections::HashMap;

impl heterogeneous_graphlets::prelude::Graph for Graph {
    type Node = NodeT;
    type NeighbourIter<'a> = impl Iterator<Item = usize> + 'a;

    fn get_number_of_nodes(&self) -> usize {
        self.get_number_of_nodes() as usize
    }

    fn get_number_of_edges(&self) -> usize {
        self.get_number_of_directed_edges() as usize
    }

    fn iter_neighbours(&self, node: usize) -> Self::NeighbourIter<'_> {
        unsafe {
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(node as NodeT)
                .map(|node| node as usize)
        }
    }
}

impl heterogeneous_graphlets::prelude::TypedGraph for Graph {
    type NodeLabel = NodeTypeT;

    fn get_number_of_node_labels(&self) -> Self::NodeLabel {
        self.get_number_of_node_types().unwrap()
    }

    fn get_number_of_node_labels_usize(&self) -> usize {
        self.get_number_of_node_labels() as usize
    }

    fn get_node_label_from_usize(&self, label_index: usize) -> Self::NodeLabel {
        label_index as NodeTypeT
    }

    fn get_node_label_index(&self, label: Self::NodeLabel) -> usize {
        label as usize
    }

    fn get_node_label(&self, node: usize) -> Self::NodeLabel {
        unsafe {
            self.get_unchecked_node_type_ids_from_node_id(node as NodeT)
                .unwrap()[0]
        }
    }
}

impl heterogeneous_graphlets::prelude::HeterogeneousGraphlets<u16, u32> for Graph {
    type GraphLetCounter = HashMap<u16, u32>;
}
