use super::*;
use std::fs;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Checker{
    pub(crate) modules: Vec<Module>,

    /// This doesn't requires an actual mutex because rust doesn't currently 
    /// support partial move yet, we need this because in all the checks we
    /// will read the modules and write the errors.
    pub(crate) errors: Mutex<Vec<Error>>,
}

impl Checker {
    pub fn new(modules: Vec<Module>) -> Checker {
        Checker{
            modules,
            errors: Mutex::new(Vec::new()),
        }
    }

    pub fn parse_files(files: Vec<String>) -> Checker {
        let mut modules = Vec::new();

        for path in files{
            // read the file
            let contents = fs::read_to_string(path).expect("File not found");
            // parse the file
            let (_reminder, module) = Module::parse(contents.as_bytes());
            modules.push(module);
        }

        Checker::new(modules)
    }    

    pub(crate) fn log(&self, error: Error){
        self.errors.lock().unwrap().push(error);
    }

    pub fn check(&mut self) {
        self.check_doc();

        // if regexes in the impl doc, the methods MUST follow the rule, can be disable with #[ignore_regex_name]

        // getters
        // check common typos like edge -> egde whether -> wether, wheter, weather
        // the capture groups in the methods name regexes must be in the allowed terms list.
        // if get_X_from_Y exists then get_Y_from_X must exists (can be disable with #[no_inverse_method])
        // if get_X_from_Y exists and get_Y_from_Z then get_X_from_Z must exist
        // if il metodo contiene _weighted_ deve esistere anche _unweighted_ eccetto se con #[no_unweighted]
        // if il metodo contiene _unweighted_ deve esistere anche _weighted_ eccetto se con #[no_weighted]
        
        // unsafe
        // is a method is called uncheked it must be unsafe
        // it a method is not called unchecked it must not be unsafe

        // iters
        // if iter_X exits then par_iter_X must exists (can be disabled with #[no_par_iter])

        // nei bindings verbose deve essere sempre un Option<bool> perche'
        // vogliamo i default nel metodo rust
    }

    pub fn display(&self){
        let errors = self.errors.lock().unwrap();
        println!("{:#4?}", &errors);
        println!("found {} errors", errors.len());
    }
}