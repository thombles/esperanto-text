esperanto-text
==============

Convert Esperanto text between UTF-8, x-system and h-system transliterations.

True Esperanto text has various diacritics that can be properly represented in
UTF-8. Those who are limited to ASCII or are unable to type these characters
often resort to the "h-system" or "x-system". In these, a suffix is added to
those letters which should have a diacritic.

This crate provides functions for converting a string between a transliteration
and UTF-8. For the x-system this can be done with complete accuracy as there is
no ambiguity. For the h-system, a small vocabulary list is used to avoid
changing the meaning of real words.

A binary called `eotext` is included to use these functions from a CLI.

### Usage

To use the published crate in your Rust program add the following to
`Cargo.toml`:

```
esperanto-text = "1"
```

If you have cloned the repo and want to run the `eotext` program you must have
a Rust toolchain installed. The recommended way to do is is via
[Rustup](https://rustup.rs/). Run the following command:

```
cargo build --release --bin eotext
```

The built binary will be located at `target/release/eotext`.

### Example: UTF-8 to x-system

```rust
let input = "eĥoŝanĝo ĉiuĵaŭde";
assert_eq!(
    esperanto_text::utf8_to_x_system(input),
    "ehxosxangxo cxiujxauxde".to_owned(),
);
```

### Example: h-system to UTF-8

```rust
let input = "Chiuj estas senchavaj kaj taugaj ideoj.";
assert_eq!(
    esperanto_text::h_system_to_utf8(input),
    "Ĉiuj estas senchavaj kaj taŭgaj ideoj.".to_owned(),
);
```

### Licence

Made available under the MIT licence. See `LICENCE` for details.
