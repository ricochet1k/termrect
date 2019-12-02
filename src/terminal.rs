use style::{Style, StyleFromTo};
use styledtext::StyledText;
use termrect::{HasSize, RawPaintable};

use std;
use std::io::Write;
use termion;

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
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText) -> bool {
        // TODO: Track cursor position and only move if necessary
        write!(
            self.w,
            "{}",
            termion::cursor::Goto(1 + pos.0 as u16, 1 + pos.1 as u16)
        )
        .unwrap();
        if self.current_style != text.style {
            write!(
                self.w,
                "{}",
                StyleFromTo {
                    from: self.current_style,
                    to: text.style
                }
            )
            .unwrap();
            self.current_style = text.style;
        }
        write!(self.w, "{}", text.text).unwrap();
        true
    }
}
