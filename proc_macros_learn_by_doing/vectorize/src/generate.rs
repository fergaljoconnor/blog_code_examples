use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

pub(crate) struct VectorizedStructName {
    name: Ident,
}

impl Parse for VectorizedStructName {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        Ok(VectorizedStructName { name: name })
    }
}

pub(crate) fn expand_vectorize(vec_name: VectorizedStructName, input: DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let vec_name = vec_name.name;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let get_field_name = || fields.iter().map(|field| &field.ident);
    let (field_name, push_field_name, pull_field_name, constructor_field, getter_return_field) = (
        get_field_name(),
        get_field_name(),
        get_field_name(),
        get_field_name(),
        get_field_name(),
    );

    let field_type = fields.iter().map(|field| &field.ty);
    let getter_return_type = field_type.clone();

    let getter_name = fields.iter().map(|field| {
        Ident::new(
            format!(
                "get_{}",
                field
                    .ident
                    .clone()
                    .expect("Struct contains a field without an ident")
                    .to_string()
            )
            .as_str(),
            Span::call_site(),
        )
    });

    TokenStream::from(quote!(

        #input

        #[derive(Clone, Debug)]
        struct #vec_name {
            #(
                #field_name: Vec<#field_type>
            )*
        }

        impl #vec_name {
            fn new() -> Self {
                Self {
                    #(
                        #constructor_field: Vec::new()
                    )*
                }
            }

            fn push(&mut self, instance: #type_name) {
                #(
                    self.#push_field_name.push(instance.#pull_field_name)
                )*
            }

            #(
              fn #getter_name(&self) -> &Vec<#getter_return_type> {
                  &self.#getter_return_field
              }
            )*
        }


    ))
}
