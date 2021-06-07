mod generate;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

use generate::VectorizedStructName;

#[proc_macro_attribute]
pub fn vectorize(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let vec_struct_name = parse_macro_input!(args as VectorizedStructName);
    generate::expand_vectorize(vec_struct_name, parsed_input)
}
