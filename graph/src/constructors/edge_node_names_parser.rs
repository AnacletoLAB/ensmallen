use super::*;

impl_struct_func!(EdgeNodeNamesParser Vocabulary<NodeT>);

impl EdgeNodeNamesParser {
    pub fn parse_strings<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            vocabulary.0.insert(src_name)?.0,
            vocabulary.0.insert(dst_name)?.0,
            edge_type_name,
            weight,
        ))
    }

    pub fn parse_strings_unchecked<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_mutable_write();
        unsafe {
            Ok((
                vocabulary.0.unchecked_insert(src_name),
                vocabulary.0.unchecked_insert(dst_name),
                edge_type_name,
                weight,
            ))
        }
    }

    pub fn get<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        Ok((
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
        ))
    }

    pub fn get_unchecked<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        unsafe {
            Ok((
                unsafe { vocabulary.get(&src_name).unwrap_unchecked() },
                unsafe { vocabulary.get(&dst_name).unwrap_unchecked() },
                edge_type_name,
                weight,
            ))
        }
    }

    pub fn to_numeric<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        let src_node_id = match src_name.parse::<NodeT>() {
            Ok(src) => Ok(src),
            Err(_) => Err(format!(
                concat!(
                    "The given source node name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                src_name
            )),
        }?;
        let dst_node_id = match dst_name.parse::<NodeT>() {
            Ok(dst) => Ok(dst),
            Err(_) => Err(format!(
                concat!(
                    "The given destination node name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                dst_name
            )),
        }?;
        if vocabulary.len() as NodeT <= src_node_id {
            return Err(format!(
                concat!(
                    "The given source node name {:?} ",
                    "has a value greater than the number ",
                    "of provided nodes {}."
                ),
                src_node_id,
                vocabulary.len()
            ));
        }
        if vocabulary.len() as NodeT <= dst_node_id {
            return Err(format!(
                concat!(
                    "The given destination node name {:?} ",
                    "has a value greater than the number ",
                    "of provided nodes {}."
                ),
                dst_node_id,
                vocabulary.len()
            ));
        }
        Ok((src_node_id, dst_node_id, edge_type_name, weight))
    }

    pub fn to_numeric_unchecked<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        unsafe {
            Ok((
                src_name.parse::<NodeT>().unwrap_unchecked(),
                dst_name.parse::<NodeT>().unwrap_unchecked(),
                edge_type_name,
                weight,
            ))
        }
    }
}
