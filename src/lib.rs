extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro_derive(VariantCount)]
pub fn variant_count(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let variants = match &ast.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("VariantCount can only be derived for enums"),
    };
    let count = variants.len();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics #ty #ty_generics
            #where_clause
        {
            #vis const VARIANT_COUNT: usize = #count;
        }
    };
    tokens.into()
}