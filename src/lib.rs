#[macro_use]
extern crate bitfield;
extern crate itertools;
extern crate unicode_width;

pub mod delta;
pub mod line;
pub mod style;
pub mod styledtext;
pub mod termrect;

#[cfg(feature = "termion")]
extern crate termion;
#[cfg(feature = "termion")]
pub mod terminal;

pub use style::{Color, Style, StyleAttr};
pub use termrect::TermRect;
