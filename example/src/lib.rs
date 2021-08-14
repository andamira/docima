//!
//! # Examples
//!
#![doc = include_str!("../images/plotters-histogram.html") ]
//!
//! Some random colors
#![doc = include_str!("../images/square-random-pixels.html") ]

/// A type example with embedded images
///
/// # An histogram
#[doc = include_str!("../images/plotters-histogram.html") ]
///
/// And a square with random pixels
#[doc = include_str!("../images/square-random-pixels.html") ]
pub struct ExampleType;

pub mod example_module {
    //! Some random colors
    #![doc = include_str!("../images/square-random-pixels.html") ]
}
