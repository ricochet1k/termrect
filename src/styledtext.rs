use style::Style;
use termrect::{HasSize, PaintableWidget, RawPaintable};

use std;
use std::ops::{Bound, RangeBounds};
use std::rc::Rc;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// StyledText represents a span of text that all has the same style. It also
/// keeps track of the unicode width of the text.
#[derive(Debug, Clone)]
pub struct StyledText {
    pub(crate) style: Style,
    pub(crate) text: Rc<String>,
    pub(crate) width: u32,
}

/// Width sensitive slice. A and B are counted in cells.
fn width_slice(txt: &str, a: usize, b: usize) -> &str {
    let mut width_so_far = 0;
    let mut found_start = false;
    let mut start_index = 0;
    let mut end_index = txt.len();
    for (i, c) in txt.char_indices() {
        if !found_start && width_so_far >= a {
            //if width_so_far > a {
            //panic!("Slice in the middle of a double-width char!");
            //}
            start_index = i;
            found_start = true;
        }
        if found_start && width_so_far >= b {
            end_index = i;
            break;
        }

        width_so_far += UnicodeWidthChar::width(c).unwrap_or(0);
    }

    if !found_start {
        return "";
    }

    // start_index and end_index are set using char_indices, so this should
    // be safe. If a problem is suspected, this will panic instead:
    //   txt.get(start_index..end_index).unwrap()
    unsafe { txt.get_unchecked(start_index..end_index) }
}

impl StyledText {
    /// Create a new StyledText.
    pub fn new(style: Style, text: String) -> StyledText {
        let width = UnicodeWidthStr::width(&text as &str);
        StyledText {
            style,
            text: Rc::new(text),
            width: width as _,
        }
    }

    /// Slice the string returning a new StyledText with the same style. Slicing is done by
    /// width, rather than by byte or by char. So the returned slice should have exactly the
    /// width specified.
    pub fn slice<R: RangeBounds<usize>>(&self, r: R) -> StyledText
    where
        String: std::ops::Index<R>,
    {
        let a = match r.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => i + 1,
            Bound::Unbounded => 0,
        };
        let b = match r.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.width as _,
        };
        let sliced = width_slice(&self.text, a, b).to_string();
        StyledText {
            style: self.style,
            text: Rc::new(sliced),
            width: (b - a) as u32,
        }
    }
}

impl HasSize for StyledText {
    fn size(&self) -> (u32, u32) {
        (self.width, 1)
    }
}

impl PaintableWidget for StyledText {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32)) {
        target.draw_text_at(pos, &self);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::clone::Clone;
    use std::fmt::Debug;

    #[test]
    fn empty_styledtext() {
        StyledText::new(Style::default(), "".to_string());
    }

    fn slice_test<R: RangeBounds<usize> + Clone + Debug>(s: &str, r: R, sl: &str, w: u32)
    where
        String: std::ops::Index<R>,
    {
        let slice = StyledText::new(Style::default(), s.to_string()).slice(r.clone());
        assert_eq!(
            *slice.text, sl,
            "Slice str is incorrect: want: {:?}, got {:?}",
            sl, slice.text
        );
        assert_eq!(
            slice.width, w,
            "Slice width is incorrect: {:?}.slice({:?}) width is {}, wanted {}",
            s, r, slice.width, w
        );
    }

    #[test]
    fn slicing() {
        slice_test("asdf", .., "asdf", 4);
        slice_test("asdf", 0..4, "asdf", 4);
        slice_test("asdf", 1..3, "sd", 2);
        slice_test("台北1234", .., "台北1234", 8); // 2 double-width chars
        slice_test("台北1234", 0..2, "台", 2); // 2 double-width chars

        // TODO: This keeps 1 double width char, even though we only asked for 1 width.
        //       Is this what terminal emulators do?
        slice_test("台北1234", 0..1, "台", 1);

        // TODO: This tries to start a slice in the middle of one char. What to do?
        //       We definitely don't want to crash.
        slice_test("台北1234", 1..2, "", 1);

        slice_test("台北1234", 2..4, "北", 2);
        slice_test("ＱＲＳ12", .., "ＱＲＳ12", 8);
        slice_test("ｱｲｳ1234", .., "ｱｲｳ1234", 7); // 3 single-width chars
    }
}
