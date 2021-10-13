extern crate proc_macro;
use proc_macro::{Delimiter, TokenStream, TokenTree};

macro_rules! create_tag {
    ($( $tag:ident)* ) => {
        $(
            #[proc_macro_attribute]
            pub fn $tag(_attr: TokenStream, item: TokenStream) -> TokenStream {
                item
            }
        )*
    };
}

// Here we create the tags we use for meta-analysis in the code.
// These do not change stuff but allows us to add a signature
// to methods without touching the documentation.
create_tag!(
    no_binding
    manual_binding
    no_inverse_method
    automatically_generated_function
    automatically_generated_binding
    no_numpy_binding
    fuzz_type
    ignore_regex_name
    no_unknown
    no_known
    module
);

// TODO! improve error messages
macro_rules! parse_comma {
    ($iter:expr) => {{
        let maybe_comma = $iter
            .next()
            .expect("Missing required comma in cached_property arguments!");
        match maybe_comma {
            TokenTree::Punct(punct) => {
                if punct != ',' {
                    panic!("Missing required comma in cached_property arguments!");
                }
            }
            _ => {
                panic!("Missing required comma in cached_property arguments!");
            }
        };
    }};
}

// TODO! improve error messages
macro_rules! parse_ident {
    ($iter:expr, $ident_name:literal) => {{
        let maybe_ident = $iter.next().expect(&format!(
            "Missing {} argument in cached_property!",
            $ident_name
        ));
        match maybe_ident {
            TokenTree::Ident(ident) => ident.to_string(),
            _ => {
                panic!(
                    "The {} argument of the cached_property macro must be an identifier",
                    $ident_name
                );
            }
        }
    }};
}

// TODO! improve error messages
macro_rules! parse_type {
    ($iter:expr, $ident_name:literal) => {{
        let mut name = parse_ident!($iter, $ident_name);
        match $iter.peek() {
            None => panic!("missign arguments after return type in cached_property macro"),
            Some(TokenTree::Punct(punct)) => {
                if punct.as_char() == '<' {
                    let mut counter = 0;
                    while let Some(token) = $iter.next() {
                        match token {
                            TokenTree::Punct(punct) => {
                                if punct.as_char() == '<' {
                                    counter += 1;
                                }
                                if punct.as_char() == '>' {
                                    counter -= 1;
                                }
                                name.push(punct.as_char());
                            }
                            x @ _ => {
                                name.push_str(&x.to_string());
                            }
                        }
                        name.push(' ');

                        if counter == 0 {
                            break;
                        }
                    }

                    if counter != 0 {
                        panic!("The angular brackets of the type are unbalanced!!.");
                    }
                }
            }
            _ => {}
        }
        name
    }};
}

// // TODO! improve error messages
// macro_rules! parse_literal {
//     ($iter:expr, $ident_name:literal) => {{
//         let maybe_ident = $iter.next().expect(&format!(
//             "Missing {} argument in cached_property!",
//             $ident_name
//         ));
//         match maybe_ident {
//             TokenTree::Literal(literal) => literal
//                 .to_string()
//                 .trim_start_matches("\"")
//                 .trim_end_matches("\"")
//                 .to_string(),
//             _ => {
//                 panic!(
//                     "The {} argument of the cached_property macro must be a literal",
//                     $ident_name
//                 );
//             }
//         }
//     }};
// }

// TODO! improve error messages
/// Parses values of type "self.struct.field"
/// This is not currently used anymore but it could be useful in the future
#[allow(unused_macros)]
macro_rules! parse_struct_field {
    ($iter:expr, $ident_name:literal) => {{
        let mut result = String::new();

        while let Some(maybe_ident) = $iter.next() {
            match maybe_ident {
                TokenTree::Ident(ident) => result.extend(ident.to_string().chars()),
                TokenTree::Punct(punct) => {
                    if punct != '.' {
                        panic!(
                            "Unexpected char '{}' found when parsing {}",
                            punct.to_string(),
                            $ident_name
                        );
                    }
                    result.push('.');
                }
                _ => {
                    panic!(
                        "The {} argument of the cached_property macro must be an identifier",
                        $ident_name
                    );
                }
            };
        }

        if result.is_empty() {
            panic!("Missing {} argument in cached_property!", $ident_name);
        }

        result
    }};
}

