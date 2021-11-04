use crate::common::Point;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range2 {
    pub x: (i32, i32),
    pub y: (i32, i32),
}

impl Range2 {
    pub fn contains(&self, p: Point) -> bool {
        self.x.0 <= p.x && p.x <= self.x.1 && self.y.0 <= p.y && p.y <= self.y.1
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
        assert!(r.contains(Point { x: 0, y: 0 }));

        let r = Range2 {
            x: (0, 5),
            y: (0, 10),
        };
        assert!(r.contains(Point { x: 3, y: 7 }));

        let r = Range2 {
            x: (-5, 5),
            y: (-5, 5),
        };
        assert!(r.contains(Point { x: 0, y: 0 }));

        let r = Range2 {
            x: (0, 5),
            y: (0, 10),
        };
        assert_eq!(r.contains(Point { x: 7, y: 3 }), false);

        let r = Range2 {
            x: (0, 10),
            y: (0, 5),
        };
        assert_eq!(r.contains(Point { x: 3, y: 7 }), false);

        let r = Range2 {
            x: (0, 10),
            y: (0, 10),
        };
        assert_eq!(r.contains(Point { x: -100, y: 5000 }), false);
    }
}
