use crate::common::{signed, Range3};
use auto_ops::*;
use nom::{
    character::complete::{char, space0},
    combinator::{cond, opt},
    error::Error,
    sequence::{pair, preceded, tuple},
    Finish, IResult,
};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub const fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        // Accepts the following, with whitespace allowed anywhere:
        //  123,456,789
        //  (123,456,789)
        let (input, _) = space0(input)?;
        let (input, open_paren) = opt(char('('))(input)?;
        let (input, (x, _, y, _, z)) = preceded(
            space0,
            tuple((
                signed,
                tuple((space0, char(','), space0)),
                signed,
                tuple((space0, char(','), space0)),
                signed,
            )),
        )(input)?;
        let (input, _) = cond(open_paren.is_some(), pair(space0, char(')')))(input)?;

        Ok((input, Self { x, y, z }))
    }

    pub fn manhattan(a: Self, b: Self) -> u32 {
        let delta = a - b;
        let distance = delta.x.abs() + delta.y.abs() + delta.z.abs();
        distance as u32
    }

    pub fn cmp_xyz(a: &Self, b: &Self) -> Ordering {
        let compare = a.x.cmp(&b.x);
        if compare != Ordering::Equal {
            return compare;
        }

        let compare = a.y.cmp(&b.y);
        if compare != Ordering::Equal {
            return compare;
        }

        a.z.cmp(&b.z)
    }

    pub fn cmp_zyx(a: &Self, b: &Self) -> Ordering {
        let compare = a.z.cmp(&b.z);
        if compare != Ordering::Equal {
            return compare;
        }

        let compare = a.y.cmp(&b.y);
        if compare != Ordering::Equal {
            return compare;
        }

        a.x.cmp(&b.x)
    }

    pub fn get_range<'a, I>(values: I) -> Option<Range3>
    where
        I: std::iter::IntoIterator<Item = &'a Self>, // Using IntoIterator instead of Iterator allows the user to pass either an iterator or something that can be turned into one
    {
        let mut point_iter = values.into_iter(); // Note: if 'values' is an Iterator it will just return itself here
        if let Some(point) = point_iter.next() {
            let range = point_iter.fold(
                Range3 {
                    x: (point.x, point.x),
                    y: (point.y, point.y),
                    z: (point.z, point.z),
                },
                |acc, p| {
                    let x = (acc.x.0.min(p.x), acc.x.1.max(p.x));
                    let y = (acc.y.0.min(p.y), acc.y.1.max(p.y));
                    let z = (acc.z.0.min(p.z), acc.z.1.max(p.z));
                    Range3 { x, y, z }
                },
            );
            Some(range)
        } else {
            None
        }
    }
}

