use crate::common::{signed, Cardinal, Range2};
use auto_ops::*;
use nom::{
    character::complete::{char, space0},
    combinator::{cond, opt},
    error::Error,
    sequence::{pair, preceded, separated_pair, tuple},
    Finish, IResult,
};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        // Accepts the following, with whitespace allowed anywhere:
        //  123,456
        //  (123,456)
        let (input, _) = space0(input)?;
        let (input, open_paren) = opt(char('('))(input)?;
        let (input, (x, y)) = preceded(
            space0,
            separated_pair(signed, tuple((space0, char(','), space0)), signed),
        )(input)?;
        let (input, _) = cond(open_paren.is_some(), pair(space0, char(')')))(input)?;

        Ok((input, Self { x, y }))
    }

    pub fn manhattan(a: Self, b: Self) -> u32 {
        let delta = a - b;
        let distance = delta.x.abs() + delta.y.abs();
        distance as u32
    }

    pub fn cmp_xy(a: &Self, b: &Self) -> Ordering {
        let compare = a.x.cmp(&b.x);
        if compare == Ordering::Equal {
            a.y.cmp(&b.y)
        } else {
            compare
        }
    }

    pub fn cmp_yx(a: &Self, b: &Self) -> Ordering {
        let compare = a.y.cmp(&b.y);
        if compare == Ordering::Equal {
            a.x.cmp(&b.x)
        } else {
            compare
        }
    }

    pub fn step(&self, direction: Cardinal, count: i32) -> Self {
        match direction {
            Cardinal::North => *self + (0, -count),
            Cardinal::South => *self + (0, count),
            Cardinal::East => *self + (count, 0),
            Cardinal::West => *self + (-count, 0),
        }
    }

    const ORTHOGONALS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    pub fn orthogonals(&self) -> impl Iterator<Item = Self> + '_ {
        Self::ORTHOGONALS.into_iter().map(move |p| self + p)
    }

    const DIAGONALS: [(i32, i32); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    pub fn diagonals(&self) -> impl Iterator<Item = Self> + '_ {
        Self::DIAGONALS.into_iter().map(move |p| self + p)
    }

    pub fn adjacents(&self) -> impl Iterator<Item = Self> + '_ {
        self.orthogonals().chain(self.diagonals())
    }

    pub fn get_range<'a, I>(values: I) -> Option<Range2>
    where
        I: std::iter::IntoIterator<Item = &'a Self>, // Using IntoIterator instead of Iterator allows the user to pass either an iterator or something that can be turned into one
    {
        let mut point_iter = values.into_iter(); // Note: if 'values' is an Iterator it will just return itself here
        if let Some(point) = point_iter.next() {
            let range = point_iter.fold(
                Range2 {
                    x: (point.x, point.x),
                    y: (point.y, point.y),
                },
                |acc, p| {
                    let x = (acc.x.0.min(p.x), acc.x.1.max(p.x));
                    let y = (acc.y.0.min(p.y), acc.y.1.max(p.y));
                    Range2 { x, y }
                },
            );
            Some(range)
        } else {
            None
        }
    }
}

