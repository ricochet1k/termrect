
use std::rc::Rc;

use termrect::{RawPaintable, PaintableWidget, HasSize};
use style::Style;
use styledtext::StyledText;

#[derive(Debug, Clone)]
pub(crate) struct Line {
    // There are no gaps between these.
    texts: Vec<StyledText>,
}

impl Line {
    pub fn new(width: u32) -> Line {
        Line {
            texts: vec![
                StyledText {
                    style: Style::default(),
                    text: Rc::new(" ".repeat(width as usize)),
                    width: width,
                }
            ],
        }
    }
}

impl Line {
    pub(crate) fn draw_text_at(&mut self, x: u32, txt: &StyledText) {
        let txt_end = x + txt.width;

        let mut t_column = 0;
        let mut start_found = false;
        let mut start_sliced = None;
        let mut start_index = 0;
        let mut end_index = self.texts.len()-1;
        let mut end_sliced = None;
        for (i, t) in self.texts.iter().enumerate() {
            let t_end = t_column + t.width;
            if !start_found {
                if t_end > x {
                    start_index = i;
                    if t_column < x {
                        start_sliced = Some(t.slice(..(x as usize)-(t_column as usize)));
                    }
                    start_found = true;
                }
            }
            if start_found {
                if t_end >= txt_end {
                    end_index = i;
                    if txt_end < t_end {
                        end_sliced = Some(t.slice((txt_end as usize)-(t_column as usize)..));
                    }
                    break;
                }
            }
            t_column = t_end;
        }

        // start is out of bounds
        if !start_found { return }

        let repl = [start_sliced, Some(txt.clone()), end_sliced];
        let repl = repl.iter().flatten().cloned();

    //if *txt.text == "x" {
        //println!("\n\rLine.draw_text_at {}, {:?}: {}..{}", x, txt.text, start_index, end_index);
        //println!("\rLine.draw_text_at before: {:?}", self.texts);
    //}
        self.texts.splice(start_index..end_index+1, repl);
    //if *txt.text == "x" {
        //println!("\rLine.draw_text_at after: {:?}", self.texts);
    //}

        //self.texts = new_texts;
    }
}

impl HasSize for Line {
    fn size(&self) -> (u32, u32) {
        (self.texts.iter().map(|t| t.width).sum(), 1)
    }
}

impl PaintableWidget for Line {
    fn draw_into<R: RawPaintable>(&self, target: &mut R, pos: (u32, u32)) {
        let mut width_so_far = 0;
        for t in self.texts.iter() {
            t.draw_into(target, (pos.0 + width_so_far, pos.1));
            width_so_far += t.width;
        }
    }
    fn draw_delta_into<R: RawPaintable>(&mut self, target: &mut R, (x, y): (u32, u32)) {
        let mut width_so_far = 0;
        for t in self.texts.iter_mut() {
            t.draw_delta_into(target, (x + width_so_far, y));
            width_so_far += t.width;
        }
    }
}

