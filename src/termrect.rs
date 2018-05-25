
use styledtext::StyledText;
use style::Style;
use line::Line;

use std::iter::repeat;


#[derive(Debug)]
pub struct TermRect {
    size: (u32, u32),
    lines: Vec<Line>,
}

impl TermRect {
    pub fn new(size: (u32, u32)) -> TermRect {
        TermRect {
            size,
            lines: repeat(Line::new(size.0)).take(size.1 as _).collect(),
        }
    }
}

pub trait RawPaintable: HasSize {
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText);

    fn draw_str_at(&mut self, pos: (u32, u32), style: Style, str: String) {
        self.draw_text_at(pos, &StyledText::new(style, str));
    }

    fn clear_line(&mut self, pos: (u32, u32), style: Style) {
        let spaces = " ".repeat(self.size().0 as _);
        self.draw_str_at(pos, style, spaces);
    }
}

pub trait HasSize {
    fn size(&self) -> (u32, u32);
}

pub trait PaintableWidget: HasSize {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32));
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, pos: (u32, u32)) {
        // if there isn't a faster implementation, just call draw_into
        self.draw_into(target, pos)
    }
}

impl RawPaintable for TermRect {
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText) {
        if pos.1 < self.size.1 && (pos.1 as usize) < self.lines.len() {
            self.lines[pos.1 as usize].draw_text_at(pos.0, text)
        }
    }
}

impl HasSize for TermRect {
    fn size(&self) -> (u32, u32) {
        self.size
    }
}

impl PaintableWidget for TermRect {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32)) {
        for (i, l) in self.lines.iter().enumerate() {
            l.draw_into(target, (pos.0, pos.1 + i as u32))
        }
    }
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, pos: (u32, u32)) {
        for (i, l) in self.lines.iter_mut().enumerate() {
            l.draw_delta_into(target, (pos.0, pos.1 + i as u32))
        }
    }
}

pub trait HasTermRect {
    fn termrect(&self) -> &TermRect;
    fn termrect_mut(&mut self) -> &mut TermRect;
}

impl<T: HasTermRect> HasSize for T {
    fn size(&self) -> (u32, u32) {
        self.termrect().size
    }
}

impl<T: HasTermRect> PaintableWidget for T {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32)) {
        let sb = self.termrect();
        sb.draw_into(target, pos)
    }
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, pos: (u32, u32)) {
        let sb = self.termrect_mut();
        sb.draw_delta_into(target, pos)
    }
}

