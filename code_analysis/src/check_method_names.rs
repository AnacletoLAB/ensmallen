use super::*;



impl Checker {
    fn check_method(&self, method: &Function, regexes: &[&str]){
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


    }

    pub fn check_names(&self) {
        for module in &self.modules {
            for imp in &module.impls {

                if imp.struct_name != "Graph" {
                    continue;
                }

                let regexes = vec![]; // TODO extract the regexs from the documentation

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