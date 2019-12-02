#[cfg(feature = "termion")]
use std::fmt::{Display, Error, Formatter};

#[cfg(feature = "termion")]
use termion;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Default,
    Indexed(u16),
    RGB(u8, u8, u8),
}

impl Color {
    #[cfg(feature = "termion")]
    fn termion_color(&self) -> Box<termion::color::Color> {
        match *self {
            Color::Default => Box::new(termion::color::Reset),
            Color::Indexed(i) => Box::new(termion::color::AnsiValue(i as _)),
            Color::RGB(r, g, b) => Box::new(termion::color::Rgb(r, g, b)),
        }
    }
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct StyleAttrs(u16);
    impl Debug;
    bold, set_bold: 0;
    italic, set_italic: 1;
    faint, set_faint: 2;
    crossedout, set_crossedout: 3;
    invert, set_invert: 4;
    underline, set_underline: 5;
}

#[derive(Copy, Clone)]
pub enum StyleAttr {
    Bold,
    Italic,
    Faint,
    CrossedOut,
    Invert,
    Underline,
}

impl StyleAttr {
    fn isset_in(&self, attrs: &StyleAttrs) -> bool {
        match self {
            StyleAttr::Bold => attrs.bold(),
            StyleAttr::Italic => attrs.italic(),
            StyleAttr::Faint => attrs.faint(),
            StyleAttr::CrossedOut => attrs.crossedout(),
            StyleAttr::Invert => attrs.invert(),
            StyleAttr::Underline => attrs.underline(),
        }
    }

    fn set_to_in(&self, to: bool, attrs: &mut StyleAttrs) {
        match self {
            StyleAttr::Bold => attrs.set_bold(to),
            StyleAttr::Italic => attrs.set_italic(to),
            StyleAttr::Faint => attrs.set_faint(to),
            StyleAttr::CrossedOut => attrs.set_crossedout(to),
            StyleAttr::Invert => attrs.set_invert(to),
            StyleAttr::Underline => attrs.set_underline(to),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(feature = "termion")]
impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
        if self.attrs.crossedout() {
            write!(f, "{}", termion::style::CrossedOut)?
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

#[cfg(feature = "termion")]
pub(crate) struct StyleFromTo {
    pub(crate) from: Style,
    pub(crate) to: Style,
}

#[cfg(feature = "termion")]
fn write_attr<S: Display, R: Display>(
    f: &mut Formatter,
    fromattr: bool,
    toattr: bool,
    set: S,
    reset: R,
) -> Result<(), Error> {
    if fromattr && !toattr {
        write!(f, "{}", reset)?;
    } else if !fromattr && toattr {
        write!(f, "{}", set)?;
    }
    Ok(())
}

#[cfg(feature = "termion")]
impl Display for StyleFromTo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.from.fg != self.to.fg {
            self.to.fg.termion_color().write_fg(f)?;
        }
        if self.from.bg != self.to.bg {
            self.to.bg.termion_color().write_bg(f)?;
        }
        write_attr(
            f,
            self.from.attrs.bold(),
            self.to.attrs.bold(),
            termion::style::Bold,
            termion::style::NoBold,
        )?;
        write_attr(
            f,
            self.from.attrs.italic(),
            self.to.attrs.italic(),
            termion::style::Italic,
            termion::style::NoItalic,
        )?;
        write_attr(
            f,
            self.from.attrs.faint(),
            self.to.attrs.faint(),
            termion::style::Faint,
            termion::style::NoFaint,
        )?;
        write_attr(
            f,
            self.from.attrs.crossedout(),
            self.to.attrs.crossedout(),
            termion::style::CrossedOut,
            termion::style::NoCrossedOut,
        )?;
        write_attr(
            f,
            self.from.attrs.invert(),
            self.to.attrs.invert(),
            termion::style::Invert,
            termion::style::NoInvert,
        )?;
        write_attr(
            f,
            self.from.attrs.underline(),
            self.to.attrs.underline(),
            termion::style::Underline,
            termion::style::NoUnderline,
        )?;
        Ok(())
    }
}
