use super::*;

impl_struct_func!(EdgeTypeParser Vocabulary<NodeT>);

impl EdgeTypeParser {
    pub fn parse_strings<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_mutable_write();
        Ok((src, dst, vocabulary.insert(edge_type_name), weight))
    }

    pub fn parse_strings_unchecked<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_mutable_write();
        unsafe {
            Ok((
                src,
                dst,
                vocabulary.unchecked_insert_values(edge_type_name),
                weight,
            ))
        }
    }

    pub fn translate<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        Ok((src, dst, vocabulary.translate(edge_type_name), weight))
    }

    pub fn translate_unchecked<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        Ok((
            src,
            dst,
            vocabulary.unchecked_translate(edge_type_name),
            weight,
        ))
    }

    pub fn to_numeric<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        let edge_type_id = edge_type_name.parse::<EdgeTypeT>().map_err(|_| {
            Err(format!(
                concat!(
                    "The given edge type name {:?} ",
                    "cannot be parsed to an integer value."
                ),
                edge_type_name
            ))
        })?;
        if vocabulary.len() as EdgeTypeT <= edge_type_id {
            return Err(format!(
                concat!(
                    "The given edge type name {:?} ",
                    "has a value greater than the number ",
                    "of provided nodes {}."
                ),
                edge_type_id,
                vocabulary.len()
            ));
        }
        Ok((src, dst, edge_type_id, weight))
    }

    pub fn to_numeric_unchecked<T, W>(
        &mut self,
        value: Result<(T, T, Option<String>, W)>,
    ) -> Result<(T, T, Option<EdgeTypeT>, W)> {
        let (src, dst, edge_type_name, weight) = value?;
        let vocabulary = self.get_immutable();
        unsafe {
            Ok((
                src,
                dst,
                edge_type_name.parse::<EdgeTypeT>().unwrap_unchecked(),
                weight,
            ))
        }
    }
}
