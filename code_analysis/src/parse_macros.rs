use super::*;

fn parse_cached_property(macro_call: &mut MacroCall) -> Function {
    let mut item = Function::default();

    let mut data = macro_call.content.as_bytes();
    item.name = parse!(data, Identifier).into();

    // skip the comma
    data = skip_whitespace(&data[1..]);

    item.return_type = Some(parse!(data, Type));
    // skip the comma
    data = skip_whitespace(&data[1..]);

    let _caching_method = parse!(data, Identifier);
    // skip the comma
    data = skip_whitespace(&data[1..]);

    let _caching_attribute = parse!(data, Identifier);
    // skip the comma
    data = skip_whitespace(&data[1..]);

    // parse the documentations
    let doc_lines = String::from_utf8(data.to_vec()).unwrap();
    let mut doc = String::new();
    for doc_line in doc_lines.split('\n') {
        // remove extra white space
        match doc_line.trim().strip_prefix("///") {
            None => {
                // maybe panic?
            }
            Some(doc_line) => {
                doc.push_str(doc_line.trim());
                doc.push('\n');
            }
        }
    }
    item.doc = doc;
    item.visibility = Visibility::Public;
    item.attributes = macro_call.attributes.clone();
    item.args = Args(vec![Arg{
        name: "self".to_string(),
        arg_modifier: TypeModifiers{
            reference: true,
            mutable: false,
            ..TypeModifiers::default()
        },
        arg_type: Type::parse_lossy_str("&self"),
    }]);
    item.is_unsafe = item.name.contains("unchecked");
    item
}

#[must_use]
/// Expand the macro calls to the generated methods
///
/// This curently handle the following macros:
/// * `cached_property`
/// 
/// # Panics 
/// The function panics if the macros are called in impossible places.
/// Codes that compiles should never make this function panic.
pub fn parse_macros(mut module: Module) -> Module {
    let mut new_functions = Vec::new();

    for macro_call in &mut module.macro_calls {
        let new_function: Option<Function> = match macro_call.name.as_str() {
            "cached_property" => panic!("A call to cached_property outside an impl is not possible."),
            // Macro not handled so it's ignored`
            x => {
                println!("Macrocall '{}' ignored because it's not yet implemented", x);
                None
            }
        };

        if let Some(mut nf) = new_function {
            nf.file_path = module.file_path.clone();
            new_functions.push(nf);
        }
    }
    module.functions.append(&mut new_functions);

    for imp in &mut module.impls {
        let mut new_methods = Vec::new();
        for macro_call in &mut imp.macro_calls {
            let new_method: Option<Function>  = match macro_call.name.as_str() {
                "cached_property" => Some(parse_cached_property(macro_call)),
                // Macro not handled so it's ignored`
                x => {
                    println!("Macrocall '{}' ignored because it's not yet implemented", x);
                    None
                }
            };
            
            if let Some(mut nm) = new_method {
                nm.file_path = module.file_path.clone();
                nm.class = Some(imp.struct_name.clone());
                new_methods.push(nm);
            }
        }
        imp.methods.append(&mut new_methods);
    }


    module
}