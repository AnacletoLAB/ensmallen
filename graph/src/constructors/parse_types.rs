use super::*;
use rayon::iter::ParallelIterator;

pub(crate) fn parse_types<TypeT: ToFromUsize>(
    types_iterator: Option<
        ItersWrapper<
            Result<(usize, String)>,
            impl Iterator<Item = Result<(usize, String)>>,
            impl ParallelIterator<Item = Result<(usize, String)>>,
        >,
    >,
    types_number: Option<TypeT>,
    numeric_type_ids: Option<bool>,
    minimum_type_id: Option<TypeT>,
    has_types: bool,
) -> Result<Option<Vocabulary<TypeT>>> {
    let numeric_type_ids = numeric_type_ids.unwrap_or(false);
    // when the graph has no node_types, the resulting vocabulary is None
    if !has_types {
        return Ok(None);
    }

    if !numeric_type_ids && minimum_type_id.is_some() {
        return Err(
            "Giving the minimum id is not meaningfull when numeric_type_ids is false.".to_string(),
        );
    }

    match (
        types_iterator,
        types_number,
        numeric_type_ids,
        minimum_type_id,
    ) {
        // If the types (either node types or edge types) are not numeric,
        // we collect them.
        (Some(nti), types_number, false, _) => {
            let types_vocabulary = Vocabulary::from_reverse_map(
                nti.map(|line| line.map(|(_, type_name)| type_name))
                    .collect::<Result<Vec<String>>>()?,
            )?;
            if let Some(types_number) = types_number{
                if TypeT::to_usize(types_number) != types_vocabulary.len(){
                    return Err(
                        format!(
                            concat!(
                                "The provided types number `{}` does not match ",
                                "the number of types computed by reading the provided ",
                                "type list iterator, which yielded `{}` types."
                            ),
                            types_number,
                            types_vocabulary.len()
                        )
                    );
                }
            }
            Ok(Some(types_vocabulary))
        },
        (Some(nti), types_number, true, _) => {
            let (min, max) = nti
                .map(|line| match line {
                    Ok((line_number, type_name)) => match type_name.parse::<TypeT>() {
                        Ok(type_id) => Ok(type_id),
                        Err(_) => Err(format!(
                            concat!(
                                "While parsing the provided node type list, ",
                                "the node type ID {:?} was found and it is not ",
                                "possible to convert it to an integer as was requested.\n",
                                "Specifically, the line with the error is {}."
                            ),
                            type_name, line_number
                        )),
                    },
                    Err(e) => Err(e),
                })
                .map(|maybe_type_id| maybe_type_id.map(|type_id| (type_id, type_id)))
                .reduce(
                    || Ok((TypeT::get_max(), TypeT::from_usize(0))),
                    |v1, v2| match (v1, v2) {
                        (Ok((min1, max1)), Ok((min2, max2))) => {
                            Ok((min1.min(min2), max1.max(max2)))
                        }
                        (Ok((min1, max1)), Err(_)) => Ok((min1, max1)),
                        (Err(_), Ok((min2, max2))) => Ok((min2, max2)),
                        (Err(e1), Err(_)) => Err(e1),
                    },
                )?;
            if let Some(types_number) = types_number{
                if types_number != max - min{
                    return Err(
                        format!(
                            concat!(
                                "The provided types number `{}` does not match ",
                                "the number of types computed by reading the provided ",
                                "type list iterator, which yielded `{}` types from the ",
                                "subtraction of the minimum and maximum values, which were ",
                                "respectively `{}` and `{}`."
                            ),
                            types_number,
                            max - min,
                            min,
                            max
                        )
                    );
                }
            }
            let minimum_node_ids = minimum_type_id.unwrap_or(min);

            if min < minimum_node_ids {
                return Err(format!(
                    concat!(
                        "The given minimum id {:?} is higher ",
                        "than the minimum id found in the iterator {:?}."
                    ),
                    minimum_node_ids, min
                ));
            }

            Ok(Some(Vocabulary::from_range(minimum_node_ids..max)))
        }
        (None, Some(ntn), true, None) => {
            Ok(Some(Vocabulary::from_range(TypeT::from_usize(0)..ntn)))
        }
        (None, Some(ntn), true, Some(min_val)) => {
            Ok(Some(Vocabulary::from_range(min_val..(
                min_val.checked_add(ntn)
                    .ok_or(format!(concat!(
                        "Error while building a numeric vocabulary, you are trying to build a range",
                        " with minimum {} and max {} + {} but the max overflows the current type.",
                    ), min_val, ntn, min_val))?
            ))))
        }
        (None, None, true, _) => {
            let min = minimum_type_id.unwrap_or(TypeT::from_usize(0));
            Ok(Some(Vocabulary::from_range(min..min)))
        }
        (None, Some(ntn), false, None) => Ok(Some(Vocabulary::with_capacity(TypeT::to_usize(ntn)))),
        (None, None, false, None) => Ok(Some(Vocabulary::new())),
        all_others => unreachable!(
            "All other cases must be explictily handled. Specifically, this case was composed of: {:?}.",
            all_others
        ),
    }
}
