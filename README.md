# Enum variant count

[![Crates.io](https://img.shields.io/crates/v/variant_count.svg)](https://crates.io/crates/variant_count)
[![Docs](https://docs.rs/variant_count/badge.svg)](https://docs.rs/variant_count)

This crate provides the `VariantCount` derive macro, which adds to an enum a `VARIANT_COUNT` constant holding the number of its variants.

The `VariantCount` usage example:

```rust
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
variant_count = "1.2"
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE)
  or [apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.
