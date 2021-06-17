use super::*;
use regex::Regex;


pub const IMPL_DOC_REGEX_REGEX: &'static str = r#"\s*\*\s*`/(.+)/`\s*"#;

impl Checker {
    fn extract_regexes_from_impl(&self, imp: &Impl) -> Vec<Regex> {
        let re = Regex::new(IMPL_DOC_REGEX_REGEX).unwrap();
        let result = re.captures_iter(&imp.doc)
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

        if !regexes.is_empty() && !does_name_match {
            self.log(
                Error::MethodNameDoesNotMatchRegex{
                    method_name: method.name.clone(),
                    regexes: regexes.iter().map(|x| x.as_str().to_string()).collect::<Vec<_>>(),
                }
            )
        }

        // unchecked must be at the end of the method name
        if method.name.contains("unchecked") && !method.name.ends_with("unchecked") {
            self.log(
                Error::
            )
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