#[proc_macro]
/// Create a method that retrieves a value from the cache, if present, and otherwise
/// call the given method to fill the cache.
/// This is usually used when you have a method that efficiently computes several
/// values, and you want to have cached getters for each of them.
///
/// ```markdown
/// cached_property!(`method_name`, `return_type`, `function_to_call`, `where_the_value_is_cached`)
/// cached_property!(get_result1, u64, "documentation", compute, self.cache.result1)
/// ```
///
/// The cache should be a field called `cache` of the current struct.
/// The cache should be wrapped inside a UnsafeCell and it should be a struct
/// containing options.
///
/// # Example:
/// ```rust
/// use macros::*;
/// use std::cell::UnsafeCell;
///
/// struct PropertiesCache {
///     result1: Option<u64>,
///     result2: Option<Result<u64, String>>,
///     result3: Option<u64>,
/// }
///
/// struct Test{
///     cache: UnsafeCell<PropertiesCache>,
/// }
///
/// impl Test {
///     fn compute(&self) {
///         println!("Computing");
///         let mut cache = unsafe{&mut (*self.cache.get())};
///         cache.result1 = Some(1);
///         cache.result2 = Some(Ok(2));
///     }
///
///     cached_property!(get_result1, u64, compute, result1,
/// /// get the first result
/// /// this is a test method
///     );
///     cached_property!(get_result2, Result<u64, String>, compute, result2,
/// /// get the second result
/// /// this is a test method
/// );
///     
///     /// Value
///     #[cache_property(result3)]
///     pub fn get_result3(&self) -> u64 {
///         println!("calling get_result3");
///         31337
///     }
/// }
///
/// fn main() {
///     let mut t = Test{cache: UnsafeCell::new(PropertiesCache{result1: None, result2: None, result3: None})};
///
///     println!("{}", t.get_result1());
///     
///     println!("{}", t.get_result1());
///     println!("{:?}", t.get_result2());
///
///     println!("{}", t.get_result3());
///     
///     println!("{}", t.get_result3());
/// }
/// ```
pub fn cached_property(items: TokenStream) -> TokenStream {
    let mut iter = items.into_iter().peekable();

    let method_name = parse_ident!(iter, "method_name");
    parse_comma!(iter);
    let return_type = parse_type!(iter, "return_type");
    parse_comma!(iter);
    let function_to_call = parse_ident!(iter, "function_to_call");
    parse_comma!(iter);
    let where_the_value_is_cached = parse_ident!(iter, "where_the_value_is_cached");
    parse_comma!(iter);
    let doc = iter.map(|x| x.to_string()).collect::<Vec<_>>().join("\n");

    format!(
        r#"
 {doc}
 ///
 /// ## Caching details
 /// This method is automatically generated using the `cached_property!` macro
 /// which on first call will execute the method `{function_to_call}` and then
 /// it will get the result from the cache at position `{where_the_value_is_cached}`.
 pub {is_unsafe} fn {method_name}(&self) -> {return_type} {{ 
     
     let maybe_result = unsafe{{ (*self.cache.get()).{where_the_value_is_cached}.clone() }};

     match maybe_result {{
         None => {{
             self.{function_to_call}();
             unsafe{{ (*self.cache.get()).{where_the_value_is_cached}.clone() }}.unwrap()
         }},
         Some(v) => v,
     }}
 }}"#,
        doc = doc,
        is_unsafe = match method_name.contains("unchecked") {
            true => "unsafe",
            false => "",
        },
        method_name = method_name,
        return_type = return_type,
        function_to_call = function_to_call,
        where_the_value_is_cached = where_the_value_is_cached,
    )
    .parse()
    .unwrap()
}

#[proc_macro_attribute]
/// Automatically cache the result of a function.
/// The cache should be a field called `cache` of the current struct.
/// The cache should be wrapped inside a UnsafeCell and it should be a struct
/// containing options.
///
/// # Example:
/// ```rust
/// use macros::*;
/// use std::cell::UnsafeCell;
///
/// struct PropertiesCache {
///     result1: Option<u64>,
///     result2: Option<Result<u64, String>>,
///     result3: Option<u64>,
/// }
///
/// struct Test{
///     cache: UnsafeCell<PropertiesCache>,
/// }
///
/// impl Test {
///     fn compute(&self) {
///         println!("Computing");
///         let mut cache = unsafe{&mut (*self.cache.get())};
///         cache.result1 = Some(1);
///         cache.result2 = Some(Ok(2));
///     }
///
///     cached_property!(get_result1, u64, compute, result1,
/// /// get the first result
/// /// this is a test method
///     );
///     cached_property!(get_result2, Result<u64, String>, compute, result2,
/// /// get the second result
/// /// this is a test method
/// );
///     
///     /// Value
///     #[cache_property(result3)]
///     pub fn get_result3(&self) -> u64 {
///         println!("calling get_result3");
///         31337
///     }
/// }
///
/// fn main() {
///     let mut t = Test{cache: UnsafeCell::new(PropertiesCache{result1: None, result2: None, result3: None})};
///
///     println!("{}", t.get_result1());
///     
///     println!("{}", t.get_result1());
///     println!("{:?}", t.get_result2());
///
///     println!("{}", t.get_result3());
///     
///     println!("{}", t.get_result3());
/// }
/// ```
pub fn cache_property(attr: TokenStream, items: TokenStream) -> TokenStream {
    let where_the_value_is_cached = attr.to_string();
    let mut iter = items.into_iter().peekable();

    let mut prologue = String::new();
    let mut outer = String::new();
    loop {
        match iter.next().unwrap() {
            TokenTree::Ident(ident) => {
                if ident.to_string() == "pub" || ident.to_string() == "fn" {
                    outer.push_str(&ident.to_string());
                    outer.push(' ');
                    break;
                }
                prologue.push_str(&ident.to_string());
                prologue.push(' ');
            }
            x @ _ => {
                prologue.push_str(&x.to_string());
                prologue.push(' ');
            }
        }
    }

    let inner_function = loop {
        match iter.next().unwrap() {
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    break group.stream();
                }
                outer.push_str(&group.to_string());
                outer.push(' ');
            }
            x @ _ => {
                outer.push_str(&x.to_string());
                outer.push(' ');
            }
        }
    };

    format!(
        r#"
        {prologue}
        ///
        /// ## Caching details
        /// This method was wrapped by the macro `cache_property` which 
        /// caches the result of the function by saving it on first call in the
        /// attribute `{where_the_value_is_cached}`.
        {outer} {{ 
            let maybe_result =  unsafe{{(*self.cache.get()).{where_the_value_is_cached}.clone()}};

            match maybe_result {{
                Some(res) => res,
                None => {{
                    let result = {{
                        {inner}
                    }};
                    unsafe{{(*self.cache.get()).{where_the_value_is_cached} = Some(result)}};
                    unsafe{{(*self.cache.get()).{where_the_value_is_cached}.clone().unwrap()}}
                }}
            }}
        }}"#,
        prologue = prologue,
        outer = outer.to_string().replace("- >", "->"),
        inner = inner_function,
        where_the_value_is_cached = where_the_value_is_cached,
    )
    .parse()
    .unwrap()
}
