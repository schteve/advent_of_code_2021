use crate::common::Point3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range3 {
    pub x: (i32, i32),
    pub y: (i32, i32),
    pub z: (i32, i32),
}

impl Range3 {
    pub fn contains(&self, p: Point3) -> bool {
        self.x.0 <= p.x
            && p.x <= self.x.1
            && self.y.0 <= p.y
            && p.y <= self.y.1
            && self.z.0 <= p.z
            && p.z <= self.z.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let r = Range3 {
            x: (0, 0),
            y: (0, 0),
            z: (0, 0),
        };
        assert!(r.contains(Point3 { x: 0, y: 0, z: 0 }));

        let r = Range3 {
            x: (0, 5),
            y: (0, 10),
            z: (0, 15),
        };
        assert!(r.contains(Point3 { x: 3, y: 7, z: 0 }));

        let r = Range3 {
            x: (-5, 5),
            y: (-5, 5),
            z: (-5, 5),
        };
        assert!(r.contains(Point3 { x: 0, y: 0, z: 0 }));

        let r = Range3 {
            x: (0, 5),
            y: (0, 10),
            z: (0, 15),
        };
        assert_eq!(r.contains(Point3 { x: 7, y: 3, z: 0 }), false);

        let r = Range3 {
            x: (0, 5),
            y: (0, 10),
            z: (0, 15),
        };
        assert_eq!(r.contains(Point3 { x: 3, y: 12, z: 0 }), false);

        let r = Range3 {
            x: (0, 5),
            y: (0, 10),
            z: (0, 15),
        };
        assert_eq!(r.contains(Point3 { x: 3, y: 7, z: 21 }), false);

        let r = Range3 {
            x: (0, 10),
            y: (0, 10),
            z: (0, 10),
        };
        assert_eq!(
            r.contains(Point3 {
                x: -100,
                y: 5000,
                z: 123456789
            }),
            false
        );
    }
}
