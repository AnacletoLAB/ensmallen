use super::*;

use std::collections::HashSet;

const INCLUDE_EXAMPLE: bool = false;

macro_rules! find_section {
    ($sections:expr, $sec:pat) => {
        $sections.iter().find(|sec| match sec {
            $sec => true,
            _ => false,
        })
    };
}

impl Checker {
    /// checks that the docs are sane.
    /// 
    /// It enforces the following rules:
    /// * all methods must have doc
    /// * if return type contiene result -> Raises section in doc
    /// * all the introdutions MUST be uniques
    /// * the arguments must be documented and their types must match the func declaration
    pub fn check_doc(&self) {

        let mut introductions = HashSet::new();

        for module in &self.modules {
            for impls in &module.impls {
                let struct_name = String::from(impls.struct_name.clone());
                if struct_name == "Graph" {
                    for method in &impls.methods {
                        if method.visibility != Visibility::Public {
                            continue;
                        }

                        let method_name = format!("{}::{}", struct_name, method.name);

                        // all methods must have doc
                        if method.doc == "" {
                            self.log(Error::NoDoc{method_name});
                            continue;
                        }

                        // parse the documentation into sections
                        let (_, doc) = Doc::parse(method.doc.as_bytes());
                        let sections = doc.sections;

                        // check existance
                        if find_section!(sections, DocSection::Introduction(_)).is_none() {
                            self.log(Error::MissingSection{
                                method_name: method_name.clone(),
                                section_name: "Introduction".to_string()
                            });
                        }
                        if method.has_arguments() && find_section!(sections, DocSection::Arguments{..}).is_none() {
                            self.log(Error::MissingSection{
                                method_name: method_name.clone(),
                                section_name: "Arguments".to_string()
                            });
                        }
                        if INCLUDE_EXAMPLE && find_section!(sections, DocSection::Example{..}).is_none() {
                            self.log(Error::MissingSection{
                                method_name: method_name.clone(),
                                section_name: "Example".to_string()
                            });
                        }
                        if method.is_unsafe() && find_section!(sections, DocSection::Unsafe{..}).is_none() {
                            self.log(Error::MissingSection{
                                method_name: method_name.clone(),
                                section_name: "Safety".to_string()
                            });
                        }
                        if method.returns_result() && find_section!(sections, DocSection::Raises{..}).is_none() {
                            self.log(Error::MissingSection{
                                method_name: method_name.clone(),
                                section_name: "Raises".to_string()
                            });
                        }

                        for sec in sections {
                            match sec {
                                DocSection::Introduction(intro) => {
                                    // check that the introduction is not empty
                                    if intro.is_empty() {
                                        self.log(Error::EmptyIntroduction{
                                            method_name: method_name.clone()
                                        })
                                    }
                                    // check that the introduction is unique
                                    if !introductions.insert(intro.clone()) {
                                        self.log(Error::DuplicatedIntroduction{
                                            method_name: method_name.clone()
                                        });
                                    }

                                    if intro.starts_with("Return true if") 
                                        || intro.starts_with("Return false if") {
                                            self.log(Error::TypoInDoc{
                                                method_name: method_name.clone(),
                                                error_msg: "Use return whether".to_string()
                                            })
                                        }
                                }
                                DocSection::Example{
                                    code,
                                    ..
                                } => {
                                    // check that the code example is not empty
                                    if code.is_empty() {
                                        self.log(Error::MissingExample{
                                            method_name: method_name.clone()
                                        });
                                    } else {
                                        // check that the method is *actually*
                                        // called
                                        if !code.contains(&method.name) {
                                            self.log(Error::MethodNotInExample{
                                                method_name: method_name.clone(),
                                            });
                                        }
                                    }
                                }
                                DocSection::Raises{
                                    exceptions,
                                    ..
                                } => {
                                    // check that all the lines are not empty
                                    for excep in exceptions {
                                        if excep.is_empty() {
                                            self.log(Error::EmptyRaisesLine{
                                                method_name: method_name.clone(),
                                            })
                                        }
                                    }
                                }
                                DocSection::Arguments{
                                    arguments: args,
                                    ..
                                } => {
                                    // Check that all the args are parsable
                                    let mut parsed_args = Vec::new();
                                    for arg in &args {
                                        match arg {
                                            Argument::Parsable(arg) => {
                                                parsed_args.push(arg);
                                            },
                                            Argument::NotParsable(line) => {
                                                self.log(Error::NotParsableArgument{
                                                    method_name: method_name.clone(),
                                                    line: line.clone(),
                                                });
                                            }
                                        }
                                    }
                                    /// Get the method arguments excluding self
                                    let method_args: Vec<_> = method.args.0.iter().filter_map(|x|
                                        match x.arg_type {
                                            Type::SelfType => None,
                                            _ => Some(x),
                                        }
                                    ).collect();

                                    // check that all the args are present and no extra arg is present
                                    let doc_names: HashSet<String> = parsed_args.iter().map(|x|
                                        x.name.clone()
                                    ).collect();
                                    let arg_names: HashSet<String> = method_args.iter().map(|x|
                                        x.name.clone()
                                    ).collect();

                                    let missing_args: Vec<String> = doc_names.difference(&arg_names).cloned().collect();
                                    let extra_args: Vec<String> = arg_names.difference(&doc_names).cloned().collect();
                                    let common_args: Vec<String> = arg_names.intersection(&doc_names).cloned().collect();
                                    // check that all arguments are present
                                    if !missing_args.is_empty() {
                                        self.log(Error::MissingArguments{
                                            method_name: method_name.clone(),
                                            arguments: missing_args,
                                        });
                                    }
                                    // check that there are no extra args
                                    if !extra_args.is_empty() {
                                        self.log(Error::ExtraArguments{
                                            method_name: method_name.clone(),
                                            arguments: extra_args,
                                        });
                                    }

                                    // thanks to the previous checks we are ensured
                                    // that args and doc_args have the same length.
                                    for arg_name in common_args {
                                        let documented = parsed_args.iter().find(|x| {
                                            x.name == arg_name
                                        }).unwrap();
                                        let truth = method_args.iter().find(|x| {
                                            x.name == arg_name
                                        }).unwrap();

                                        if String::from(truth.arg_type.clone()) != documented.arg_type {
                                            self.log(Error::WrongTypeArgument{
                                                method_name: method_name.clone(),
                                                truth_type: String::from(truth.arg_type.clone()),
                                                doc_type: documented.arg_type.clone(),
                                            })
                                        }

                                        if documented.description.is_empty() {
                                            self.log(Error::EmptyArgumentDescription{
                                                method_name: method_name.clone(),
                                                arg_name: documented.name.clone(),
                                            })
                                        }
                                    }   
                                }
                                // no need to check custom sections
                                DocSection::CustomSection{..} => {},
                                DocSection::References{..} => {},
                                DocSection::Unsafe{..} => {},
                            }
                        }
                        

                        
                    }
                }
            }
        }
    }
}