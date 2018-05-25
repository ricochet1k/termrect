
use termrect::{RawPaintable, PaintableWidget, HasSize};
use style::Style;

use std;
use std::ops::RangeBounds;
use std::rc::Rc;

use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};


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
        if !found_start {
            if width_so_far == a {
                start_index = i;
                found_start = true;
            }
            else if width_so_far > a {
                panic!("Slice in the middle of a double-width char!");
            }
        }
        if found_start {
            if width_so_far >= b {
                end_index = i;
                break;
            }
        }

        width_so_far += UnicodeWidthChar::width(c).unwrap_or(0);
    }

    if !found_start {
        return "";
    }

    //println!("\rwidth_slice: {:?}[{}..{}] : {}..{}", txt, a, b, start_index, end_index);
    // txt.get(start_index..end_index).unwrap()
    unsafe { txt.get_unchecked(start_index..end_index) }
}

impl StyledText {
    pub fn new(style: Style, text: String) -> StyledText {
        if text.len() == 0 { panic!("Zero width StyledText"); }

        let width = UnicodeWidthStr::width(&text as &str);
        StyledText { style, text: Rc::new(text), width: width as _ }
    }

    pub fn slice<R: RangeBounds<usize>>(&self, r: R) -> StyledText where String: std::ops::Index<R> {
        let a = match r.start() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => i+1,
            std::ops::Bound::Unbounded => 0,
        };
        let b = match r.end() {
            std::ops::Bound::Included(i) => *i+1,
            std::ops::Bound::Excluded(i) => *i,
            std::ops::Bound::Unbounded => self.text.len(),
        };
        let n = StyledText::new(self.style, width_slice(&self.text, a, b).to_string());
        //println!("\n\rStyledText::slice: {}..{}  {:?}", a, b, n);
        n
    }
}

impl HasSize for StyledText {
    fn size(&self) -> (u32, u32) {
        (self.width, 1)
    }
}

impl PaintableWidget for StyledText {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32)) {
        target.draw_text_at(pos, &self)
    }
}

