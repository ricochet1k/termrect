
use std;
use termion;

use termrect::{RawPaintable, HasSize};
use style::Style;
use styledtext::StyledText;
use std::io::Write;

pub struct Terminal<W: Write> {
    size: (u32, u32),
    current_style: Style,
    w: W,
}

impl<W: Write> Terminal<W> {
    pub fn new(mut w: W, size: (u32, u32)) -> Terminal<W> {
        let current_style = Style::default();
        write!(w, "{}", current_style).unwrap();
        Terminal {
            size,
            current_style,
            w,
        }
    }
    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.w.flush()
    }
}

impl<W: Write> HasSize for Terminal<W> {
    fn size(&self) -> (u32, u32) {
        self.size
    }
}

impl<W: Write> RawPaintable for Terminal<W> {
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText) {
        write!(self.w, "{}{}{}", termion::cursor::Goto(1 + pos.0 as u16, 1 + pos.1 as u16), text.style, text.text).unwrap();
    }
}

