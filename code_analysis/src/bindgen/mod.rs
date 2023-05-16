use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;

mod binding;
pub use binding::*;

mod skeleton;
pub use skeleton::*;

mod translate_doc;
pub use translate_doc::*;

mod translate_type;
pub use translate_type::*;

mod tfidf_gen;
pub use tfidf_gen::*;

pub fn extract_module_name_from_path(path: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\.\./graph/src/(.+)/.+\.rs").unwrap();
    }
    RE.captures(path)
        .map(|x| x.get(1).unwrap().as_str().to_string())
}

fn extract_module_name_from_func(func: &Function) -> Option<String> {
    extract_module_name_from_path(func.file_path.as_str()).or_else(|| {
        let module_attr = func
            .attributes
            .iter()
            .find(|attr| attr.0.starts_with("module"));
        module_attr.map(|module| module.0[7..module.0.len() - 1].to_string())
    })
}

fn extract_module_name_from_struct(ztruct: &Struct) -> Option<String> {
    extract_module_name_from_path(ztruct.file_path.as_str()).or_else(|| {
        let module_attr = ztruct
            .attributes
            .iter()
            .find(|attr| attr.0.starts_with("module"));
        module_attr.map(|module| module.0[7..module.0.len() - 1].to_string())
    })
}

/// If we should emit a binding for the given function
fn is_to_bind(func: &Function) -> bool {
    !func.name.starts_with("iter")
        && !func.name.starts_with("par_iter")
        && func.visibility == Visibility::Public
        && !func.attributes.iter().any(|x| x == "no_binding")
        && !func.attributes.iter().any(|x| x == "manual_binding")
        && func
            .return_type
            .as_ref()
            .map(|x| !x.to_string().contains("Iterator"))
            .unwrap_or(true)
}

macro_rules! format_vec {
    ($values:expr, $fmt_str:literal, $join_sep:literal) => {
        $values
            .iter()
            .map(|x| format!($fmt_str, x))
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>()
            .join($join_sep)
    };
}

/// General trait for objects that can emit the code for their python bindings
trait GenBinding {
    fn gen_python_binding(self: &Self) -> String;
}

#[derive(Clone, Debug)]
struct Class {
    ztruct: Struct,
    impls: Vec<Impl>,
}

impl Class {
    fn is_empty(&self) -> bool {
        self.impls.is_empty()
    }

    fn new(ztruct: Struct) -> Class {
        Class {
            ztruct,
            impls: Vec::new(),
        }
    }

    fn get_methods_names(&self) -> Vec<&str> {
        let mut result = Vec::new();
        assert!(!self.impls.is_empty(), "The set of impls is empty!");
        for imp in &self.impls {
            for method in &imp.methods {
                if is_to_bind(method) {
                    result.push(method.name.as_str());
                }
            }
        }
        assert!(!result.is_empty(), "The set of methods is empty!");
        result
    }
}

impl GenBinding for Class {
    fn gen_python_binding(&self) -> String {
        let methods_names = self.get_methods_names();

        assert!(!methods_names.is_empty(), "The method names are empty!");

        let (terms, tfidf) = tfidf_gen(&methods_names);

        let derive = if self.ztruct.attributes.iter().any(|x| x.0.contains("Clone")) {
            ", Clone"
        } else {
            ""
        };

        let impl_ord = self.impls.iter().any(|x| {
            let trait_impl = x
                .impl_trait
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or(String::new());

            trait_impl.contains("Ord")
        }) && self
            .impls
            .iter()
            .any(|x| x.methods.iter().any(|f| f.name == "cmp"));
        let impl_partial_ord = self.impls.iter().any(|x| {
            let trait_impl = x
                .impl_trait
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or(String::new());

            trait_impl.contains("PartialOrd")
        }) && self
            .impls
            .iter()
            .any(|x| x.methods.iter().any(|f| f.name == "partial_cmp"));
        println!(
            "Richcmp: {:30} Ord: {:6} PartialOrd: {:6}",
            self.ztruct.struct_type.to_string(),
            impl_ord,
            impl_partial_ord,
        );
        let cmp_impl = if impl_ord || impl_partial_ord {
            r#"
fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
    match op {
        CompareOp::Lt => self.inner < other.inner,
        CompareOp::Le => self.inner <= other.inner,
        CompareOp::Eq => self.inner == other.inner,
        CompareOp::Ne => self.inner != other.inner,
        CompareOp::Gt => self.inner > other.inner,
        CompareOp::Ge => self.inner >= other.inner,
    }
}
"#
        } else {
            ""
        };

