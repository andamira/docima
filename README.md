# docima

Generate images at build time for embedding in the Rust documentation.

[![Crate](https://img.shields.io/crates/v/docima.svg)](https://crates.io/crates/docima)
[![API](https://docs.rs/docima/badge.svg)](https://docs.rs/docima/)
[![Lines Of Code](https://tokei.rs/b1/github/andamira/docima?category=code)](https://github.com/andamira/docima)

## Warning

The current API is going to evolve a lot while the major version number is 0.

## Usage

1. First configure your build script to generate the images in the desired path.
   They will be encoded as `png` → `base64` and saved as `HTML` tags. E.g.:

```rust
ImageFile::new()
    .path("images/my_image.html")
    .width(600)
    .height(400)
    .title("My image")
    .id("image-01")
    .style("display: block; margin:auto;")
    .wrapper("div")
    .wrapper_style("background-color:red; padding:3px;")
    .overwrite(true)
    .generate(my_image_generator_function)?;
```

2. Include your image in the Rust documentation by using the [doc][0] attribute
   and the [include_str][1] macro.

```rust
#[doc = include_str!("../images/my_image.html") ]
```
[0]:https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
[1]:https://doc.rust-lang.org/std/macro.include_str.html

3. Generate the docs:

```sh
cargo doc --open
```

# Learning

You can refer to the crate [documentation][2] and the [source code][3] for more
complete information and practical examples.

[2]:https://docs.rs/docima/
[3]:https://github.com/andamira/docima/blob/main/build.rs

## Similar crates

- [embed-doc-image](https://crates.io/crates/embed-doc-image)
