//! # docima
//!
//! ## Usage
//!
//! 1. Configure your build script to generate the images in the desired path.
//!
//! ```
//! ImageFile::new()
//!     .path("images/my_image.html")
//!     .width(600)
//!     .height(400)
//!     .attr("title", "My image")
//!     .attr("id", "image-01")
//!     .attr("style", "display: block; margin:auto;")
//!     .wrapper("div")
//!     .wrapper_attr("style", "background-color:red; padding:3px;")
//!     .overwrite(true)
//!     .generate(my_image_generator_function)?;
//! ```
//!
//! 2. Include your image in the docs by using the [doc][0] attribute and
//!    the [include_str][1] macro.
//!
//! ```
//! #[doc = include_str!("../images/my_image.html")]
//! struct Foo;
//! ```
//! [0]:https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
//! [1]:https://doc.rust-lang.org/std/macro.include_str.html
//!
//! 3. Generate the docs:
//! ```sh
//! $ cargo doc --open
//! ```
//! # Examples
//!
//! The following examples are generated by this [`build.rs`][buildrs]:
//!
//! [buildrs]:https://github.com/andamira/docima/blob/main/build.rs
//!
#![doc = include_str!("../images/plotters-histogram.html") ]
//!
//! Inline
#![doc = include_str!("../images/square-random-pixels.html") ]
//! embedding.
//!

use data_encoding::BASE64_MIME;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::{Cursor, Write},
    path::Path,
};

mod error;
mod utils;
pub use error::{DocimaError, DocimaResult, StdResult};
use utils::root_path;

/// An image file generator.
///
/// The minimum required setup methods are: [`width`][ImageFile#method.width],
/// [`height`][ImageFile#method.height] and [`path`][ImageFile#method.path]
///
/// # Example
///
/// ```
/// ImageFile::new()
///     .width(600)
///     .height(400)
///     .path("images/plotters-histogram.html")
///     .overwrite(true)
///     .title("the title")
///     .alt("the alt text")
///     .wrapper("div")
///     .wrapper_style("padding: 10px; background-color: red;")
///     .generate(my_generator_function)?;
/// ```
pub struct ImageFile {
    // required fields:

    // image width & height
    width: u32,
    height: u32,
    // output file path
    path: String,

    // optional fields:

    // <img> attributes
    attributes: BTreeMap<String, String>,

    // html wrapper tag & attributes
    wrapper: String,
    wrapper_attributes: BTreeMap<String, String>,

    // controls whether existing images should be overwritten.
    overwrite: bool,
}

impl Default for ImageFile {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            path: String::default(),
            attributes: BTreeMap::new(),

            wrapper: String::default(),
            wrapper_attributes: BTreeMap::new(),

            #[cfg(feature = "not_default_overwrite")]
            overwrite: false,
            #[cfg(not(feature = "not_default_overwrite"))]
            overwrite: true,
        }
    }
}

/// # Constructor & Generator
impl ImageFile {
    /// Start configuring a new image.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finishes the image, calling the generator function and saving the file.
    pub fn generate(
        self,
        generator: impl Fn(&mut Vec<u8>, u32, u32) -> StdResult<()>,
    ) -> DocimaResult<()> {
        if self.width == 0 {
            return Err(DocimaError::MissingField("width".into()));
        } else if self.height == 0 {
            return Err(DocimaError::MissingField("height".into()));
        } else if self.path.is_empty() {
            return Err(DocimaError::MissingField("path".into()));
        }

        // When the `build_when_doc` feature is active the images will be built
        // only if the `doc` feature is used too, usually alongside `cargo doc`.
        if cfg![feature = "build_when_doc"] && cfg![not(feature = "doc")] {
            return Ok(());
        }

        // prepare the output path
        let filepath_str = root_path(&self.path);
        let filepath = Path::new(&filepath_str);
        let dirpath = filepath.parent().ok_or_else(|| {
            DocimaError::Custom(format![
                "no parent: `{}`",
                filepath.to_str().expect("filepath.to_str()")
            ])
        })?;
        if !dirpath.exists() {
            create_dir_all(dirpath)?;
        }

        // Don't generate this image if the file already exists and we're not overwriting.
        if filepath.exists() && !self.overwrite {
            return Ok(());
        }

        let mut rgb_buffer = vec![0; self.width as usize * self.height as usize * 3];

        // generate the image as rgb8 using the provided function
        generator(&mut rgb_buffer, self.width, self.height)?;

        // encode the image as png data in a memory buffer
        let mut png_buffer = Vec::<u8>::new();
        {
            let cursor_buffer = Cursor::new(&mut png_buffer);
            let mut encoder = png::Encoder::new(cursor_buffer, self.width, self.height);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_compression(png::Compression::Best);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(&rgb_buffer)?;
        }

        // encode the png data as base64 data
        let base64 = BASE64_MIME.encode(png_buffer.as_slice());

        // embed the base64 data in HTML tag
        let mut content = format!["<img src=\"data:image/png;base64,\n{}\" ", base64];

        // add the <img> attributes
        for (attr, value) in self.attributes {
            content += &format!["{0}=\"{1}\" ", attr, value];
        }

        content += "/>";

        // add the wrapper HTML tag
        if !self.wrapper.is_empty() {
            let mut wrapper_open = format!["<{0} ", self.wrapper];

            // add the wrapper attributes to the opening tag
            for (attr, value) in self.wrapper_attributes {
                wrapper_open += &format!["{0}=\"{1}\" ", attr, value];
            }

            wrapper_open += ">";

            content = format!["{0}{1}</{2}>", wrapper_open, content, self.wrapper];
        }

        // write the output string to the desired location only
        let mut outfile = File::create(filepath)?;
        write!(outfile, "{}", content)?;

        Ok(())
    }
}

