use super::*;

impl_struct_func!(NodeTypeParser Vocabulary<NodeTypeT>);

impl NodeTypeParser {
    pub fn ignore<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        Ok((value?.0, None))
    }

    pub fn parse_strings<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((node_name, node_type_names.unwrap_unchecked().into_iter().map(|ntn|{
            vocabulary.insert_value()?
        }))
    }

    pub fn parse_strings_unchecked<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((
            node_name,
            vocabulary.unchecked_insert_values(node_type_names)?,
        ))
    }

    pub fn get<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_immutable();
        Ok((node_name, vocabulary.get(node_type_names)?))
    }

    pub fn get_unchecked<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_immutable();
        Ok((node_name, unsafe {
            vocabulary.unchecked_get(node_type_names)?
        }))
    }

    pub fn to_numeric<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_immutable();
        let node_type_ids = node_type_names.map_or(
            || Ok(None),
            |ntns| {
                Ok(Some(
                    ntns.into_iter()
                        .map(|node_type_name| {
                            let node_type_id =
                                node_type_name.parse::<NodeTypeT>().map_err(|_| {
                                    Err(format!(
                                        concat!(
                                            "The given node type name {:?} ",
                                            "cannot be parsed to an integer value."
                                        ),
                                        node_type_name
                                    ))
                                })?;
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
                        .collect::<Result<Vec<NodeTypeT>>>()?,
                ))
            },
        )?;
        Ok((node_name, node_type_ids))
    }

    pub fn to_numeric_unchecked<N>(
        &mut self,
        value: Result<(N, Option<Vec<String>>)>,
    ) -> Result<(N, Option<Vec<NodeTypeT>>)> {
        let (node_name, node_type_names) = value?;
        let vocabulary = self.get_immutable();
        let node_type_ids = node_type_names.map(|ntns| unsafe {
            ntns.into_iter()
                .map(|node_type_name| node_type_name.parse::<NodeTypeT>().unwrap_unchecked())
                .collect::<Vec<NodeTypeT>>()
        })?;
        Ok((node_name, node_type_ids))
    }
}
