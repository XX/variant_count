/*!
This crate provides `VariantCount` derive macro for enum which adds to it the `VARIANT_COUNT` constant, containing count of enum variants.

## Example

```rust
extern crate variant_count;

use variant_count::VariantCount;

#[derive(VariantCount)]
enum Test {
    First(i32),
    Second(Option<String>),
    Third,
}

assert_eq!(Test::VARIANT_COUNT, 3);
```

## Usage

If you're using Cargo, just add it to your Cargo.toml:

```toml
[dependencies]
variant_count = "1.1"
```

Use `VariantCount` in the `derive` enum attribute.
!*/

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