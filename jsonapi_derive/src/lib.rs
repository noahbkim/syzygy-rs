extern crate proc_macro;

use proc_macro2;
use quote::quote;
use syn;

#[proc_macro_derive(JsonApiResource)]
pub fn jsonapi_resource_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    let output: proc_macro2::TokenStream = {
        let name = &input.ident;
        quote! {
            impl JsonApiResoure for #name {

            }
        }
    };
    proc_macro::TokenStream::from(output)
}
