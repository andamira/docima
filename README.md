# docima

Generate images at build time for embedding in the Rust documentation.

[![Crate](https://img.shields.io/crates/v/docima.svg)](https://crates.io/crates/docima)
[![API](https://docs.rs/docima/badge.svg)](https://docs.rs/docima/)
[![Lines Of Code](https://tokei.rs/b1/github/andamira/docima?category=code)](https://github.com/andamira/docima)

# How it works

1. First configure your build script to generate the images in the desired path.
   They will be encoded as `png` → `base64` and saved as `HTML` tags. E.g.:

```rust
generate_image(
    my_image_generator_function,
    600,
    400,
    "images/my_image.html",
    "the alt-text attribute",
    "the title attribute",
    "div",
)?;
```

2. Include your image in the Rust documentation by using the [doc][0] attribute
   along with the [include_str][1] macro.

```rust
#[doc = include_str!("../images/my_image.html") ]
```

[0]:https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
[1]:https://doc.rust-lang.org/std/macro.include_str.html

3. Generate the docs and enjoy:

```sh
cargo doc --open
```

You can refer to [the crate documentation](https://docs.rs/docima/) for more
details. And there's also a [full practical example][2] available.

[2]:https://github.com/joseluis/docima/tree/master/example


## Similar crates

- [embed-doc-image](https://crates.io/crates/embed-doc-image)
