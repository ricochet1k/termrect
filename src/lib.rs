#![feature(collections_range, iterator_flatten, specialization, try_from)]

#[macro_use]
extern crate bitfield;
extern crate termion;
extern crate unicode_width;

pub mod line;
pub mod style;
pub mod styledtext;
pub mod terminal;
pub mod termrect;

pub use style::{Color, Style, StyleAttr};
pub use terminal::Terminal;
