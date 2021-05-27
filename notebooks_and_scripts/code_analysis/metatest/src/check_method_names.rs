use super::*;
use regex::Regex;

pub const METHOD_NAME_REGEXES_REGEX: &str = r" *\* `/([^\n]+)/`";

impl Checker {
    
    pub fn check_method_names(&self) {
        for module in &self.modules{
            for _impl in &module.impls {
                self.validate_impl(&_impl);
            }
        }
    }

    fn validate_impl(&self, imp: &Impl){
        println!("doc: {}", &imp.doc);
        let re = Regex::new(METHOD_NAME_REGEXES_REGEX).unwrap();
        let regexes = re.captures_iter(&imp.doc)
            .map(|x| x.get(1).unwrap().as_str().to_string())
            .collect::<Vec<String>>();
        println!("Found regexes: {:?}", regexes);

        let regexes = regexes.into_iter()
            .map(|x| Regex::new(&x).unwrap())
            .collect::<Vec<Regex>>();
        
        for method in &imp.methods {
            let method_name = method.name.clone();
            let at_least_one_match = regexes.iter().any(|re| {
                match re.captures(&method_name) {
                    None => false,
                    Some(captures) => {
                        let captures = captures.iter()
                            .map(|x| x.unwrap().as_str().to_string())
                            .skip(1)
                            .collect::<Vec<_>>();
                        println!("{:?}", captures);
                        true
                    }
                }
            });
            println!("{} {}", method_name, at_least_one_match);
        }


    }
}