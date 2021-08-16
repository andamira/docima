# docima

Generate images at build time for embedding in the Rust documentation.

[![Crate](https://img.shields.io/crates/v/docima.svg)](https://crates.io/crates/docima)
[![API](https://docs.rs/docima/badge.svg)](https://docs.rs/docima/)
[![Lines Of Code](https://tokei.rs/b1/github/andamira/docima?category=code)](https://github.com/andamira/docima)

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

You can refer to [the crate documentation](https://docs.rs/docima/) for more
details. And there's also a [full practical example][2] available.

[2]:https://github.com/andamira/docima/tree/master/example


## Similar crates

- [embed-doc-image](https://crates.io/crates/embed-doc-image)
