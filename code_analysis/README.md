# Code analysis
This is the crate that contains all the meta-programming related code.
Everything builds on the rust analysis which is in the crate `./rust_parser`.

A simple example of a function that prints the names of all the methods of Graph is:

```rust
use rust_parser::*;

use std::fs;
use std::fs::read_dir;

// Get all the files path in the folder
let all_files: Vec<String> = read_dir("../graph/src")
    .unwrap()
    .map(|path| {
        path.unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap()
            .to_string()
    })
    .collect();

for path in all_files {
    // read the file
    let contents = fs::read_to_string(path).expect("File not found");

    // parse the file
    let module = Module::parse_lossy(contents.as_bytes());

    // iter over all the impls in the file
    for current_impl in module.impls {

        // keep only the ones which implement the struct Graph
        if current_impl.struct_name != "Graph" {
            continue;
        }

        // iter over all the methods in the current impl
        for method in current_impl.functions {
            println!("{}", method.name);
        }
    }
}
method_names
```

### Check that the code quality is up to standard
We also have a set of rules that must be enforced, to run this check do:
```bash
cargo run --release --bin check
```
it will print all the errors found and exit with code 1 if there where errors 
and 0 if the code is up to standard.

### Genereate the python bindings
Most of the python bindings are automatically generated, to do so run:
```bash
cargo run --release --bin bindgen
```

It will write two files: `../bindings/python/src/auto_generated_bindings.rs`
and `../bindings/python/src/method_names_list.ts` which contains the method names list
and the TFIDF values of each method name's term which are used for the method suggestion
to the user (`__getattr__`).

### Genereate meta-test harness
The fuzzer has a meta-harness which randomly generate a graph and randomly calls
10 methods on it with random arguments.
To generate it run:
```bash
cargo run --release --bin metatest
```
This will write the file `../fuzzing/graph_harness/src/metatest.rs`.