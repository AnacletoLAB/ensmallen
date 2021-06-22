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
            vocabulary.insert_values(src_name)?,
            vocabulary.insert_values(dst_name)?,
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
                vocabulary.unchecked_insert_values(src_name)?,
                vocabulary.unchecked_insert_values(dst_name)?,
                edge_type_name,
                weight,
            ))
        }
    }

    pub fn translate<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            vocabulary.translate(src_name)?,
            vocabulary.translate(dst_name)?,
            edge_type_name,
            weight,
        ))
    }

    pub fn translate_unchecked<E, W>(
        &mut self,
        value: Result<(String, String, E, W)>,
    ) -> Result<(NodeT, NodeT, E, W)> {
        let (src_name, dst_name, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        unsafe {
            Ok((
                vocabulary.unchecked_translate(src_name)?,
                vocabulary.unchecked_translate(dst_name)?,
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
        let src_node_id = src_name.parse::<NodeT>().map_err(|_| {
            Err(format!(
                concat!(
                    "The given source node name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                src_name
            ))
        })?;
        let dst_node_id = dst_name.parse::<NodeT>().map_err(|_| {
            Err(format!(
                concat!(
                    "The given destination node name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                dst_name
            ))
        })?;
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
