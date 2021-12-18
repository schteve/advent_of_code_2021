/*

*/

use crate::common::{signed, Point2, Range2};
use nom::{bytes::complete::tag, sequence::tuple, IResult};
use std::cmp::{max, Ordering};

// Not included in Range2 mod because the syntax for future range inputs is unlikely to be the same.
// Also, the syntax uses .. like Rust uses ..= which is just confusing.
fn range2_parser(input: &str) -> IResult<&str, Range2> {
    let (input, (_, x0, _, x1, _, y0, _, y1)) = tuple((
        tag("target area: x="),
        signed,
        tag(".."),
        signed,
        tag(", y="),
        signed,
        tag(".."),
        signed,
    ))(input)?;

    Ok((
        input,
        Range2 {
            x: (x0, x1),
            y: (y0, y1),
        },
    ))
}

struct Probe {
    position: Point2,
    velocity: Point2,
}

impl Probe {
    fn from_vel(velocity: Point2) -> Self {
        Self {
            position: Point2::origin(),
            velocity,
        }
    }

    fn simulate(&mut self, target: &Range2) -> Option<i32> {
        let mut highest_y = 0;
        while self.position.x <= target.x.1 && self.position.y >= target.y.0 {
            highest_y = max(highest_y, self.position.y);
            if target.contains(self.position) {
                return Some(highest_y);
            } else {
                self.position.x += self.velocity.x;
                match self.velocity.x.cmp(&0) {
                    Ordering::Less => self.velocity.x += 1,
                    Ordering::Greater => self.velocity.x -= 1,
                    _ => (),
                }

                self.position.y += self.velocity.y;
                self.velocity.y -= 1;
            }
        }
        None
    }
}

fn find_most_stylish(target: &Range2) -> i32 {
    let mut best = None;
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            let mut probe = Probe::from_vel(Point2 { x, y });
            let res = probe.simulate(target);
            if let Some(n) = res {
                if let Some(b) = best {
                    if n > b {
                        best = Some(n);
                    }
                } else {
                    best = Some(n);
                }
            }
        }
    }
    best.unwrap()
}

fn count_hits(target: &Range2) -> u32 {
    let mut count = 0;
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            let mut probe = Probe::from_vel(Point2 { x, y });
            let res = probe.simulate(target);
            if res.is_some() {
                count += 1;
            }
        }
    }
    count
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Range2 {
    range2_parser(input).unwrap().1
}

#[aoc(day17, part1)]
pub fn part1(input: &Range2) -> i32 {
    let best = find_most_stylish(input);
    assert_eq!(best, 5050);
    best
}

#[aoc(day17, part2)]
pub fn part2(input: &Range2) -> u32 {
    let count = count_hits(input);
    assert_eq!(count, 2223);
    count
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_probe_sim() {
        let target = input_generator(EXAMPLE_INPUT);

        let mut probe = Probe::from_vel(Point2 { x: 7, y: 2 });
        let res = probe.simulate(&target);
        assert_eq!(res, Some(3));
        assert_eq!(probe.position, Point2 { x: 28, y: -7 });

        let mut probe = Probe::from_vel(Point2 { x: 6, y: 3 });
        let res = probe.simulate(&target);
        assert_eq!(res, Some(6));
        assert_eq!(probe.position, Point2 { x: 21, y: -9 });

        let mut probe = Probe::from_vel(Point2 { x: 9, y: 0 });
        let res = probe.simulate(&target);
        assert_eq!(res, Some(0));
        assert_eq!(probe.position, Point2 { x: 30, y: -6 });

        let mut probe = Probe::from_vel(Point2 { x: 17, y: -4 });
        let res = probe.simulate(&target);
        assert_eq!(res, None);

        let mut probe = Probe::from_vel(Point2 { x: 6, y: 9 });
        let res = probe.simulate(&target);
        assert_eq!(res, Some(45));
    }

    #[test]
    fn test_find_most_stylish() {
        let target = input_generator(EXAMPLE_INPUT);
        let best = find_most_stylish(&target);
        assert_eq!(best, 45);
    }

    #[test]
    fn test_count_hits() {
        let target = input_generator(EXAMPLE_INPUT);
        let count = count_hits(&target);
        assert_eq!(count, 112);
    }
}
