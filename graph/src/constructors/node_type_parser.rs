use super::*;

impl_struct_func!(NodeTypeParser Vocabulary<NodeTypeT>);

impl NodeTypeParser {
    pub fn ignore<N>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, _)) = value?;
        Ok((line_number, (node_name, None)))
    }

    pub fn parse_strings<N>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            line_number,
            (
                node_name,
                node_type_names.map_or(Ok::<_, String>(None), |ntns| {
                    Ok(Some(
                        ntns.into_iter()
                            .map(|ntn| Ok(vocabulary.0.insert(ntn)?.0))
                            .collect::<Result<Vec<NodeTypeT>>>()?,
                    ))
                })?,
            ),
        ))
    }

    pub fn parse_strings_unchecked<N>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            line_number,
            (
                node_name,
                node_type_names.map(|ntns| {
                    ntns.into_iter()
                        .map(|ntn| unsafe { vocabulary.0.unchecked_insert(ntn) })
                        .collect::<Vec<NodeTypeT>>()
                }),
            ),
        ))
    }

    pub fn get<N: std::fmt::Debug + Clone>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let vocabulary = self.get_immutable();
        let node_ids = node_type_names.map_or(Ok::<_, String>(None), |ntns| {
            let ids = ntns
                .into_iter()
                .map(|node_type_name| match vocabulary.get(&node_type_name) {
                    Some(node_type_id) => Ok(node_type_id),
                    None => Err(format!(
                        concat!(
                            "Found an unknown node type while reading the node list.\n",
                            "Specifically the unknown node type is {:?}.\n",
                            "The list of the known node types is {:#4?}"
                        ),
                        node_type_name,
                        vocabulary.keys()
                    )),
                })
                .collect::<Result<Vec<NodeTypeT>>>()?;
            if ids.is_empty() {
                return Err(format!(
                    concat!(
                        "The node {:?} has an empty node types list, which ",
                        "should be provided as an unknown field. If you are getting ",
                        "this error from reading a file, it should be provided as ",
                        "an empty field."
                    ),
                    node_name.clone()
                ));
            }
            Ok(Some(ids))
        })?;
        Ok((line_number, (node_name, node_ids)))
    }

    pub fn get_unchecked<N>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            line_number,
            (
                node_name,
                node_type_names.map(|ntns| {
                    ntns.into_iter()
                        .map(|ntn| vocabulary.get(&ntn).unwrap())
                        .collect::<Vec<NodeTypeT>>()
                }),
            ),
        ))
    }

    pub fn to_numeric<N: std::fmt::Debug + Clone>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let vocabulary = self.get_immutable();
        let node_type_ids = node_type_names.map_or(Ok::<_, String>(None), |ntns| {
            let node_type_ids = ntns
                .into_iter()
                .map(|node_type_name| {
                    let node_type_id = match node_type_name.parse::<NodeTypeT>() {
                        Ok(node_type_id) => Ok(node_type_id),
                        Err(_) => Err(format!(
                            concat!(
                                "The given node type name {:?} ",
                                "cannot be parsed to an integer value."
                            ),
                            node_type_name
                        )),
                    }?;
                    if vocabulary.len() as NodeTypeT <= node_type_id {
                        return Err(format!(
                            concat!(
                                "The given node type name {:?} ",
                                "has a value greater than the number ",
                                "of provided node types {}."
                            ),
                            node_type_name,
                            vocabulary.len()
                        ));
                    }
                    Ok(node_type_id)
                })
                .collect::<Result<Vec<NodeTypeT>>>()?;
            if node_type_ids.is_empty() {
                return Err(format!(
                    concat!(
                        "The node {:?} has an empty node types list, which ",
                        "should be provided as an unknown field. If you are getting ",
                        "this error from reading a file, it should be provided as ",
                        "an empty field."
                    ),
                    node_name.clone()
                ));
            }
            Ok(Some(node_type_ids))
        })?;
        Ok((line_number, (node_name, node_type_ids)))
    }

    pub fn to_numeric_unchecked<N>(
        &mut self,
        value: Result<(usize, (N, Option<Vec<String>>))>,
    ) -> Result<(usize, (N, Option<Vec<NodeTypeT>>))> {
        let (line_number, (node_name, node_type_names)) = value?;
        let node_type_ids = node_type_names.map(|ntns| {
            ntns.into_iter()
                .map(|node_type_name| node_type_name.parse::<NodeTypeT>().unwrap())
                .collect::<Vec<NodeTypeT>>()
        });
        Ok((line_number, (node_name, node_type_ids)))
    }
}
