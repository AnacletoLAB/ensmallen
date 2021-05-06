use super::*;
#[derive(Debug, Clone)]
pub struct Module {
    pub doc: String,
    pub file: String,
    pub name: String,
    pub uses: Vec<Use>,
    pub structs: Vec<Struct>,
    pub types: Vec<TypeDefinition>,
    pub consts: Vec<Const>,
    pub statics: Vec<Static>,
    pub impls: Vec<Impl>,
    pub functions: Vec<Function>,
    pub externs: Vec<Extern>,
    pub mods: Vec<Module>,
}

impl Default for Module {
    fn default() -> Self {
        Module{
            doc: String::new(),
            file: String::new(),
            name: String::new(),
            uses: Vec::new(),
            types: Vec::new(),
            structs: Vec::new(),
            consts: Vec::new(),
            statics: Vec::new(),
            impls: Vec::new(),
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

            if Use::can_parse(data) {
                result.uses.push(parse!(data, Use));
                continue;
            }
            if Function::can_parse(data) {
                let mut func = parse!(data, Function);
                func.doc = doc;
                doc = String::new();
                func.attributes = attrs;
                attrs = Vec::new();
                result.functions.push(func);
                continue;
            }
            if Struct::can_parse(data) {
                let mut struc = parse!(data, Struct);
                struc.doc = doc;
                doc = String::new();
                struc.attributes = attrs;
                attrs = Vec::new();
                result.structs.push(struc);
                continue;
            }
            if Const::can_parse(data) {
                let mut current_const = parse!(data, Const);
                current_const.doc = doc;
                doc = String::new();
                current_const.attributes = attrs;
                attrs = Vec::new();
                result.consts.push(current_const);
                continue;
            }
            if Static::can_parse(data) {
                let mut current_stat = parse!(data, Static);
                current_stat.doc = doc;
                doc = String::new();
                current_stat.attributes = attrs;
                attrs = Vec::new();
                result.statics.push(current_stat);
                continue;
            }
            if TypeDefinition::can_parse(data) {
                let mut typedef = parse!(data, TypeDefinition);
                typedef.doc = doc;
                doc = String::new();
                typedef.attributes = attrs;
                attrs = Vec::new();
                result.types.push(typedef);
                continue;
            }
            if Impl::can_parse(data) {
                let mut current_impl = parse!(data, Impl);
                current_impl.doc = doc;
                doc = String::new();
                current_impl.attributes = attrs;
                attrs = Vec::new();
                result.impls.push(current_impl);
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
            if Attribute::can_parse(data){
                let attr: String = parse!(data, Attribute).into();
                attrs.push(attr);
                continue;
            }
            if data.starts_with(b"//") {
                let (inner, _comment) = split_at(data, b'\n');
                data = inner;
                continue;
            }

            panic!("Cannot parse the following module line: {}", &String::from_utf8(data.to_vec()).unwrap()[..50]);
        }

        result.doc = module_doc;

        (data, result)
    }
}