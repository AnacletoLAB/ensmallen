use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub doc: String,
    pub file: String,
    pub name: String,
    pub uses: Vec<Use>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub types: Vec<TypeDefinition>,
    pub traits: Vec<TraitDefinition>,
    pub consts: Vec<Const>,
    pub statics: Vec<Static>,
    pub impls: Vec<Impl>,
    pub macros: Vec<Macro>,
    pub macro_calls: Vec<MacroCall>,
    pub functions: Vec<Function>,
    pub externs: Vec<Extern>,
    pub mods: Vec<Module>,
}

impl Module {
    pub fn get_function_names(&self) -> Vec<String> {
        let mut result = Vec::new();

        result.extend(
            self.functions.iter().map(|x| x.name.clone())
        );

        for imp in &self.impls {
            result.extend(
                imp.methods.iter().map(|x| x.name.clone())
            );
        }

        result
    }
}

impl Default for Module {
    fn default() -> Self {
        Module{
            doc: String::new(),
            file: String::new(),
            name: String::new(),
            uses: Vec::new(),
            enums: Vec::new(),
            types: Vec::new(),
            traits: Vec::new(),
            structs: Vec::new(),
            consts: Vec::new(),
            statics: Vec::new(),
            impls: Vec::new(),
            macros: Vec::new(),
            macro_calls: Vec::new(),
            functions: Vec::new(),
            externs: Vec::new(),
            mods: Vec::new(),
        }
    }
}

impl Parse for Module {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Module::default();
        let mut module_doc = String::new();

        let mut doc = String::new();
        let mut attrs = Vec::new();

        loop {
            data = skip_whitespace(data);

            if data.is_empty(){
                break;
            }

            if Attribute::can_parse(data){
                let attr = parse!(data, Attribute);
                attrs.push(attr);
                continue;
            }
            if DocLine::can_parse(data) {
                let doc_line: String = parse!(data, DocLine).into();
                doc.push_str(doc_line.as_str());
                doc.push('\n');
                continue;
            }
            if ModuleDocLine::can_parse(data) {
                let doc_line: String = parse!(data, ModuleDocLine).into();
                module_doc.push_str(doc_line.as_str());
                module_doc.push('\n');
                continue;
            }
            if data.starts_with(b"//") {
                let (inner, _comment) = split_at(data, b'\n');
                data = inner;
                continue;
            }

            maybe_parse!(data, doc, attrs, Macro,           result.macros);
            maybe_parse!(data, doc, attrs, MacroCall,       result.macro_calls);
            maybe_parse!(data, doc, attrs, Function,        result.functions);
            maybe_parse!(data, doc, attrs, Struct,          result.structs);
            maybe_parse!(data, doc, attrs, Enum,            result.enums);
            maybe_parse!(data, doc, attrs, Const,           result.consts);
            maybe_parse!(data, doc, attrs, Static,          result.statics);
            maybe_parse!(data, doc, attrs, TypeDefinition,  result.types);
            maybe_parse!(data, doc, attrs, TraitDefinition, result.traits);
            maybe_parse!(data, doc, attrs, Impl,            result.impls);
            maybe_parse!(data, doc, attrs, Extern,          result.externs);
            maybe_parse!(data, doc, attrs, Use,             result.uses);

            panic!("Cannot parse the following module line: {}", &String::from_utf8(data.to_vec()).unwrap()[..50]);
        }

        result.doc = module_doc;

        (data, result)
    }
}