impl From<(i32, i32, i32)> for Point3 {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl_op_ex!(+ |a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex_commutative!(+ |a: &Point3, b: &(i32, i32, i32)| -> Point3 {
    Point3 {
        x: a.x + b.0,
        y: a.y + b.1,
        z: a.z + b.2,
    }
});

impl_op_ex!(+= |a: &mut Point3, b: &Point3| { *a = *a + b });
impl_op_ex!(+= |a: &mut Point3, b: &(i32, i32, i32)| { *a = *a + b });

impl_op_ex!(-|a: &Point3, b: &Point3| -> Point3 {
    Point3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(-|a: &Point3, b: &(i32, i32, i32)| -> Point3 {
    Point3 {
        x: a.x - b.0,
        y: a.y - b.1,
        z: a.z - b.2,
    }
});

impl_op_ex!(-= |a: &mut Point3, b: &Point3| { *a = *a - b });
impl_op_ex!(-= |a: &mut Point3, b: &(i32, i32, i32)| { *a = *a - b });

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::str::FromStr for Point3 {
    type Err = Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Point3::parser(s).finish() {
            Ok((_remaining, point)) => Ok(point),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl PartialOrd for Point3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point3 {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp_zyx(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(
            "123,456,789".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            " 123,456,789".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "123 ,456,789".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "123, 456,789".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "123,456,789 ".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            " 123 , 456,789 ".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "(123,456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            " (123,456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "( 123,456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "(123 ,456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "(123, 456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "(123,456,789 )".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "(123,456,789) ".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "( 123 , 456,789 )".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
        assert_eq!(
            "123,456,789)".parse(),
            Ok(Point3 {
                x: 123,
                y: 456,
                z: 789
            })
        );
    }

    #[test]
    #[should_panic]
    fn test_from_string_fail() {
        "(123,456,789".parse::<Point3>().unwrap();
    }

    #[test]
    fn test_manhattan() {
        let a = Point3 { x: 0, y: 0, z: 0 };
        let b = Point3 { x: 0, y: 0, z: 0 };
        assert_eq!(Point3::manhattan(a, b), 0);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 3, y: 4, z: 6 };
        assert_eq!(Point3::manhattan(a, b), 5);

        let a = Point3 {
            x: -1,
            y: -2,
            z: -5,
        };
        let b = Point3 { x: 3, y: 4, z: 6 };
        assert_eq!(Point3::manhattan(a, b), 21);
    }

    #[test]
    fn test_cmp_xyz() {
        let a = Point3 { x: 0, y: 0, z: 0 };
        let b = Point3 { x: 0, y: 0, z: 0 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Equal);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 3, y: 4, z: 6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 3, y: -5, z: 6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 1, y: 4, z: 6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 1, y: -5, z: 6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Greater);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 1, y: 2, z: 6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 1, y: 2, z: -6 };
        assert_eq!(Point3::cmp_xyz(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_cmp_y_x() {
        let a = Point3 { x: 0, y: 0, z: 0 };
        let b = Point3 { x: 0, y: 0, z: 0 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Equal);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 3, y: 4, z: 6 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 3, y: -5, z: 2 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Greater);

        let a = Point3 { x: 2, y: 1, z: 5 };
        let b = Point3 { x: 4, y: 1, z: 6 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Less);

        let a = Point3 { x: 2, y: 1, z: 5 };
        let b = Point3 { x: -5, y: 1, z: 2 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Greater);

        let a = Point3 { x: 2, y: 1, z: 5 };
        let b = Point3 { x: 1, y: 1, z: 6 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Less);

        let a = Point3 { x: 1, y: 2, z: 5 };
        let b = Point3 { x: 1, y: 2, z: -6 };
        assert_eq!(Point3::cmp_zyx(&a, &b), Ordering::Greater);
    }

    #[test]
    fn test_add() {
        let a = Point3 { x: 0, y: 0, z: 0 };
        let b = Point3 { x: 0, y: 0, z: 0 };
        assert_eq!(a + b, Point3 { x: 0, y: 0, z: 0 });

        let a = Point3 { x: 1, y: 1, z: 1 };
        let b = Point3 { x: 2, y: 2, z: 2 };
        assert_eq!(a + b, Point3 { x: 3, y: 3, z: 3 });

        let a = Point3 { x: 1, y: 1, z: 1 };
        let b = Point3 {
            x: -1,
            y: -1,
            z: -1,
        };
        assert_eq!(a + b, Point3 { x: 0, y: 0, z: 0 });

        let a = Point3 {
            x: 1_000_000_000,
            y: -1_000_000_000,
            z: 1_000_000_000,
        };
        let b = Point3 { x: -1, y: 1, z: 1 };
        assert_eq!(
            a + b,
            Point3 {
                x: 999_999_999,
                y: -999_999_999,
                z: 1_000_000_001,
            }
        );
    }

    #[test]
    fn test_sub() {
        let a = Point3 { x: 0, y: 0, z: 0 };
        let b = Point3 { x: 0, y: 0, z: 0 };
        assert_eq!(a - b, Point3 { x: 0, y: 0, z: 0 });

        let a = Point3 { x: 1, y: 1, z: 1 };
        let b = Point3 { x: 2, y: 2, z: 2 };
        assert_eq!(
            a - b,
            Point3 {
                x: -1,
                y: -1,
                z: -1
            }
        );

        let a = Point3 { x: 1, y: 1, z: 1 };
        let b = Point3 {
            x: -1,
            y: -1,
            z: -1,
        };
        assert_eq!(a - b, Point3 { x: 2, y: 2, z: 2 });

        let a = Point3 {
            x: 1_000_000_000,
            y: -1_000_000_000,
            z: 1_000_000_000,
        };
        let b = Point3 { x: -1, y: 1, z: 1 };
        assert_eq!(
            a - b,
            Point3 {
                x: 1_000_000_001,
                y: -1_000_000_001,
                z: 999_999_999,
            }
        );

        let a = Point3 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
            z: 0x7FFFFFFF,
        };
        let b = Point3 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
            z: 0x7FFFFFFF,
        };
        let c = Point3 {
            x: 0x7FFFFFFF,
            y: -0x7FFFFFFF,
            z: 0x7FFFFFFF,
        };
        assert_eq!(
            a - b - c,
            Point3 {
                x: -0x7FFFFFFF,
                y: 0x7FFFFFFF,
                z: -0x7FFFFFFF,
            }
        );
    }

    #[test]
    fn test_get_range() {
        let points = Vec::new();
        let range = Point3::get_range(&points);
        assert_eq!(range, None);

        let points = vec![Point3 { x: 0, y: 0, z: 0 }];
        let range = Point3::get_range(&points);
        assert_eq!(
            range,
            Some(Range3 {
                x: (0, 0),
                y: (0, 0),
                z: (0, 0),
            })
        );

        let points = vec![
            Point3 { x: -5, y: 0, z: 1 },
            Point3 { x: 0, y: 7, z: 5 },
            Point3 { x: 4, y: 4, z: -3 },
        ];
        let range = Point3::get_range(&points);
        assert_eq!(
            range,
            Some(Range3 {
                x: (-5, 4),
                y: (0, 7),
                z: (-3, 5),
            })
        );

        let points = vec![
            Point3 {
                x: 24,
                y: -86,
                z: -37,
            },
            Point3 {
                x: -80,
                y: 33,
                z: 10,
            },
            Point3 {
                x: 16,
                y: -81,
                z: 97,
            },
            Point3 {
                x: 59,
                y: 14,
                z: -46,
            },
            Point3 {
                x: -97,
                y: -7,
                z: 32,
            },
            Point3 {
                x: 73,
                y: -40,
                z: -90,
            },
            Point3 {
                x: 16,
                y: -29,
                z: -96,
            },
            Point3 {
                x: 5,
                y: 69,
                z: -70,
            },
            Point3 { x: 2, y: 22, z: 2 },
        ];
        let range = Point3::get_range(&points);
        assert_eq!(
            range,
            Some(Range3 {
                x: (-97, 73),
                y: (-86, 69),
                z: (-96, 97),
            })
        );

        let points = vec![
            Point3 {
                x: 311147,
                y: 388530,
                z: 785954,
            },
            Point3 {
                x: 459992,
                y: 742648,
                z: 811715,
            },
            Point3 {
                x: 307738,
                y: 247421,
                z: -255643,
            },
            Point3 {
                x: 132960,
                y: 182207,
                z: -752108,
            },
            Point3 {
                x: 822741,
                y: 727272,
                z: -659351,
            },
            Point3 {
                x: 979388,
                y: 603831,
                z: -453137,
            },
            Point3 {
                x: 784738,
                y: 563251,
                z: 142567,
            },
            Point3 {
                x: 696914,
                y: 315058,
                z: 42110,
            },
            Point3 {
                x: 449283,
                y: 180916,
                z: 552886,
            },
        ];
        let range = Point3::get_range(&points);
        assert_eq!(
            range,
            Some(Range3 {
                x: (132960, 979388),
                y: (180916, 742648),
                z: (-752108, 811715),
            })
        );
    }
}