        format!(
            r#"
{struct_doc}
#[derive(Debug{derive})]
pub struct {struct_name} {{
    pub inner: graph::{struct_name},
}}

impl From<graph::{struct_name}> for {struct_name} {{
    fn from(val: graph::{struct_name}) -> {struct_name} {{
        {struct_name}{{inner: val}}
    }}
}}

impl From<{struct_name}> for graph::{struct_name} {{
    fn from(val: {struct_name}) -> graph::{struct_name} {{
        val.inner
    }}
}}

impl<'a> From<&'a {struct_name}> for &'a graph::{struct_name} {{
    fn from(val: &'a {struct_name}) -> &'a graph::{struct_name} {{
        &val.inner
    }}
}}

#[pymethods]
impl {struct_name} {{
{methods}
}}

pub const {struct_name_upper}_METHODS_NAMES: &[&str] = &[
{method_names}
];

pub const {struct_name_upper}_TERMS: &[&str] = &[
{terms}
];

pub const {struct_name_upper}_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
{tfidf}
];

#[pymethods]
impl {struct_name} {{
    fn _repr_html_(&self) -> String {{
        self.__repr__()
    }}
}}
            
#[pymethods]
impl {struct_name} {{
    fn __str__(&self) -> String {{
        self.inner.to_string()
    }}
    fn __repr__(&self) -> String {{
        self.__str__()
    }}

    fn __hash__(&self) -> PyResult<isize> {{
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }}

    {cmp_impl}

    fn __getattr__(&self, name: String) -> PyResult<()> {{
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {{
                let mut similarities = {struct_name_upper}_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            }})
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = {struct_name_upper}_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {{
                (
                    id,
                    jaro_winkler(&name, {struct_name_upper}_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {{
                                match tokens_expanded.iter().find(|(token, _)| token == term) {{
                                    Some((_, similarity)) => similarity * weight,
                                    None => 0.0,
                                }}
                            }})
                            .sum::<f64>(),
                )
            }})
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method `{{}}` does not exists, did you mean one of the following?\n\n{{}}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {{ 
                    format!("* `{{}}`", {struct_name_upper}_METHODS_NAMES[*method_id].to_string()) 
                }})
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }}
}}
"#,
            struct_doc = self
                .ztruct
                .doc
                .trim()
                .split("\n")
                .map(|x| format!("/// {}", x))
                .collect::<Vec<_>>()
                .join("\n")
                .trim(),
            derive = derive,
            struct_name = self.ztruct.struct_type.get_name().unwrap(),
            struct_name_upper = self.ztruct.struct_type.get_name().unwrap().to_uppercase(),
            methods = format_vec!(
                self.impls
                    .iter()
                    .flat_map(|imp| imp
                        .methods
                        .iter()
                        .filter(|func| is_to_bind(func))
                        .map(GenBinding::gen_python_binding)
                        .filter(|x| !x.is_empty()))
                    .collect::<Vec<_>>(),
                "{}",
                "\n\n"
            ),
            cmp_impl = cmp_impl,
            method_names = format_vec!(methods_names, "    \"{}\",", "\n"),
            terms = format_vec!(terms, "    \"{}\",", "\n"),
            tfidf = format_vec!(tfidf, "&{:?},", "\n"),
        )
    }
}

#[derive(Clone, Debug)]
struct BindingsModule {
    module_name: String,
    modules: BTreeMap<String, BindingsModule>,
    funcs: Vec<Function>,
    structs: BTreeMap<String, Class>,
}

impl BindingsModule {
    fn is_empty(&self) -> bool {
        self.structs.is_empty()
    }

    fn push_class(&mut self, ztruct: Struct) {
        self.structs
            .insert(ztruct.struct_type.get_name().unwrap(), Class::new(ztruct));
    }

    fn new(name: String) -> Self {
        BindingsModule {
            module_name: name,
            modules: BTreeMap::new(),
            funcs: Vec::new(),
            structs: BTreeMap::new(),
        }
    }

    fn get_submodule(&mut self, name: Option<String>) -> &mut BindingsModule {
        if let Some(module_name) = name {
            self.modules
                .entry(module_name.clone())
                .or_insert_with(move || BindingsModule::new(module_name))
        } else {
            self
        }
    }
}

