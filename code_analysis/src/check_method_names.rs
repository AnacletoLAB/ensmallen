use super::*;
use regex::Regex;
use lazy_static::lazy_static;

// the regexes inside the impl doc must match this regex
pub const IMPL_DOC_REGEX_REGEX_STR: &'static str = r#"\s*\*\s*`/(.+)/`\s*"#;
lazy_static! {
    static ref IMPL_DOC_REGEX_REGEX: Regex = Regex::new(IMPL_DOC_REGEX_REGEX_STR).unwrap();
}

// The method names must have the keywords at the end of the method, in the right order
pub const METHOD_KEYWORD_REGEX_STR: &'static str = r#".+(_weighted)?(_unchecked)?"#;
lazy_static! {
    static ref METHOD_KEYWORD_REGEX: Regex = Regex::new(METHOD_KEYWORD_REGEX_STR).unwrap();
}

impl Checker {
    fn extract_regexes_from_impl(&self, imp: &Impl) -> Vec<Regex> {
        let result = IMPL_DOC_REGEX_REGEX.captures_iter(&imp.doc)
            .map(|x| Regex::new(&x[1].to_string()))
            .collect::<Result<Vec<Regex>, regex::Error>>();

        match result {
            Ok(res) => res, 
            Err(error) => {
                self.log(Error::RegexSytnaxError{
                    source: imp.doc.clone(),
                    error_msg: format!("{:?}", error),
                });
                vec![]
            }
        }
    }
    

    fn check_method(&self, method: &Function, regexes: &Vec<Regex>){
        // the method has uncechked in the name iff the method is unsafe
        match (method.is_unsafe(), method.name.contains(&"unchecked")) {
            (true, false) => {
                self.log(
                    Error::MissingUnchecked{
                        method_name: method.name.clone(),
                    }
                );
            }
            (false, true) => {
                self.log(
                    Error::MissingUnsafe{
                        method_name: method.name.clone(),
                    }
                );
            }
            _ => {},
        };

        // Check that if there are regexes in the impl doc, all the methods should
        // match them.
        let does_name_match = regexes.iter()
            .any(|re| {
                re.is_match(&method.name)
            });

        // if regexes in the impl doc, the methods MUST follow the rule, can be disable with #[ignore_regex_name]
        if !regexes.is_empty() && !does_name_match && !method.attributes.iter().any(|x| x == "ignore_regex_name") {
            self.log(
                Error::MethodNameDoesNotMatchRegex{
                    method_name: method.name.clone(),
                    regexes: regexes.iter().map(|x| x.as_str().to_string()).collect::<Vec<_>>(),
                }
            );
        }

        // unchecked must be at the end of the method name
        if method.name.contains("unchecked") && !METHOD_KEYWORD_REGEX.is_match(&method.name) {
            self.log(
                Error::WrongKeywordPositionInMethodName{
                    method_name: method.name.clone(),
                    keyword: "unchecked".to_string(),
                }
            );
            return;
        }

        // weighted must be at the end of the method name
        if method.name.contains("weighted") && !METHOD_KEYWORD_REGEX.is_match(&method.name) {
            self.log(
                Error::WrongKeywordPositionInMethodName{
                    method_name: method.name.clone(),
                    keyword: "weighted".to_string(),
                }
            );
            return;
        }

        // if il metodo contiene unknown deve esistere anche known eccetto se con #[no_known]
        if method.name.contains("unknown") 
            && !self.method_names.contains(&method.name.replace("unknown", "known")) 
            && !method.attributes.iter().any(|x| x == "no_known")  {
            self.log(
                Error::MissingDualMethod{
                    method_name: method.name.clone(),
                    keyword: "unknown -> known".to_string(),
                }
            );
            return;
        }

        // if il metodo contiene known deve esistere anche unknown eccetto se con #[no_unknown]
        if method.name.contains("known") 
            && !self.method_names.contains(&method.name.replace("known", "unknown")) 
            && !method.attributes.iter().any(|x| x == "no_unknown")  {
            self.log(
                Error::MissingDualMethod{
                    method_name: method.name.clone(),
                    keyword: "known -> unknown".to_string(),
                }
            );
            return;
        }

    }

    pub fn check_names(&self) {
        for module in &self.modules {
            for imp in &module.impls {

                if imp.struct_name != "Graph" {
                    continue;
                }

                let regexes = self.extract_regexes_from_impl(&imp);
                println!("{:?}", regexes);

                for method in &imp.methods {

                    if method.visibility != Visibility::Public {
                        continue;
                    }

                    self.check_method(method, &regexes);
                }
            }
        }
    }
}