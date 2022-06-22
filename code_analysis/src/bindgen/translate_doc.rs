use super::*;

pub fn translate_doc(doc: &str, user_defined_types: &[&str]) -> String {
    let mut result = String::new();

    // parse the documentation into sections
    let (_, doc) = Doc::parse(doc.as_bytes());
    let sections = doc.sections;

    for section in sections {
        match section {
            DocSection::Introduction(intro) => {
                result.push_str(&bytes_to_string(trim(intro.as_bytes())));
            }
            DocSection::Arguments {
                arguments,
                ..
            } => {
                result.push_str("\n\nParameters\n----------\n");

                //args_sec.extend(prologue.chars());
                let mut handle_walk_parameters = false; 
                for argument in arguments {
                    match argument {
                        Argument::Parsable(DocArg {
                            name,
                            arg_type,
                            description,
                        }) => {
                            let translated_arg_type = translate_type_str(arg_type, &user_defined_types);
                            if translated_arg_type == "WalksParameters" {
                                handle_walk_parameters=true;
                            } else {
                                result.push_str(
                                    &format!(
                                        "{name}: {arg_type}\n    {description}\n",
                                        name = name,
                                        arg_type = translated_arg_type,
                                        description = description,
                                    )
                                )
                            }
                        },
                        Argument::NotParsable(_) => {}
                    }
                }

                if handle_walk_parameters {
                    result.push_str(
&r#"
return_weight: float = 1.0
    Weight on the probability of returning to node coming from
    Having this higher tends the walks to be
    more like a Breadth-First Search.
    Having this very high  (> 2) makes search very local.
    Equal to the inverse of p in the Node2Vec paper.
explore_weight: float = 1.0
    Weight on the probability of visiting a neighbor node
    to the one we're coming from in the random walk
    Having this higher tends the walks to be
    more like a Depth-First Search.
    Having this very high makes search more outward.
    Having this very low makes search very local.
    Equal to the inverse of q in the Node2Vec paper.
change_edge_type_weight: float = 1.0
    Weight on the probability of visiting a neighbor node of a
    different type than the previous node. This only applies to
    colored graphs, otherwise it has no impact.
change_node_type_weight: float = 1.0
    Weight on the probability of visiting a neighbor edge of a
    different type than the previous edge. This only applies to
    multigraphs, otherwise it has no impact.
random_state: int = 42
    random_state to use to reproduce the walks.
iterations: int = 1
    Number of cycles on the graphs to execute.
dense_node_mapping: Dict[int, int] = None
    Mapping to use for converting sparse walk space into a dense space.
    This object can be created using the method available from graph
    called `get_dense_node_mapping` that returns a mapping from
    the non trap nodes (those from where a walk could start) and
    maps these nodes into a dense range of values.
max_neighbours: Optional[int] = 100
    Maximum number of randomly sampled neighbours to consider.
    If this parameter is used, the walks becomes probabilistic in nature
    and becomes an approximation of an exact walk.
normalize_by_degree: Optional[bool] = False
    Whether to normalize the random walks by the node degree.
"#[1..]
                    );
                }

                //args_sec.extend(epilogue.chars());
            }
            DocSection::Raises {
                exceptions,
                ..
            } => {
                result.push_str("\n\nRaises\n-------\n");

                for excp in exceptions {
                    result.push_str(&format!("ValueError\n    {}\n", excp));
                }
            }
            DocSection::Unsafe { text } => {
                result.push_str("\n\nSafety\n------\n");
                result.push_str(&text);
            }
            _ => {}
        }
    }

    result
        .split('\n')
        .map(|x| format!("    /// {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}