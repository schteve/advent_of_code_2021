use std::cmp::{max, min};

use crate::common::Point2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range2 {
    pub x: (i32, i32),
    pub y: (i32, i32),
}

impl Range2 {
    pub const fn origin() -> Self {
        Self {
            x: (0, 0),
            y: (0, 0),
        }
    }

    pub fn contains(&self, p: Point2) -> bool {
        self.x.0 <= p.x && p.x <= self.x.1 && self.y.0 <= p.y && p.y <= self.y.1
    }

    pub fn expand(&mut self, p: &Point2) {
        self.x.0 = min(self.x.0, p.x);
        self.x.1 = max(self.x.1, p.x);
        self.y.0 = min(self.y.0, p.y);
        self.y.1 = max(self.y.1, p.y);
    }
}

impl std::fmt::Display for Range2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x={}..={}, y={}..={})",
            self.x.0, self.x.1, self.y.0, self.y.1
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let r = Range2 {
            x: (0, 0),
            y: (0, 0),
        };
        assert!(r.contains(Point2 { x: 0, y: 0 }));

        let r = Range2 {
            x: (0, 5),
            y: (0, 10),
        };
        assert!(r.contains(Point2 { x: 3, y: 7 }));

        let r = Range2 {
            x: (-5, 5),
            y: (-5, 5),
        };
        assert!(r.contains(Point2 { x: 0, y: 0 }));

        let r = Range2 {
            x: (0, 5),
            y: (0, 10),
        };
        assert_eq!(r.contains(Point2 { x: 7, y: 3 }), false);

        let r = Range2 {
            x: (0, 10),
            y: (0, 5),
        };
        assert_eq!(r.contains(Point2 { x: 3, y: 7 }), false);

        let r = Range2 {
            x: (0, 10),
            y: (0, 10),
        };
        assert_eq!(r.contains(Point2 { x: -100, y: 5000 }), false);
    }
}
