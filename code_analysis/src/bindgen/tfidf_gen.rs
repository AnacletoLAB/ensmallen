use super::*;

use graph::okapi_bm25_tfidf;

/// Pre-compute the TD-IDF weight for each term of each binding.
/// Then write the compute weights in a file at the given path.
pub fn tfidf_gen(file_path: &str) {

    println!("Reading all the bindings names");
    print_sep();
    // Get the names of all the bdingins
    let method_names = get_binding_names();

    print_sep();
    println!("Generating the TF-IDF weights for the name of the bindings at {}", file_path);
    print_sep();

    let documents = method_names
        .iter()
        .map(|x| split_words(x))
        .collect::<Vec<Vec<String>>>();

    let vals = documents
        .iter()
        .map(|x| x.iter().map(|y| y.as_str()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tfidf = okapi_bm25_tfidf(&vals[..], None, None, None, None).unwrap();

    let mut terms = HashSet::new();
    documents.iter().for_each(|document| {
        document.iter().for_each(|term| {
            terms.insert(term);
        });
    });

    let method_names_list = format!(
        r#"pub const METHODS_NAMES: &'static [&'static str] = &[
{}
];

pub const TERMS: &'static [&'static str] = &[
{}
];

pub const TFIDF_FREQUENCIES: &'static [&'static [(&'static str, f64)]] = &[
{}
];
"#,
        method_names
            .iter()
            .map(|x| format!("    \"{}\",", x))
            .collect::<Vec<String>>()
            .join("\n"),
        terms
            .iter()
            .map(|x| format!("    \"{}\",", x))
            .collect::<Vec<String>>()
            .join("\n"),
        tfidf
            .iter()
            .map(|vals| format!("&{:?},", vals.iter().collect::<Vec<(&String, &f64)>>()))
            .collect::<Vec<String>>()
            .join("\n"),
    );


    // Write to file
    fs::write(
        file_path,
        method_names_list,
    )
    .expect("Cannot write the method names list file");
}


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