impl From<(i32, i32)> for Point2 {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl_op_ex!(+ |a: &Point2, b: &Point2| -> Point2 {
    Point2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});

impl_op_ex_commutative!(+ |a: &Point2, b: &(i32, i32)| -> Point2 {
    Point2 {
        x: a.x + b.0,
        y: a.y + b.1,
    }
});

impl_op_ex!(+= |a: &mut Point2, b: &Point2| { *a = *a + b });
impl_op_ex!(+= |a: &mut Point2, b: &(i32, i32)| { *a = *a + b });

impl_op_ex!(-|a: &Point2, b: &Point2| -> Point2 {
    Point2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(-|a: &Point2, b: &(i32, i32)| -> Point2 {
    Point2 {
        x: a.x - b.0,
        y: a.y - b.1,
    }
});

impl_op_ex!(-= |a: &mut Point2, b: &Point2| { *a = *a - b });
impl_op_ex!(-= |a: &mut Point2, b: &(i32, i32)| { *a = *a - b });

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::str::FromStr for Point2 {
    type Err = Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Point2::parser(s).finish() {
            Ok((_remaining, point)) => Ok(point),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2 {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp_yx(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!("123,456".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!(" 123,456".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("123 ,456".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("123, 456".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("123,456 ".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!(" 123 , 456 ".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("(123,456)".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!(" (123,456)".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("( 123,456)".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("(123 ,456)".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("(123, 456)".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("(123,456 )".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("(123,456) ".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("( 123 , 456 )".parse(), Ok(Point2 { x: 123, y: 456 }));
        assert_eq!("123,456)".parse(), Ok(Point2 { x: 123, y: 456 }));
    }

    #[test]
    #[should_panic]
    fn test_from_string_fail() {
        "(123,456".parse::<Point2>().unwrap();
    }

    #[test]
    fn test_manhattan() {
        let a = Point2 { x: 0, y: 0 };
        let b = Point2 { x: 0, y: 0 };
        assert_eq!(Point2::manhattan(a, b), 0);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 3, y: 4 };
        assert_eq!(Point2::manhattan(a, b), 4);

        let a = Point2 { x: -1, y: -2 };
        let b = Point2 { x: 3, y: 4 };
        assert_eq!(Point2::manhattan(a, b), 10);
    }

    #[test]
    fn test_cmp_x_y() {
        let a = Point2 { x: 0, y: 0 };
        let b = Point2 { x: 0, y: 0 };
        assert_eq!(Point2::cmp_xy(&a, &b), Ordering::Equal);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 3, y: 4 };
        assert_eq!(Point2::cmp_xy(&a, &b), Ordering::Less);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 3, y: -5 };
        assert_eq!(Point2::cmp_xy(&a, &b), Ordering::Less);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 1, y: 4 };
        assert_eq!(Point2::cmp_xy(&a, &b), Ordering::Less);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 1, y: -5 };
        assert_eq!(Point2::cmp_xy(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_cmp_y_x() {
        let a = Point2 { x: 0, y: 0 };
        let b = Point2 { x: 0, y: 0 };
        assert_eq!(Point2::cmp_yx(&a, &b), Ordering::Equal);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 3, y: 4 };
        assert_eq!(Point2::cmp_yx(&a, &b), Ordering::Less);

        let a = Point2 { x: 1, y: 2 };
        let b = Point2 { x: 3, y: -5 };
        assert_eq!(Point2::cmp_yx(&a, &b), Ordering::Greater);

        let a = Point2 { x: 2, y: 1 };
        let b = Point2 { x: 4, y: 1 };
        assert_eq!(Point2::cmp_yx(&a, &b), Ordering::Less);

        let a = Point2 { x: 2, y: 1 };
        let b = Point2 { x: -5, y: 1 };
        assert_eq!(Point2::cmp_yx(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_add() {
        let a = Point2 { x: 0, y: 0 };
        let b = Point2 { x: 0, y: 0 };
        assert_eq!(a + b, Point2 { x: 0, y: 0 });

        let a = Point2 { x: 1, y: 1 };
        let b = Point2 { x: 2, y: 2 };
        assert_eq!(a + b, Point2 { x: 3, y: 3 });

        let a = Point2 { x: 1, y: 1 };
        let b = Point2 { x: -1, y: -1 };
        assert_eq!(a + b, Point2 { x: 0, y: 0 });

        let a = Point2 {
            x: 1_000_000_000,
            y: -1_000_000_000,
        };
        let b = Point2 { x: -1, y: 1 };
        assert_eq!(
            a + b,
            Point2 {
                x: 999_999_999,
                y: -999_999_999,
            }
        );
    }

    #[test]
    fn test_sub() {
        let a = Point2 { x: 0, y: 0 };
        let b = Point2 { x: 0, y: 0 };
        assert_eq!(a - b, Point2 { x: 0, y: 0 });

        let a = Point2 { x: 1, y: 1 };
        let b = Point2 { x: 2, y: 2 };
        assert_eq!(a - b, Point2 { x: -1, y: -1 });

        let a = Point2 { x: 1, y: 1 };
        let b = Point2 { x: -1, y: -1 };
        assert_eq!(a - b, Point2 { x: 2, y: 2 });

        let a = Point2 {
            x: 1_000_000_000,
            y: -1_000_000_000,
        };
        let b = Point2 { x: -1, y: 1 };
        assert_eq!(
            a - b,
            Point2 {
                x: 1_000_000_001,
                y: -1_000_000_001,
            }
        );

        let a = Point2 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
        };
        let b = Point2 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
        };
        let c = Point2 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
        };
        assert_eq!(
            a - b - c,
            Point2 {
                x: -0x7FFFFFFF,
                y: 0x7FFFFFFF,
            }
        );
    }

    #[test]
    fn test_get_range() {
        let points = Vec::new();
        let range = Point2::get_range(&points);
        assert_eq!(range, None);

        let points = [Point2 { x: 0, y: 0 }];
        let range = Point2::get_range(&points);
        assert_eq!(
            range,
            Some(Range2 {
                x: (0, 0),
                y: (0, 0),
            })
        );

        let points = [
            Point2 { x: -5, y: 0 },
            Point2 { x: 0, y: 7 },
            Point2 { x: 4, y: 4 },
        ];
        let range = Point2::get_range(&points);
        assert_eq!(
            range,
            Some(Range2 {
                x: (-5, 4),
                y: (0, 7),
            })
        );

        let points = [
            Point2 { x: 24, y: -86 },
            Point2 { x: -80, y: 33 },
            Point2 { x: 16, y: -81 },
            Point2 { x: 59, y: 14 },
            Point2 { x: -97, y: -7 },
            Point2 { x: 73, y: -40 },
            Point2 { x: 16, y: -29 },
            Point2 { x: 5, y: 69 },
            Point2 { x: 2, y: 22 },
        ];
        let range = Point2::get_range(&points);
        assert_eq!(
            range,
            Some(Range2 {
                x: (-97, 73),
                y: (-86, 69),
            })
        );

        let points = [
            Point2 {
                x: 311147,
                y: 388530,
            },
            Point2 {
                x: 459992,
                y: 742648,
            },
            Point2 {
                x: 307738,
                y: 247421,
            },
            Point2 {
                x: 132960,
                y: 182207,
            },
            Point2 {
                x: 822741,
                y: 727272,
            },
            Point2 {
                x: 979388,
                y: 603831,
            },
            Point2 {
                x: 784738,
                y: 563251,
            },
            Point2 {
                x: 696914,
                y: 315058,
            },
            Point2 {
                x: 449283,
                y: 180916,
            },
        ];
        let range = Point2::get_range(&points);
        assert_eq!(
            range,
            Some(Range2 {
                x: (132960, 979388),
                y: (180916, 742648),
            })
        );
    }
}
