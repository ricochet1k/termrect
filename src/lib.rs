#![feature(collections_range, iterator_flatten, specialization, try_from)]

#[macro_use]
extern crate bitfield;
extern crate termion;
extern crate unicode_width;

pub mod termrect;
pub mod terminal;
pub mod line;
pub mod style;
pub mod styledtext;

pub use termrect::{TermRect, PaintableWidget, RawPaintable, HasSize, HasTermRect};
pub use terminal::Terminal;
pub use style::{Style, Color, StyleAttr};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
