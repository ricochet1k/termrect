use std::borrow::Borrow;
use std::ops::Range;

/// Delta tracks changes to a list.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Delta {
    Unchanged,
    Range(usize, usize),
}
use delta::Delta::*;

impl Delta {
    pub fn add(&mut self, i: usize) {
        *self = match *self {
            Unchanged => Range(i, i + 1),
            Range(a, b) => Range(a.min(i), b.max(i + 1)),
        }
    }

    pub fn add_range<R: Borrow<Range<usize>>>(&mut self, r: R) {
        let r = r.borrow();
        *self = match *self {
            Unchanged => Range(r.start, r.end),
            Range(a, b) => Range(a.min(r.start), b.max(r.end)),
        }
    }

    /// add_splice_range is to be used in conjuction with std::vec::Vec.splice.
    /// Pass the same range and the length of the replace_width.
    pub fn add_splice_range<R: Borrow<Range<usize>>>(&mut self, r: R, len: usize) {
        let r = r.borrow();
        let len = len as usize;
        *self = match *self {
            Unchanged => Range(r.start, r.start + len),
            Range(a, b) => {
                let a = if a <= r.start {
                    a
                } else if a < r.end {
                    r.start
                } else {
                    a + len - (r.end - r.start)
                };
                let b = if b <= r.start {
                    b
                } else if b < r.end {
                    r.start + len
                } else {
                    b + len - (r.end - r.start)
                };
                Range(a.min(r.start), b.max(r.start + len))
            }
        }
    }

    pub fn contains(&self, i: usize) -> bool {
        match *self {
            Unchanged => false,
            Range(a, b) => a <= i && i < b,
        }
    }

    pub fn to_range(&self) -> Range<usize> {
        match *self {
            Unchanged => Range { start: 0, end: 0 },
            Range(a, b) => Range { start: a, end: b },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_delta() {
        let mut d = Unchanged;
        assert_eq!(d.to_range().len(), 0);
        assert!(!d.contains(0));
        assert!(!d.contains(1));
        assert!(!d.contains(2));

        d.add(1);
        assert_eq!(d.to_range().len(), 1);
        assert!(!d.contains(0));
        assert!(d.contains(1));
        assert!(!d.contains(2));
        assert!(!d.contains(3));

        d.add(3);
        assert!(d.to_range().len() >= 2);
        assert!(d.contains(3));

        let r = 4..7;
        for i in r.clone() {
            assert!(!d.contains(i));
        }
        d.add_range(r.clone());
        for i in r {
            assert!(d.contains(i));
        }
    }

    #[test]
    fn test_delta_splice() {
        // insert right of delta
        let mut d = Range(2, 3);
        d.add_splice_range(4..4, 3);
        assert_eq!(d, Range(2, 7));

        // delete right of delta
        let mut d = Range(1, 2);
        d.add_splice_range(2..3, 0);
        assert_eq!(d, Range(1, 2));

        // insert left of delta
        let mut d = Range(2, 3);
        d.add_splice_range(0..0, 3);
        assert_eq!(d, Range(0, 6));

        // delete left of delta
        let mut d = Range(2, 3);
        d.add_splice_range(0..2, 1);
        assert_eq!(d, Range(0, 2));

        // insert inside delta
        let mut d = Range(2, 5);
        d.add_splice_range(3..3, 3);
        assert_eq!(d, Range(2, 8));

        // delete inside delta
        let mut d = Range(2, 5);
        d.add_splice_range(3..4, 0);
        assert_eq!(d, Range(2, 4));

        // modify intersecting right of delta
        let mut d = Range(2, 5);
        d.add_splice_range(4..6, 3);
        assert_eq!(d, Range(2, 7));

        // modify intersecting left of delta
        let mut d = Range(2, 5);
        d.add_splice_range(1..2, 2);
        assert_eq!(d, Range(1, 6));
    }
}
