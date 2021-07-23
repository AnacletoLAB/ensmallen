
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    // MissingIntroduction{
    //     method_name: String,
    // },
    // MissplacedDocSection{
    //     method_name: String,
    //     section_name: String,
    // },
    NoDoc{
        method_name: String,
    },
    TypoInDoc{
        method_name: String,
        error_msg: String,
    },
    EmptyArgumentDescription{
        method_name: String,
        arg_name: String,
    },
    MissingExample {
        method_name: String,
    },
    MethodNotInExample {
        method_name: String,
    },
    EmptyRaisesLine {
        method_name: String,
    },
    EmptyIntroduction{
        method_name: String,
    },
    DuplicatedIntroduction{
        method_name: String,
    },
    MissingSection{
        method_name: String,
        section_name: String,
    },
    MissingArguments{
        method_name: String,
        arguments: Vec<String>,
    },
    ExtraArguments{
        method_name: String,
        arguments: Vec<String>,
    },
    NotParsableArgument{
        method_name: String,
        line: String,
    },
    WrongTypeArgument{
        method_name: String,
        truth_type: String,
        doc_type: String,
    },
    MissingUnsafe{
        method_name: String,
    },
    MissingUnchecked{
        method_name: String,
    },
    MethodNameDoesNotMatchRegex{
        method_name: String,
        regexes: Vec<String>,
    },
    RegexSytnaxError{
        source: String,
        error_msg: String,
    },
    WrongKeywordPositionInMethodName {
        method_name: String,
        keyword: String,
    },
    MissingDualMethod {
        method_name: String,
        keyword: String,
    },
}