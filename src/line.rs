use std::rc::Rc;

use style::Style;
use styledtext::StyledText;
use termrect::{HasSize, PaintableWidget, RawPaintable};

#[derive(Debug, Clone)]
pub(crate) struct Line {
    // There are no gaps between these.
    texts: Vec<StyledText>,
}

impl Line {
    pub fn new(width: u32) -> Line {
        Line {
            texts: vec![StyledText {
                style: Style::default(),
                text: Rc::new(" ".repeat(width as usize)),
                width: width,
            }],
        }
    }
}

impl Line {
    pub(crate) fn draw_text_at(&mut self, x: u32, txt: &StyledText) {
        let txt_end = x + txt.width;

        let mut t_column;
        let mut t_end = 0;
        let mut start_found = false;
        let mut start_sliced = None;
        let mut start_index = 0;
        let mut end_index = self.texts.len() - 1;
        let mut end_sliced = None;
        for (i, t) in self.texts.iter().enumerate() {
            t_column = t_end;
            t_end = t_column + t.width;
            if !start_found {
                if t_end > x {
                    start_index = i;
                    if t_column < x {
                        start_sliced = Some(t.slice(..(x as usize) - (t_column as usize)));
                    }
                    start_found = true;
                }
            }
            if start_found {
                if t_end >= txt_end {
                    end_index = i;
                    if txt_end < t_end {
                        end_sliced = Some(t.slice((txt_end as usize) - (t_column as usize)..));
                    }
                    break;
                }
            }
        }

        // start is out of bounds
        if !start_found {
            return;
        }

        let text = if end_index == self.texts.len() - 1 && txt_end > t_end {
            txt.slice(..(txt.width - (txt_end - t_end)) as usize)
        } else {
            txt.clone()
        };

        let repl = [start_sliced, Some(text), end_sliced];
        let repl = repl.iter().flatten().cloned();

        self.texts.splice(start_index..end_index + 1, repl);
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

#[cfg(test)]
mod test {
    use super::*;

    fn strings_of(line: &Line) -> Vec<&str> {
        line.texts.iter().map(|t| &*t.text as &str).collect()
    }

    #[test]
    fn draw_text() {
        let mut line = Line::new(10);

        assert_eq!(strings_of(&line), vec!["          "]);

        line.draw_text_at(1, &StyledText::new(Style::default(), "a".to_string()));

        assert_eq!(strings_of(&line), vec![" ", "a", "        "]);

        line.draw_text_at(0, &StyledText::new(Style::default(), "xxx".to_string()));

        assert_eq!(strings_of(&line), vec!["xxx", "       "]);

        // chop off anything that extends past the end of the line
        line.draw_text_at(9, &StyledText::new(Style::default(), "123".to_string()));
        assert_eq!(strings_of(&line), vec!["xxx", "      ", "1"]);

        line.draw_text_at(12, &StyledText::new(Style::default(), "123".to_string()));
        assert_eq!(strings_of(&line), vec!["xxx", "      ", "1"]);
    }
}
