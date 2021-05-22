extern crate proc_macro;
use proc_macro::TokenStream;

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
    no_inverse_method
);