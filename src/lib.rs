/*!
This crate provides `VariantCount` derive macro for enum which adds to it the `VARIANT_COUNT` constant,
containing count of enum variants.

## Example

```rust
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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(VariantCount)]
pub fn variant_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let tokens = variant_count_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}

fn variant_count_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let (vis, ty, generics) = (&ast.vis, &ast.ident, &ast.generics);
    let variants = match &ast.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports enums.")),
    };
    let count = variants.len();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #ty #ty_generics
            #where_clause
        {
            #vis const VARIANT_COUNT: usize = #count;
        }
    })
}
