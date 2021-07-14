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
                unsafe { vocabulary.get(&src_name).unwrap_unchecked() },
                unsafe { vocabulary.get(&dst_name).unwrap_unchecked() },
                edge_type_name,
                weight,
            ),
        ))
    }

    pub fn to_numeric<E, W>(
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

    pub fn to_numeric_unchecked<E, W>(
        &mut self,
        value: Result<(usize, (String, String, E, W))>,
    ) -> Result<(usize, (NodeT, NodeT, E, W))> {
        let (line_number, (src_name, dst_name, edge_type_name, weight)) = value?;
        unsafe {
            Ok((
                line_number,
                (
                    src_name.parse::<NodeT>().unwrap_unchecked(),
                    dst_name.parse::<NodeT>().unwrap_unchecked(),
                    edge_type_name,
                    weight,
                ),
            ))
        }
    }
}