/// # Required Configuration methods
impl ImageFile {
    /// Sets the width of the image.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the image.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the name and path of the output file, including the path from the
    /// project's root.
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.into();
        self
    }
}

/// # Optional Configuration methods
impl ImageFile {
    /// Sets the chosen attribute for the `<img>` tag.
    ///
    /// Valid attributes are: "alt", "title", "id", "class", "style", ???
    pub fn attr(mut self, attribute: &str, value: &str) -> Self {
        self.attributes.insert(attribute.into(), value.into());
        self
    }

    /// Sets the `<img>` `alt` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn alt(mut self, alt: &str) -> Self {
        self.attributes.insert("alt".into(), alt.into());
        self
    }

    /// Sets the `<img>` `title` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn title(mut self, title: &str) -> Self {
        self.attributes.insert("title".into(), title.into());
        self
    }

    /// Sets the `<img>` `id` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn id(mut self, id: &str) -> Self {
        self.attributes.insert("id".into(), id.into());
        self
    }

    /// Sets the `<img>` `class` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn class(mut self, class: &str) -> Self {
        self.attributes.insert("class".into(), class.into());
        self
    }

    /// Sets the `<img>` `style` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn style(mut self, style: &str) -> Self {
        self.attributes.insert("style".into(), style.into());
        self
    }

    /// Sets the `wrapper` HTML tag around `<img>`.
    pub fn wrapper(mut self, wrapper: &str) -> Self {
        self.wrapper = wrapper.into();
        self
    }

    /// Sets an attribute for the wrapper tag around `<img>`.
    ///
    /// Valid attributes are: "alt", "title", "id", "class", "style"???
    /// And if the wrapper is an anchor "a", then "href" & "target".
    pub fn wrapper_attr(mut self, attribute: &str, value: &str) -> Self {
        self.wrapper_attributes
            .insert(attribute.into(), value.into());
        self
    }

    /// Sets the wrapper tag `alt` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_alt(mut self, alt: &str) -> Self {
        self.wrapper_attributes.insert("alt".into(), alt.into());
        self
    }

    /// Sets the wrapper tag `title` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_title(mut self, title: &str) -> Self {
        self.wrapper_attributes.insert("title".into(), title.into());
        self
    }

    /// Sets the wrapper tag `id` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_id(mut self, id: &str) -> Self {
        self.wrapper_attributes.insert("id".into(), id.into());
        self
    }

    /// Sets the wrapper tag `class` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_class(mut self, class: &str) -> Self {
        self.wrapper_attributes.insert("class".into(), class.into());
        self
    }

    /// Sets the wrapper tag `style` attribute.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_style(mut self, style: &str) -> Self {
        self.wrapper_attributes.insert("style".into(), style.into());
        self
    }

    /// Sets the wrapper tag `href` attribute.
    ///
    /// The `wrapper` must be an anchor tag (`a`) for this attribute to be valid.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_href(mut self, href: &str) -> Self {
        self.wrapper_attributes.insert("href".into(), href.into());
        self
    }

    /// Sets the wrapper tag `target` attribute.
    ///
    /// The `wrapper` must be an anchor tag (`a`) for this attribute to be valid.
    #[deprecated]
    #[doc(hidden)]
    pub fn wrapper_target(mut self, target: &str) -> Self {
        self.wrapper_attributes
            .insert("target".into(), target.into());
        self
    }

    /// Sets the `overwrite` preference for the generated image.
    ///
    /// If `false` the image will only be generated if the chosen output file
    /// doesn't already exist.
    /// If `true` the image will always be generated, and the file overwritten.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }
}
