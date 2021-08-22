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
    .attr("title", "My image")
    .attr("id", "image-01")
    .attr("style", "display: block; margin:auto;")
    .wrapper("div")
    .wrapper_attr("style", "background-color:red; padding:3px;")
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

## Features

There are several features that allows to customize the defaults.

### `not_default_overwrite`

By default all images will always be generated even when the target file already
exists. By using this feature you can avoid generation in case the file exists.

Note that you can always override the default by using the `overwrite` method.

### `build_when_doc`

By enabling this feature images will only be generated when the following `doc`
feature is also used.

This can be very useful in case you need to always overwrite the generated files
but don't want to generate the images when normally compiling the source code.

### `doc`

Use this feature manually, in conjunction to `build_when_doc`.

You'll have to customize your setup using the the following example as a model:

Add to `Cargo.toml`:
```toml
[features]
doc = ["docima/doc"]

[build-dependencies]
docima = { version = "*", features = ["build_when_doc"] }
```

Then the images will *only* be generated when you use this feature, like this:
```sh
doc --features=doc
```

For greater convenience you can create a new alias in `.cargo/config.toml`:
```toml
[alias]
docdoc = "doc --features=doc"
```

# More info

You can refer to the crate [documentation][2] and the [source code][3] for more
complete information and practical examples.

[2]:https://docs.rs/docima/
[3]:https://github.com/andamira/docima/blob/main/build.rs

## Similar crates

- [embed-doc-image](https://crates.io/crates/embed-doc-image)
