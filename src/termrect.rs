use delta::{Delta, Delta::*};
use line::Line;
use style::Style;
use styledtext::StyledText;

use std::iter::repeat;

/// TermRect is a representation of a rectangle of characters in a terminal
/// grid. It keeps track of changes between calls to draw_delta_into, to always
/// do only minimal updates.
#[derive(Debug)]
pub struct TermRect {
    size: (u32, u32),
    lines: Vec<Line>,
    delta: Delta,
}

impl TermRect {
    /// Create a new TermRect filled with blank lines. Starts out with an empty
    /// delta, so the first draw_delta_into will do nothing.
    pub fn new(size: (u32, u32)) -> TermRect {
        TermRect {
            size,
            lines: repeat(Line::new(size.0)).take(size.1 as _).collect(),
            // TODO: Should this be Range(0, size.1) instead?
            delta: Unchanged,
        }
    }
}

pub trait RawPaintable: HasSize {
    /// Draw the text at the position. Return true if something changed.
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText) -> bool;

    /// Draw the string at the position. Return true if something changed. This
    /// is a simple wrapper around draw_text_at if you don't already have a
    /// StyledText.
    fn draw_str_at(&mut self, pos: (u32, u32), style: Style, str: String) -> bool {
        self.draw_text_at(pos, &StyledText::new(style, str))
    }

    /// Clear the whole line after pos. Return true if something changed.
    fn clear_line(&mut self, pos: (u32, u32), style: Style) -> bool {
        let spaces = " ".repeat(self.size().0 as _);
        self.draw_str_at(pos, style, spaces)
    }
}

pub trait HasSize {
    /// The width, height of the widget.
    fn size(&self) -> (u32, u32);
}

pub trait PaintableWidget: HasSize {
    /// Draw this widget into the target. Same as mark_all_changed followed by draw_delta_into.
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32));

    /// Draw only what has changed since last time into the target. This clears
    /// the delta info for next time.
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, pos: (u32, u32)) {
        // if there isn't a faster implementation, just call draw_into
        self.draw_into(target, pos)
    }

    /// Mark everything changed. Next draw_delta_into will redraw everything.
    fn mark_all_changed(&mut self) {}

    /// Mark nothing changed. Next draw_delta_into will do nothing.
    fn mark_none_changed(&mut self) {}
}

impl RawPaintable for TermRect {
    fn draw_text_at(&mut self, pos: (u32, u32), text: &StyledText) -> bool {
        let y = pos.1 as usize;
        if pos.1 < self.size.1 && y < self.lines.len() {
            if self.lines[y].draw_text_at(pos.0, text) {
                self.delta.add(y);
                true
            } else {
                false
            }
        } else {
            false
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
            l.draw_into(target, (pos.0, pos.1 + i as u32));
        }
    }
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, pos: (u32, u32)) {
        for (i, l) in self.lines.iter_mut().enumerate() {
            if self.delta.contains(i) {
                l.draw_delta_into(target, (pos.0, pos.1 + i as u32));
            }
        }
        self.mark_none_changed();
    }

    fn mark_all_changed(&mut self) {
        self.delta = Range(0, self.size.1 as usize);
    }

    fn mark_none_changed(&mut self) {
        self.delta = Unchanged;
    }
}

/// HasTermRect is intended for use in widgets that render themselves to an
/// embedded TermRect. By implementing this, you atomatically get an impl for
/// PaintableWidget and HasSize.
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

    fn mark_all_changed(&mut self) {
        self.termrect_mut().mark_all_changed()
    }

    fn mark_none_changed(&mut self) {
        self.termrect_mut().mark_none_changed()
    }
}
