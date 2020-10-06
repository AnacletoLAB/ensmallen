extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn built_tests(_item: TokenStream) -> TokenStream {
    let mut result = String::new();
    let bools = &[true, false];
    for load_nodes in bools {
        for load_edge_types in bools {
            for load_weights in bools {
                //for directed in bools {
                    //for skip_self_loops in bools {
                        //for verbose in bools {
                            result.push_str(&format!(
                                concat!(
                                    "#[test]\n",
                                    "fn test_load_nodes_{load_nodes}_load_edge_types_{load_edge_types}_load_weights_{load_weights}_directed_{directed}_skip_self_loops{skip_self_loops}_verbose_{verbose}() {{\n",
                                    "inner_test({load_nodes}, {load_edge_types}, {load_weights}, {directed}, {verbose}, {skip_self_loops}) \n",
                                    "}}"
                                ),
                                load_nodes=load_nodes,
                                load_edge_types=load_edge_types,
                                load_weights=load_weights,
                                directed=false,
                                skip_self_loops=false,
                                verbose=true//verbose
                            ));
                        //}
                    //}
                //}
            }
        }
    }

    // convert to code
    result.parse().unwrap()
}