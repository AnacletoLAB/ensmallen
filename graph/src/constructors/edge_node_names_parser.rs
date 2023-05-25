use super::*;

impl_struct_func!(EdgeNodeNamesParser Vocabulary<NodeT>);

impl EdgeNodeNamesParser {
    pub fn parse_strings<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            line_number,
            (
                vocabulary.0.insert(src_name)?.0,
                vocabulary.0.insert(dst_name)?.0,
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn parse_strings_unchecked<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary = self.get_mutable_write();
        unsafe {
            Ok((
                line_number,
                (
                    vocabulary.0.unchecked_insert(src_name),
                    vocabulary.0.unchecked_insert(dst_name),
                    edge_type_name,
                    weight,
                ),
            ))
        }
    }

    pub fn get<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            line_number,
            (
                match vocabulary.get(&src_name) {
                    Some(src) => Ok(src),
                    None => Err(format!(
                        concat!(
                            "Found an unknown source node name while reading the edge list.\n",
                            "Specifically the unknown source node name is {:?}.\n",
                            "The edge in question is composed of ({:?}, {:?})."
                        ),
                        src_name, src_name, dst_name
                    )),
                }?,
                match vocabulary.get(&dst_name) {
                    Some(dst) => Ok(dst),
                    None => Err(format!(
                        concat!(
                            "Found an unknown destination node name while reading the edge list.\n",
                            "Specifically the unknown destination node name is {:?}.\n",
                            "The edge in question is composed of ({:?}, {:?})."
                        ),
                        dst_name, src_name, dst_name
                    )),
                }?,
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn get_unchecked<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            line_number,
            (
                vocabulary.get(&src_name).unwrap(),
                vocabulary.get(&dst_name).unwrap(),
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn to_numeric_with_insertion<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            line_number,
            (
                vocabulary.0.insert(src_name)?.0,
                vocabulary.0.insert(dst_name)?.0,
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn to_numeric_checked<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        let vocabulary_length = self.get_immutable().len() as NodeT;
        let mut numeric_source_node = 0;
        let mut numeric_destination_node = 0;
        for (node_name, node_column, node_id) in [
            (src_name, "source", &mut numeric_source_node),
            (dst_name, "destination", &mut numeric_destination_node),
        ] {
            *node_id = match node_name.parse::<NodeT>() {
                Ok(node_id) => {
                    if node_id >= vocabulary_length {
                        return Err(format!(
                            concat!(
                                "The provided {node_column} node '{node_name}' is being treated as the ",
                                "number {node_id} since you requested the numeric conversion ",
                                "of the nodes appearing within the edge list.\n",
                                "However, the current node vocabulary has length {vocabulary_length}.\n",
                                "It follows that the number of the node {node_id} is higher than the ",
                                "number of nodes in the current node vocabulary {vocabulary_length}.\n",
                                "One possible cause of this is that we are expecting a DENSE RANGE of node ids ",
                                "for the edge list, starting from zero and ending at {vocabulary_length}.\n",
                                "At this time, the minimum node ID in your vocabulary is {minimum_node_id}."
                            ),
                            node_column=node_column,
                            node_name=node_name,
                            node_id=node_id,
                            vocabulary_length=vocabulary_length,
                            minimum_node_id=match self.get_immutable().get_minimum_id() {
                                Some(minimum_node_id) => if minimum_node_id > 0 {
                                    format!(
                                        concat!(
                                            "equal to {minimum_node_id} (which is higher than zero).\n",
                                            "Therefore your node vocabulary is not compatible ",
                                            "with the DENSE numeric edge list loading. You likely ",
                                            "want to densify your node ids in the edge list by shifting ",
                                            "them by {minimum_node_id}, so to be able to densely load them."
                                        ),
                                        minimum_node_id=minimum_node_id
                                    )
                                } else {
                                    format!(
                                        concat!(
                                            "already equal to zero, making this error peculiar. ",
                                            "Please open an issue on the GRAPE GitHub repository."
                                        ),
                                    )
                                },
                                None => unreachable!(
                                    "The node vocabulary should have a minimum node ID at this point."
                                )
                            }
                        ));
                    } else {
                        node_id
                    }
                }
                Err(_) => {
                    return Err(format!(
                        "Unable to parse to integer the provided {} node {}.",
                        node_name, node_id
                    ))
                }
            };
        }
        Ok((
            line_number,
            (
                numeric_source_node,
                numeric_destination_node,
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn to_numeric_unchecked<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        Ok((
            line_number,
            (
                unsafe { atoi_c(src_name.as_str()) },
                unsafe { atoi_c(dst_name.as_str()) },
                edge_type_name,
                weight,
            ),
        ))
    }
}
