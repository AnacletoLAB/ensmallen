use rust_parser::*;
use libmetatest::*;
use std::collections::HashSet;

const TEMPLATE: &'static str = r#"
use super::*;
use arbitrary::Arbitrary;
use std::collections::{{HashSet, HashMap}};
use rayon::iter::ParallelIterator;

struct Rng{{
    seed: u64
}}

impl Rng {{
    pub fn new() -> Rng {{
        Rng{{
            seed: 0xbad5eed ^ unsafe{{core::arch::x86_64::_rdtsc()}}
        }}
    }}

    pub fn next(&mut self) -> u64 {{
        let mut x = self.seed;
        x ^= x << 17;
        x ^= x >> 7;
        x ^= x << 13;
        self.seed = x;
        x
    }}
}}

{structs}

{meta_struct}

pub fn meta_test(data: MetaParams) -> Result<(), String> {{
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
        data.from_vec.verbose,
    )?;

    let mut rng = Rng::new();
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
"#;

const STRUCT_TEMPLATE: &'static str = r#"
#[derive(Arbitrary, Debug, Clone)]
pub struct {struct_name} {{
{fields}
}}
"#;

const FUNCTION_CALL_TEMPLATE: &'static str = r#"
{method_id} => {{
    trace.push(format!("{func_name}({args_format})", {args_from_data}));

    let g_copy = graph.clone();
    let trace2 = trace.clone();

    std::panic::set_hook(Box::new(move |info| {
        handle_panics_meta_test_once_loaded(Some(info), data_for_panic_handler.clone(), g_copy.clone(), Some(trace2.clone()));
    }));
    {method_call}
}}
"#;

fn build(method: Function){
    let struct_name = method.name.split("_").map(|x| {
        let mut x = x.to_string();
        x.get_mut(0..1).map(|s| {s.make_ascii_uppercase(); &*s});
        x
    }).collect::<Vec<_>>().join("");
    println!("sname : {}", struct_name);
    for arg in method.args.0 {

    }


}

fn main() {
    for module in get_library_sources() {
        for imp in module.impls {
            if imp.struct_name != "Graph" {
                continue
            }
            for method in imp.methods {
                build(method);
            }
        }
    }
}