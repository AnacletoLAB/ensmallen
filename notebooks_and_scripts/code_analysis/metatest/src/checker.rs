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

        // if il metodo contiene known deve esistere anche unknown eccetto se con #[no_unknown]
        // if il metodo contiene unknown deve esistere anche known eccetto se con #[no_known]

        // if il metodo contiene directed deve esistere anche undirected eccetto se con #[no_undirected]
        // if il metodo contiene undirected deve esistere anche directed eccetto se con #[no_directed]
        
        // unsafe
        // is a method is called uncheked it must be unsafe
        // it a method is not called unchecked it must not be unsafe

        // iters
        // if iter_X exits then par_iter_X must exists (can be disabled with #[no_par_iter])
        // se eiste iter_x deve esister get_x unless tag
        // se eiste par_iter_x deve esistere get_x (con priorita' rispetto ad iter_x) unless tag

        // nei bindings verbose deve essere sempre un Option<bool> perche'
        // vogliamo i default nel metodo rust

        // if no_binding come attr -> NO BINDING
        // if manual_binding come attr deve esistere il binding
        // if no attr -> deve esistere il binding
        // if deve essere generato il binding, parametri e ritorno devono
        // avere tipi "semplici"

        // se il tipo e' una tupla, ci deve essere la sezione Tuple Content che
        // descrive cosa ritorna, e.g. ShortestPathsResultBFS
        // la desc deve essere una lista che segue una regex e lunga quanto la tupla

        // non possono esserci righe di doc e commenti mischiate bcuz probabbly typo

        // check common parameters, se e' verbose allora il tipod eve essere bool
        // se e' random_state allora deve essere u64
        // verbose se presente deve essere l'ultimo argomento
        
        // if &mut self ci deve essere inplace nel nome.

    }

    pub fn display(&self){
        let errors = self.errors.lock().unwrap();
        println!("{:#4?}", &errors);
        println!("found {} errors", errors.len());
    }
}