impl GenBinding for BindingsModule {
    fn gen_python_binding(&self) -> String {
        let mut registrations = Vec::new();

        for (klass_name, klass) in self.structs.iter() {
            if !klass.ztruct.attributes.iter().any(|x| x == "no_binding")
                && klass.ztruct.visibility == Visibility::Public
            {
                registrations.push(format!("\t_m.add_class::<{}>()?;", klass_name));
            }
        }

        for func in &self.funcs {
            if is_to_bind(func) {
                registrations.push(format!(
                    "\t_m.add_wrapped(wrap_pyfunction!({}))?;",
                    func.name
                ));
            }
        }

        let mut mod_names = self.modules.keys().map(|x| x.as_str()).collect::<Vec<_>>();

        if self.module_name == "ensmallen" {
            mod_names.push("preprocessing");
            mod_names.push("models");
            mod_names.push("express_measures");

            registrations.push("\tenv_logger::init();".into());
        }

        for mods_name in mod_names {
            registrations.push(format!(
                "\tlet submod = PyModule::new(_py, \"{}\")?;",
                mods_name
            ));
            registrations.push(format!("\tregister_{}(_py, submod)?;", mods_name));
            registrations.push(format!("\t_m.add_submodule(submod)?;"));
        }

        format!(
            r#"
pub fn register_{module_name}(_py: Python, _m:&PyModule) -> PyResult<()> {{
    {registrations}
    Ok(())
}}

{functions}

{classes}

{modules}
"#,
            module_name = self.module_name,
            registrations = registrations.join("\n"),
            functions = format_vec!(
                self.funcs
                    .iter()
                    .filter(|func| is_to_bind(func))
                    .map(GenBinding::gen_python_binding)
                    .collect::<Vec<_>>(),
                "{}",
                "\n\n"
            ),
            classes = format_vec!(
                self.structs
                    .values()
                    .filter(|c| {
                        !c.ztruct.attributes.iter().any(|x| x == "no_binding")
                            && c.ztruct.visibility == Visibility::Public
                            && !c.is_empty()
                    })
                    .map(|c| {
                        println!(
                            "Generating struct: {}",
                            c.ztruct.struct_type.get_name().unwrap()
                        );
                        format!(
                            "#[pyclass(module=\"{module_name}\")]{}",
                            c.gen_python_binding(),
                            module_name = self.module_name
                        )
                    })
                    .collect::<Vec<_>>(),
                "{}",
                "\n\n"
            ),
            modules = format_vec!(
                self.modules
                    .values()
                    .map(GenBinding::gen_python_binding)
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>(),
                "{}",
                "\n\n"
            ),
        )
    }
}

impl Default for BindingsModule {
    fn default() -> Self {
        BindingsModule {
            module_name: String::new(),
            modules: BTreeMap::new(),
            funcs: Vec::new(),
            structs: BTreeMap::new(),
        }
    }
}

fn group_data(modules: Vec<Module>) -> BindingsModule {
    let mut bindings = BindingsModule::default();
    bindings.module_name = "ensmallen".to_string();

    let mut struct_modules_map = BTreeMap::new();

    // collect info about all the structs
    for module in &modules {
        for ztruct in &module.structs {
            let submodule = extract_module_name_from_struct(&ztruct);
            struct_modules_map.insert(ztruct.struct_type.to_string(), submodule.clone());
            bindings.get_submodule(submodule).push_class(ztruct.clone());
        }
    }

    // collect info about all the functions
    for module in &modules {
        for func in &module.functions {
            bindings
                .get_submodule(extract_module_name_from_func(func))
                .funcs
                .push(func.clone());
        }
    }

    // For each struct, collect all its implementaitons
    for module in &modules {
        for imp in &module.impls {
            if let Some(struct_name) = &imp.struct_name.get_name() {
                // find the correct submodule
                if let Some(struct_module) = struct_modules_map.get(struct_name) {
                    let struct_ref = bindings
                        .get_submodule(struct_module.clone())
                        .structs
                        .get_mut(&imp.struct_name.get_name().unwrap());

                    if let Some(struct_ref) = struct_ref {
                        // add it to the impls
                        struct_ref.impls.push(imp.clone());
                    } else {
                        println!(
                            "Skipping impl for '{}' at '{}'.",
                            imp.struct_name.get_name().unwrap(),
                            imp.file_path
                        );
                    }
                } else {
                    println!(
                        "Skipping impl for '{}' at '{}'.",
                        imp.struct_name.get_name().unwrap(),
                        imp.file_path
                    );
                }
            }
        }
    }

    bindings
}

pub fn gen_bindings(to_parse_path: &str, path: &str, init_path: &str) {
    print_sep();
    println!("Parsing the library source files");
    print_sep();
    let data = group_data(parse_crate(to_parse_path, DENY_LIST));

    print_sep();
    println!("Generating the bindings");
    print_sep();

    let file_content = format!(
        r#"
#[allow(unused_variables)]    
use super::*;
#[allow(unused_imports)]    
use pyo3::{{wrap_pyfunction, wrap_pymodule}};
use rayon::iter::*;
use std::hash::{{Hash, Hasher}};
use std::collections::hash_map::DefaultHasher;
use strsim::*; 
use pyo3::class::basic::CompareOp;

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
fn split_words(method_name: &str) -> Vec<String> {{
    method_name.split("_")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase())
        .collect()
}}

{}
"#,
        data.gen_python_binding()
    );

    fs::write(path, file_content).expect("Cannot write the automatically generated bindings file");

    /*
    let mut lines = vec![
        "\"\"\"Module offering fast graph processing and graph datasets.\"\"\"".into(),
    ];

    let mut elements = functions_modules.keys().cloned().collect::<Vec<_>>();
    elements.push("Graph".into());
    elements.push("preprocessing".into());

    for module in elements.iter() {
        lines.push(format!("from .ensmallen import {} # pylint: disable=import-error", module));
    }

    lines.push("from . import datasets".into());
    elements.push("datasets".into());

    // TODO: add datasets
    lines.push(format!(
        "__all__ = {:?}", elements
    ));

    fs::write(
        init_path,
        lines.join("\n"),
    )
    .expect("Cannot write the init file");
    */
}
