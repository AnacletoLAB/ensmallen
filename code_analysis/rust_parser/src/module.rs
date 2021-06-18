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
            functions: Vec::new(),
            externs: Vec::new(),
            mods: Vec::new(),
        }
    }
}

/// standard parsing of statements
macro_rules! maybe_parse {
    ($data:expr, $doc:ident, $attrs:expr, $type:ty, $res_vector:expr) => {
        if <$type>::can_parse($data) {
            let (tmp_data, mut val) = <$type>::parse(skip_whitespace($data));
            $data = skip_whitespace(tmp_data);
            val.doc = $doc;
            $doc = String::new();
            val.attributes = $attrs;
            $attrs = Vec::new();
            $res_vector.push(val);
            continue;
        }
    };
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
            maybe_parse!(data, doc, attrs, Macro,           result.macros);

            panic!("Cannot parse the following module line: {}", &String::from_utf8(data.to_vec()).unwrap()[..50]);
        }

        result.doc = module_doc;

        (data, result)
    }
}