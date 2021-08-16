//! # docima
//!
//! The functions are intended to be called from the build script
//!
//! ## Usage
//!
//! 1. Configure your build script to generate the images in the desired path.
//!
//! ```ignore
//! ImageFile::new()
//!     .path("images/my_image.html")
//!     .width(600)
//!     .height(400)
//!     .title("My image")
//!     .id("image-01")
//!     .style("display: block; margin:auto;")
//!     .wrapper("div")
//!     .wrapper_style("background-color:red; padding:3px;")
//!     .overwrite(true)
//!     .generate(my_image_generator_function)?;
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
use std::{
    fs::{create_dir_all, File},
    io::{Cursor, Write},
    path::Path,
};

mod error;
pub use error::{DocimaError, DocimaResult, StdResult};

/// An image file generator.
///
/// The minimum required setup methods are: [`width`][ImageFile#method.width],
/// [`height`][ImageFile#method.height] and [`path`][ImageFile#method.path]
///
/// # Example
///
/// ```ignore
///
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
    alt: String,
    title: String,
    id: String,
    class: String,
    style: String,

    // html wrapper tag + attributes
    wrapper: String,
    wrapper_alt: String,
    wrapper_title: String,
    wrapper_id: String,
    wrapper_class: String,
    wrapper_style: String,
    // `href` & `target` attributes will be used only when wrapper == "a"
    wrapper_href: String,
    wrapper_target: String,

    // controls whether existing images should be overwritten.
    overwrite: bool,
}

impl Default for ImageFile {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            path: String::default(),
            alt: String::default(),
            title: String::default(),
            id: String::default(),
            class: String::default(),
            style: String::default(),
            wrapper: String::default(),
            wrapper_alt: String::default(),
            wrapper_title: String::default(),
            wrapper_id: String::default(),
            wrapper_class: String::default(),
            wrapper_style: String::default(),
            wrapper_href: String::default(),
            wrapper_target: String::default(),
            // MAYBE set with feature
            overwrite: false,
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

        // TODO WIP

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
        if !self.id.is_empty() {
            content += &format!["id=\"{}\" ", self.id];
        }
        if !self.class.is_empty() {
            content += &format!["class=\"{}\" ", self.class];
        }
        if !self.alt.is_empty() {
            content += &format!["alt=\"{}\" ", self.alt];
        }
        if !self.title.is_empty() {
            content += &format!["title=\"{}\" ", self.title];
        }
        if !self.style.is_empty() {
            content += &format!["style=\"{}\" ", self.style];
        }
        content += "/>";

        // add the wrapper HTML tag
        if !self.wrapper.is_empty() {
            let mut wrapper_open = format!["<{0} ", self.wrapper];

            // add the wrapper attributes to the opening tag
            if !self.wrapper_id.is_empty() {
                wrapper_open += &format!["class=\"{}\" ", self.wrapper_id];
            }
            if !self.wrapper_class.is_empty() {
                wrapper_open += &format!["class=\"{}\" ", self.wrapper_class];
            }
            if !self.wrapper_alt.is_empty() {
                wrapper_open += &format!["alt=\"{}\" ", self.wrapper_alt];
            }
            if !self.wrapper_title.is_empty() {
                wrapper_open += &format!["title=\"{}\" ", self.wrapper_title];
            }
            if !self.wrapper_style.is_empty() {
                wrapper_open += &format!["style=\"{}\" ", self.wrapper_style];
            }
            // anchor specific attributes
            if self.wrapper == "a" {
                if !self.wrapper_href.is_empty() {
                    wrapper_open += &format!["href=\"{}\" ", self.wrapper_href];
                }
                if !self.wrapper_target.is_empty() {
                    wrapper_open += &format!["target=\"{}\" ", self.wrapper_target];
                }
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
    /// Sets the `<img>` `alt` attribute.
    pub fn alt(mut self, alt: &str) -> Self {
        self.alt = alt.into();
        self
    }

    /// Sets the `<img>` `title` attribute.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the `<img>` `id` attribute.
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the `<img>` `class` attribute.
    pub fn class(mut self, class: &str) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the `<img>` `style` attribute.
    pub fn style(mut self, style: &str) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the `wrapper` HTML tag around `<img>`.
    pub fn wrapper(mut self, wrapper: &str) -> Self {
        self.wrapper = wrapper.into();
        self
    }

    /// Sets the wrapper tag `alt` attribute.
    pub fn wrapper_alt(mut self, alt: &str) -> Self {
        self.wrapper_alt = alt.into();
        self
    }

    /// Sets the wrapper tag `title` attribute.
    pub fn wrapper_title(mut self, title: &str) -> Self {
        self.wrapper_title = title.into();
        self
    }

    /// Sets the wrapper tag `id` attribute.
    pub fn wrapper_id(mut self, id: &str) -> Self {
        self.wrapper_id = id.into();
        self
    }

    /// Sets the wrapper tag `class` attribute.
    pub fn wrapper_class(mut self, class: &str) -> Self {
        self.wrapper_class = class.into();
        self
    }

    /// Sets the wrapper tag `style` attribute.
    pub fn wrapper_style(mut self, style: &str) -> Self {
        self.wrapper_style = style.into();
        self
    }

    /// Sets the wrapper tag `href` attribute.
    ///
    /// The `wrapper` must be an anchor tag (`a`) in order for this to be used.
    pub fn wrapper_href(mut self, href: &str) -> Self {
        self.wrapper_href = href.into();
        self
    }

    /// Sets the wrapper tag `target` attribute.
    ///
    /// The `wrapper` must be an anchor tag (`a`) in order for this to be used.
    pub fn wrapper_target(mut self, target: &str) -> Self {
        self.wrapper_target = target.into();
        self
    }

    /// Sets the wrapper tag `style` attribute.
    ///
    /// If `false` the image will only be generated if the chosen output file
    /// doesn't already exist.
    /// If `true` the image will always be generated, and the file overwritten.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }
}

/// Returns a path relative to the root of the project.
fn root_path(relative: &str) -> String {
    let mut path = project_root::get_project_root().expect("get_project_root()");
    path.push(relative);
    path.to_str().expect("path.to_str()").to_owned()
}
