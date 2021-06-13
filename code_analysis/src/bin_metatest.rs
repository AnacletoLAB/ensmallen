use rust_parser::*;
use libcodeanalysis::*;
use std::collections::HashSet;

const METHODS_BLACKLIST: &'static [&'static str] = &[
    "eq",
    "hash",
];

const TYPES_BLACKLIST: &'static [&'static str] = &[
    "Fn", 
    "NodeFileReader", 
    "EdgeFileReader", 
    "Graph", 
    "Compute_hash_Params",
    "str",
    "S",
    "[u32]",
    "SingleWalkParameters",
    "WalksParameters",
    "WalkWeights",
    "Self",
    "[String]",
];

fn build(method_id: usize, method: Function) -> Option<(String, String, String, String)> {
    let struct_name = method.name.split("_").map(|x| {
        let mut x = x.to_string();
        x.get_mut(0..1).map(|s| {s.make_ascii_uppercase(); &*s});
        x
    }).collect::<Vec<_>>().join("");

    let struct_field_name = method.name.split("_").collect::<Vec<_>>().join("");

    let fuzz_types = method.attributes.iter()
        .filter_map(|attr| attr.parse_fuzz_type())
        .collect::<Vec<_>>();
    
    let mut fields = Vec::new();
    let mut call_args = Vec::new();
    let mut arg_names = Vec::new();
    for arg in method.args.0.clone() {
        // check if the type is banned or not
        for deny_type in TYPES_BLACKLIST {
            if String::from(arg.arg_type.clone()).contains(deny_type) {
                return None;
            }
        }

        match fuzz_types.iter().find(|x| x.0 == arg.name) {
            Some((fuzz_name, fuzz_type)) => {
                match (fuzz_type, arg.arg_type) {
                    (x, y) if x == "Option<_>" && y == "Option<_>" => {
                        arg_names.push(arg.name.clone());
                        fields.push((arg.name.clone(), fuzz_type.to_string()));

                        // Extract the value inside the option
                        let prim_type = match y {
                            Type::SimpleType{
                                generics,
                                ..
                            } => {
                                match &generics[0] {
                                    GenericValue::Type(result) => result.clone(),
                                    _ => unreachable!("An option should only have a type as generics"),
                                }
                            }
                            _ => unreachable!("The if should already have checked this."),
                        };

                        call_args.push(format!("data.{}.{}.map(|x| x as {})", struct_field_name, arg.name, prim_type));
                    }
                    (x, y) if x == "Primitive" && y == "Primitive" => {
                        arg_names.push(arg.name.clone());
                        fields.push((arg.name.clone(), x.to_string()));
                        call_args.push(format!("data.{}.{} as {}", struct_field_name, arg.name, y));
                    }
                    _ => panic!("The fuzz type attribute was called with not-supported types."),
                }
                continue;
            }
            None => {}
        };


        match arg.arg_type {
            x if x == "self" || x == "&self" || x == "&mut self" => {},
            Type::SimpleType{
                name,
                mut modifiers,
                generics,
                traits,
            } => {
                arg_names.push(arg.name.clone());
                fields.push((arg.name.clone(), Type::SimpleType{
                    name,
                    modifiers: TypeModifiers::default(),
                    generics,
                    traits,
                }.to_string()));
                modifiers.lifetime = None;
                call_args.push(format!("{}data.{}.{}", modifiers, struct_field_name, arg.name));
            }
            _ => {
                arg_names.push(arg.name.clone());
                fields.push((arg.name.clone(), arg.arg_type.to_string()));
                call_args.push(format!("data.{}.{}", struct_field_name, arg.name));
            }
        }
    }

    let (struct_string, field_string) = if fields.len() > 0 {
        (format!(
r#"
#[derive(Arbitrary, Debug, Clone)]
pub struct {struct_name} {{
{fields}
}}
"#,
            struct_name=struct_name,
            fields=fields.iter().map(|(name, field_type)| {
                format!("    pub {} : {},", name, field_type)
            }).collect::<Vec<_>>().join("\n"),
        ), 
        format!("    pub {} : {},", struct_field_name, struct_name)
    )
    } else {
        (String::new(), String::new())
    };

    let mut method_call = format!(
        "graph.{}({})", 
        method.name, 
        call_args.iter()
            .map(|x| format!("{}.clone()", x))
            .collect::<Vec<_>>().join(", ")
    );

    if let Some(rt) = method.return_type.clone() {
        method_call = match rt {
            x if x == "Graph" => {
                format!("graph = {};", method_call)
            }
            x if x  == "Result<Graph, _>" => {
                format!(
        r#"
        if let Ok(res) = {} {{
            graph = res;
        }}
        "#,     
                method_call)
            }
            x if x  == "Result<(Graph, Graph), _>" => {
                format!(
        r#"
        if let Ok((res1, res2)) = {} {{
            if rng.next() % 2 == 0 {{
                graph = res1;
            }} else {{
                graph = res2;
            }}
        }}
        "#,     
                method_call)
            }
            x if x.cmp_str_without_modifiers("impl Iterator<Item=_>")
                || x.cmp_str_without_modifiers("impl IndexedParallelIterator<Item=_>")
                || x.cmp_str_without_modifiers("Box<impl Iterator<Item=_>>")
                || x.cmp_str_without_modifiers("impl ParallelIterator<Item=_>") => {
                format!(
            r#"
            let _ = {}.collect::<Vec<_>>();
            "#,
                method_call)
            }
            x if x.cmp_str_without_modifiers("Result<impl Iterator<Item = _>, _>") 
                || x.cmp_str_without_modifiers("Result<impl IndexedParallelIterator<Item = _>, _>") 
                || x.cmp_str_without_modifiers("Result<impl ParallelIterator<Item = _>, _>") => {
                format!(
            r#"
            let _ = {}.map(|x| x.collect::<Vec<_>>());
            "#,
                method_call)
            }
            x @ _ => {
                format!("let _ = {};", method_call)
            }
        };
    }

    let method_calls_without_handling = format!( r#"
    {method_id} => {{
        {method_call}
    }}
    "#,
        method_id=method_id,
        method_call=method_call,
    );

    method_call = format!( r#"
    {method_id} => {{
        trace.push(format!("{func_name}({args_format})", {args_from_data}));
    
        let g_copy = graph.clone();
        let trace2 = trace.clone();
    
        std::panic::set_hook(Box::new(move |info| {{
            handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
        }}));
        {method_call}
    }}
    "#,
        method_id=method_id,
        func_name=method.name,
        method_call=method_call,
        args_format=arg_names.iter().map(|x| format!("{}: {{:?}}", x)).collect::<Vec<_>>().join(", "),
        args_from_data=call_args.iter().map(|x| format!("&{}", x)).collect::<Vec<_>>().join(", "),
    );

    Some((
        struct_string,
        field_string,
        method_call,
        method_calls_without_handling,
    ))
}

