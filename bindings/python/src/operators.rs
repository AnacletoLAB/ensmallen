use super::*;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::number::PyNumberProtocol;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use strsim::damerau_levenshtein;

#[pyproto]
impl PyNumberProtocol for EnsmallenGraph {
    fn __or__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph | &rhs.graph)?,
        })
    }

    fn __sub__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph - &rhs.graph)?,
        })
    }

    fn __and__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph & &rhs.graph)?,
        })
    }

    fn __xor__(lhs: EnsmallenGraph, rhs: EnsmallenGraph) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(&lhs.graph ^ &rhs.graph)?,
        })
    }
}

/// Returns the given method name separated in the component parts.
///
/// # Implementative details
/// The methods contains terms such as:
/// * `node_name`
/// * `node_type_id`
/// * `node_id`
///
/// Since these terms are functionally a single word, we do not split
/// the terms composed by the words:
/// * `id` or `ids`
/// * `type` or `types`
/// * `name` or `names`
///
/// # Arguments
/// * `method_name`: &str - Name of the method to split.
fn split_words(method_name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in method_name.split("_") {
        match word {
            "type" | "types" | "id" | "ids" | "name" | "names" => match result.last_mut() {
                Some(last) => {
                    last.push('_');
                    last.extend(word.chars());
                }
                None => {
                    result.push(word.to_string());
                }
            },
            _ => {
                result.push(word.to_string());
            }
        };
    }

    result.into_iter().filter(|x| !x.is_empty()).collect()
}

#[pyproto]
impl PyObjectProtocol for EnsmallenGraph {
    fn __str__(&'p self) -> PyResult<String> {
        pe!(self.graph.textual_report(Some(true)))
    }
    fn __repr__(&'p self) -> PyResult<String> {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        Ok(self.hash() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // Compute the similarity between each token in the query
        // and the vector of terms for the pre-computed tfidf matrix 
        let edit_distances = tokens.iter()
            .map(|token| {
                TFIDF_TERMS.iter()
                    .map(|&term| {
                        1.0 - (
                            damerau_levenshtein(&token, &term) as f64 
                            / std::cmp::max(token.len(), term.len()) as f64
                        )
                    })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = TFIDF_FREQUENCIES.par_iter()
            .enumerate()
            .map(|(id, frequencies_doc)| {
                (id, frequencies_doc.iter()
                    .zip(edit_distances.iter())
                    .map(|(term_frequency, term_similarities)| {
                        term_similarities.iter()
                            .map(|term_similarity| term_similarity.exp() * term_frequency)
                            .sum::<f64>()
                    }).sum::<f64>())
            })
            .collect::<Vec<(usize, f64)>>();
        
        
        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());
        println!("{:?}", doc_scores.iter().map(|(id, score)| (METHODS_NAMES[*id], *score)).collect::<Vec<(&str, f64)>>());

        Err(PyTypeError::new_err(format!(
            "The method {} does not exists, did you mean {:?}?",
            name,
            doc_scores.iter()
                .map(|(method_id, _)| METHODS_NAMES[*method_id].to_string())
                .collect::<Vec<String>>(),
        )))
        
        // Err(PyTypeError::new_err(format!(
        //     "The method {} does not exists, did you mean one of the followings?\n{:#4?}",
        //     name,
        //     &methods_scores[..5].iter()
        //         .map(|(method, _)| method.to_string())
        //         .collect::<Vec<String>>(),
        // )))
    }
}

#[pymethods]
impl EnsmallenGraph {
    fn _repr_html_(&self) -> PyResult<String> {
        Ok(format!(
            r#"<h4>{}</h4><p style="text-align: justify; text-justify: inter-word;">{}</p>"#,
            self.graph.get_name(),
            pe!(self.__repr__())?
        ))
    }
}
