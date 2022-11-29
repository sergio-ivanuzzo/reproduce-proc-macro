extern crate proc_macro;

use proc_macro::{TokenStream};
use std::io::{Error, ErrorKind};
use std::io::ErrorKind::Other;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, ItemStruct, Token, punctuated::Punctuated, NestedMeta};

#[proc_macro_derive(Debugger, attributes(dynamic_field))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);

    let field_data = fields.iter().map(|f| {
        let ident = f.ident.clone().unwrap().to_string();
        println!("IDENT: {:?}", ident);
        let field_type = f.ty.clone();
        let args = f.attrs
            .iter()
            .flat_map(|a| {
                match a.parse_meta().unwrap() {
                    syn::Meta::Path(meta) => {
                        Some(meta.get_ident().unwrap().to_string())
                    },
                    syn::Meta::List(meta) => {
                        Some(meta.path.get_ident().unwrap().to_string())
                    },
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        Item {
            name: f.ident.clone().unwrap().to_string(),
            item_type: quote!(#field_type).to_string(),
            attrs: args,
        }
    }).collect::<Vec<_>>();

    let output = quote! {
        impl #ident {
            pub fn debug() {
                println!("{:#?}",  vec![#(#field_data),*]);
            }
        }
    };

    TokenStream::from(output)
}

#[derive(Debug)]
struct Item {
    name: String,
    item_type: String,
    attrs: Vec<String>,
}

impl quote::ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