fn main() {
    let mut counter = 0;
    let mut calls = Vec::new();
    let mut calls_no_panic = Vec::new();
    let mut structs = Vec::new();
    let mut fields = Vec::new();
    for module in get_library_sources() {
        for imp in module.impls {
            if imp.struct_name != "Graph" {
                continue
            }
            for method in imp.methods {

                if METHODS_BLACKLIST.contains(&method.name.as_str()) 
                    || method.name.starts_with("from")
                    || method.visibility != Visibility::Public
                    || method.is_unsafe() {
                    continue
                }

                if let Some((struct_string, struct_field, method_call, method_calls_without_handling)) = build(counter, method) {
                    if struct_string != "" {
                        structs.push(struct_string);
                    }
                    calls.push(method_call);
                    calls_no_panic.push(method_calls_without_handling);
                    fields.push(struct_field);
                    counter += 1;
                }
            }
        }
    }

    println!("Generated the harnesses for {} methods.", counter - 1);

    let meta_struct = format!(
r#"
#[derive(Arbitrary, Debug, Clone)]
pub struct MetaParams {{
    pub seed: u64,
{fields}
    pub from_vec: FromVecHarnessParams,
}}
"#,
        fields=fields.join("\n"),
    );

    let result = format!(
        r#"
use super::*;
use arbitrary::Arbitrary;
use std::collections::{{HashSet, HashMap}};
use rayon::iter::ParallelIterator;

struct Rng{{
    seed: u64
}}

impl Rng {{
    pub fn new(seed: u64) -> Rng {{
        Rng{{
            seed: seed,
        }}
    }}

    pub fn next(&mut self) -> u64 {{
        let mut x = self.seed;
        x = x.wrapping_add(0xbadf00ddeadbeef);
        x ^= x << 17;
        x ^= x >> 7;
        x ^= x << 13;
        self.seed = x;
        x
    }}
}}

{structs}

{meta_struct}

pub fn meta_test_harness_with_panic_handling(data: MetaParams) -> Result<(), String> {{
    let panic_handler_data_before_load = data.clone();
    let data_copy_for_tests = data.clone();
    std::panic::set_hook(Box::new(move |info| {{
        handle_panics_meta_test(Some(info), panic_handler_data_before_load.clone(), None);
    }}));

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        "MetaTest",
        data.from_vec.ignore_duplicated_nodes,
        false,
        data.from_vec.ignore_duplicated_edges,
        false,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_edge_weights,
        true,
        true,
        true,
        true,
        data.from_vec.verbose,
    )?;

    let mut rng = Rng::new(data.seed);
    let mut trace = Vec::new();
    for _ in 0..10 {{
        let data_for_current_test = data_copy_for_tests.clone();
        let data_for_panic_handler = data_copy_for_tests.clone();
        match rng.next() % {n_of_calls} {{
{calls}
            _ => unreachable!()
        }}
    }}
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}}

pub fn meta_test_harness(data: MetaParams) -> Result<(), String> {{

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        "MetaTest",
        data.from_vec.ignore_duplicated_nodes,
        false,
        data.from_vec.ignore_duplicated_edges,
        false,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_edge_weights,
        true,
        true,
        true,
        true,
        data.from_vec.verbose,
    )?;

    let mut rng = Rng::new(data.seed);
    for _ in 0..10 {{
        match rng.next() % {n_of_calls} {{
{calls_no_panic}
            _ => unreachable!()
        }}
    }}
    
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}}
"#,
        n_of_calls=counter,
        calls=calls.join("\n"),
        calls_no_panic=calls_no_panic.join("\n"),
        structs=structs.join("\n"),
        meta_struct=meta_struct,
    );

    std::fs::write("../fuzzing/graph_harness/src/meta_test.rs", result);
}