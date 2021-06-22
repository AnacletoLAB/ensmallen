use super::*;
use rayon::iter::ParallelIterator;
use std::str::FromStr;

pub(crate) fn parse_types<TypeT: FromStr>(
    types_iterator: Option<impl ParallelIterator<Item=Result<String>>>,
    types_number: Option<TypeT>,
    numeric_ids: bool,
    minimum_id: Option<TypeT>,
    has_node_types: bool,
) -> Result<Option<Vocabulary<TypeT>>> {
    // when the graph has no node_types, the resulting vocabulary is None
    if !has_node_types {
        return Ok(None);
    }

    if !numeric_ids && minimum_id.is_some() {
        return Err("Giving the minimum id is not meaningfull when numeric_ids is false.")
    }

    match (types_iterator, types_number, numeric_ids, minimum_id) {
        (Some(nti), _, false, None) => {
            Vocabulary::from_reverse_map(nti.collect())
        },
        (Some(nti), None, true, _) => {
            let (min, max) = nti
                .map(|x| x.parse::<TypeT>().map_err(|_|{format!("The string '{}' cannot be parsed as an integer", x)}))
                .map(|x| (x, x))
                .reduce(
                |v1, v2| {
                    match (v1, v2) {
                        (Ok(min1, max1), Ok(min2, max2)) => {
                            Ok((min1.min(min2), max1.max(max2)))
                        }
                        (Err(e), _) | (_, Err(e))=> Err(e),
                    }
                }
            )?;
            let minimum_id = minimum_id.unwrap_or(min);

            if min < minimum_id {
                // TODO! improve error
                return Err("The given minimum id is bigger than the minimum id found in the iterator");
            }

            Ok(Vocabulary::from_range(min.min(minimum_id)..max))
        },
        (None, Some(ntn), true, None) => {
            Ok(Vocabulary::from_range(0..ntn))
        }
        (None, Some(ntn), true, Some(min_val)) => {
            Ok(Vocabulary::from_range(min_val..min_val + ntn))
        }
        (None, Some(ntn), true, _) => {
            let min = minimum_id.unwrap_or(0);
            Ok(Vocabulary::from_range(min..min))
        }
        (None, Some(ntn), false, None) => {
            Ok(Vocabulary::with_capacity(ntn))
        }
        (None, None, false, None) => {
            Ok(Vocabulary::new())
        }
        // TODO! imporve error
        _ => unreachable!("All other cases must be explictily handled.")
    }
}