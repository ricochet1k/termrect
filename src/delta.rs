use std::borrow::Borrow;
use std::ops::Range;

/// Delta tracks changes to a list.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Delta {
    Unchanged,
    Range(u32, u32),
}
use delta::Delta::*;

impl Delta {
    pub fn add(&mut self, i: u32) {
        *self = match *self {
            Unchanged => Range(i, i + 1),
            Range(a, b) => Range(a.min(i), b.max(i + 1)),
        }
    }

    pub fn add_range<R: Borrow<Range<u32>>>(&mut self, r: R) {
        let r = r.borrow();
        *self = match *self {
            Unchanged => Range(r.start, r.end),
            Range(a, b) => Range(a.min(r.start), b.max(r.end)),
        }
    }

    pub fn contains(&self, i: u32) -> bool {
        match *self {
            Unchanged => false,
            Range(a, b) => a <= i && i < b,
        }
    }

    pub fn to_range(&self) -> Range<u32> {
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
}
