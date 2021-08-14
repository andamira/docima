//! # docima
//!
//! The functions are intended to be called from the build script
//!
//! ## Usage
//!
//! 1. Configure your build script to generate the images in the desired path.
//!
//! ```ignore
//!     generate_image(
//!        my_image_generator_function,
//!        600,
//!        400,
//!        "images/image01.html",
//!        "the alt-text attribute",
//!        "the title attribute",
//!        "div",
//!     )?;
//! ```
//! 
//! 2. Include your image in the docs by using the [doc][0] attribute and
//!    the [include_str][1] macro.
//!
//! ```ignore
//! #[doc = include_str!("../images/my_image.html") ]
//! ```
//! [0]:https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
//! [1]:https://doc.rust-lang.org/std/macro.include_str.html
//!
//! 3. Generate the docs:
//! ```sh
//! cargo doc --open
//! ```
//!
//! See also the full
//! [provided example](https://github.com/andamira/docima/tree/master/example).

use data_encoding::BASE64_MIME;
use image::{
    codecs::png::{CompressionType, FilterType, PngEncoder},
    ColorType,
};
use std::{
    fs::{create_dir_all, File},
    io::{Cursor, Write},
    path::Path,
};

mod error;
pub use error::{DocimaError, DocimaResult, StdResult};

/// Writes an image file as an HTML <img> tag with base64 encoded png data,
/// generated by a custom function.
///
/// # Parameters
/// - `f`: a function that takes a mutable RGB8 buffer, a width, a height,
///   and returns a result.
/// - `width`: the width of the image
/// - `height`: the height of the image
/// - `filename`: the output filename including its path from the project's root.
/// - `alt`: the alt attribute for the `<img>` tag.
/// - `title`: the title attribute for the `<img>` tag.
/// - `wrap`: the optional HTML tag to wrap the `<img>` tag with (without brackets).
///
/// # Example
///
/// In order to generate the image from the build script (e.g. in `build.rs`):
/// ```ignore
/// generate_image(my_function, 40, 40, "images/my_image.html", "alt", "title", "span")?;
/// ```
///
/// And in order to embed the image in the Rust documentation (e.g. in `src/lib.rs`):
/// ```ignore
/// /// A truly beautiful image.
/// ///
/// #[doc = include_str!("../images/my_image.html") ]
/// pub struct MyBeautifulImage;
/// ```
///
pub fn generate_image<F>(
    f: F,
    width: u32,
    height: u32,
    output_file: &str,
    alt: &str,
    title: &str,
    wrap: &str,
) -> DocimaResult<()>
where
    F: Fn(&mut Vec<u8>, u32, u32) -> StdResult<()>,
{
    let mut rgb_buffer = vec![0; width as usize * height as usize * 3];

    // generate the image as rgb8 using the provided function
    f(&mut rgb_buffer, width, height)?;

    // encode the image as png data
    let mut png_buffer = Vec::<u8>::new();
    let cursor_buffer = Cursor::new(&mut png_buffer);
    let encoder =
        PngEncoder::new_with_quality(cursor_buffer, CompressionType::Best, FilterType::Paeth);
    encoder.encode(rgb_buffer.as_slice(), width, height, ColorType::Rgb8)?;

    // encode the png data as base64 data
    let base64 = BASE64_MIME.encode(png_buffer.as_slice());

    // embed the base64 data in HTML tags
    let mut content = format![
        "<img src=\"data:image/png;base64,\n{}\" alt=\"{}\" title=\"{}\" />",
        base64, alt, title
    ];

    if !wrap.is_empty() {
        content = format!["<{0}>{1}</{0}>", wrap, content];
    }

    // prepare the output path
    let filepath_str = root_path(output_file);
    let filepath = Path::new(&filepath_str);
    let dirpath = filepath.parent().ok_or_else(|| {
        DocimaError::Custom(format![
            "no parent: `{}`",
            filepath.to_str().expect("filepath.to_str()")
        ])
    })?;

    create_dir_all(dirpath)?;

    // write the output string to the desired location
    let mut outfile = File::create(filepath)?;
    write!(outfile, "{}", content)?;

    Ok(())
}

/// Returns a path relative to the root of the project.
fn root_path(relative: &str) -> String {
    let mut path = project_root::get_project_root().expect("get_project_root");
    path.push(relative);
    path.to_str().expect("path.to_str()").to_owned()
}
