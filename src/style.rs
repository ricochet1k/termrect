use std;
use std::fmt::{Display, Formatter};
use termion;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Default,
    Indexed(u16),
    RGB(u8, u8, u8),
}

impl Color {
    fn termion_color(&self) -> Box<termion::color::Color> {
        match *self {
            Color::Default => Box::new(termion::color::Reset),
            Color::Indexed(i) => Box::new(termion::color::AnsiValue(i as _)),
            Color::RGB(r, g, b) => Box::new(termion::color::Rgb(r, g, b)),
        }
    }
}

bitfield!{
    #[derive(Copy, Clone)]
    pub struct StyleAttrs(u16);
    impl Debug;
    bold, set_bold: 0;
    italic, set_italic: 1;
    faint, set_faint: 2;
    framed, set_framed: 3;
    invert, set_invert: 4;
    underline, set_underline: 5;
}

#[derive(Copy, Clone)]
pub enum StyleAttr {
    Bold,
    Italic,
    Faint,
    Framed,
    Invert,
    Underline,
}

impl StyleAttr {
    fn isset_in(&self, attrs: &StyleAttrs) -> bool {
        match self {
            StyleAttr::Bold => attrs.bold(),
            StyleAttr::Italic => attrs.italic(),
            StyleAttr::Faint => attrs.faint(),
            StyleAttr::Framed => attrs.framed(),
            StyleAttr::Invert => attrs.invert(),
            StyleAttr::Underline => attrs.underline(),
        }
    }

    fn set_to_in(&self, to: bool, attrs: &mut StyleAttrs) {
        match self {
            StyleAttr::Bold => attrs.set_bold(to),
            StyleAttr::Italic => attrs.set_italic(to),
            StyleAttr::Faint => attrs.set_faint(to),
            StyleAttr::Framed => attrs.set_framed(to),
            StyleAttr::Invert => attrs.set_invert(to),
            StyleAttr::Underline => attrs.set_underline(to),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Style {
    fg: Color,
    bg: Color,
    attrs: StyleAttrs,
}

impl Style {
    pub fn set_fg(&self, c: Color) -> Style {
        Style {
            fg: c,
            bg: self.bg,
            attrs: self.attrs,
        }
    }

    pub fn set_bg(&self, c: Color) -> Style {
        Style {
            fg: self.fg,
            bg: c,
            attrs: self.attrs,
        }
    }

    pub fn isset(&self, a: StyleAttr) -> bool {
        a.isset_in(&self.attrs)
    }

    pub fn set(&self, a: StyleAttr) -> Style {
        let mut newattrs = self.attrs;
        a.set_to_in(true, &mut newattrs);
        Style {
            fg: self.fg,
            bg: self.bg,
            attrs: newattrs,
        }
    }

    pub fn clear(&self, a: StyleAttr) -> Style {
        let mut newattrs = self.attrs;
        a.set_to_in(false, &mut newattrs);
        Style {
            fg: self.fg,
            bg: self.bg,
            attrs: newattrs,
        }
    }

    pub fn toggle(&self, a: StyleAttr) -> Style {
        let mut newattrs = self.attrs;
        a.set_to_in(!a.isset_in(&self.attrs), &mut newattrs);
        Style {
            fg: self.fg,
            bg: self.bg,
            attrs: newattrs,
        }
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::Default,
            bg: Color::Default,
            attrs: StyleAttrs(0),
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.fg.termion_color().write_fg(f)?;
        self.bg.termion_color().write_bg(f)?;
        write!(f, "{}", termion::style::Reset)?;
        if self.attrs.bold() {
            write!(f, "{}", termion::style::Bold)?
        }
        if self.attrs.italic() {
            write!(f, "{}", termion::style::Italic)?
        }
        if self.attrs.faint() {
            write!(f, "{}", termion::style::Faint)?
        }
        if self.attrs.framed() {
            write!(f, "{}", termion::style::Framed)?
        }
        if self.attrs.invert() {
            write!(f, "{}", termion::style::Invert)?
        }
        if self.attrs.underline() {
            write!(f, "{}", termion::style::Underline)?
        }
        Ok(())
    }
}
