/*
    --- Day 17: Trick Shot ---
    You finally decode the Elves' message. HI, the message says. You continue searching for the sleigh keys.

    Ahead of you is what appears to be a large ocean trench. Could the keys have fallen into it? You'd better send a probe to investigate.

    The probe launcher on your submarine can fire the probe with any integer velocity in the x (forward) and y (upward, or downward if negative) directions. For example, an initial x,y velocity like 0,10 would fire the probe straight up, while an initial velocity like 10,-1 would fire the probe forward at a slight downward angle.

    The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving in steps. On each step, these changes occur in the following order:

    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    Due to gravity, the probe's y velocity decreases by 1.
    For the probe to successfully make it into the trench, the probe must be on some trajectory that causes it to be within a target area after any step. The submarine computer has already calculated this target area (your puzzle input). For example:

    target area: x=20..30, y=-10..-5
    This target area means that you need to find initial x,y velocity values such that after any step, the probe's x position is at least 20 and at most 30, and the probe's y position is at least -10 and at most -5.

    Given this target area, one initial velocity that causes the probe to be within the target area after any step is 7,2:

    .............#....#............
    .......#..............#........
    ...............................
    S........................#.....
    ...............................
    ...............................
    ...........................#...
    ...............................
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTT#TT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    In this diagram, S is the probe's initial position, 0,0. The x coordinate increases to the right, and the y coordinate increases upward. In the bottom right, positions that are within the target area are shown as T. After each step (until the target area is reached), the position of the probe is marked with #. (The bottom-right # is both a position the probe reaches and a position in the target area.)

    Another initial velocity that causes the probe to be within the target area after any step is 6,3:

    ...............#..#............
    ...........#........#..........
    ...............................
    ......#..............#.........
    ...............................
    ...............................
    S....................#.........
    ...............................
    ...............................
    ...............................
    .....................#.........
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................T#TTTTTTTTT
    ....................TTTTTTTTTTT
    Another one is 9,0:

    S........#.....................
    .................#.............
    ...............................
    ........................#......
    ...............................
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTT#
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    ....................TTTTTTTTTTT
    One initial velocity that doesn't cause the probe to be within the target area after any step is 17,-4:

    S..............................................................
    ...............................................................
    ...............................................................
    ...............................................................
    .................#.............................................
    ....................TTTTTTTTTTT................................
    ....................TTTTTTTTTTT................................
    ....................TTTTTTTTTTT................................
    ....................TTTTTTTTTTT................................
    ....................TTTTTTTTTTT..#.............................
    ....................TTTTTTTTTTT................................
    ...............................................................
    ...............................................................
    ...............................................................
    ...............................................................
    ................................................#..............
    ...............................................................
    ...............................................................
    ...............................................................
    ...............................................................
    ...............................................................
    ...............................................................
    ..............................................................#
    The probe appears to pass through the target area, but is never within it after any step. Instead, it continues down and to the right - only the first few steps are shown.

    If you're going to fire a highly scientific probe out of a super cool probe launcher, you might as well do it with style. How high can you make the probe go while still reaching the target area?

    In the above example, using an initial velocity of 6,9 is the best you can do, causing the probe to reach a maximum y position of 45. (Any higher initial y velocity causes the probe to overshoot the target area entirely.)

    Find the initial velocity that causes the probe to reach the highest y position and still eventually be within the target area after any step. What is the highest y position it reaches on this trajectory?

    --- Part Two ---
    Maybe a fancy trick shot isn't the best idea; after all, you only have one probe, so you had better not miss.

    To get the best idea of what your options are for launching the probe, you need to find every initial velocity that causes the probe to eventually be within the target area after any step.

    In the above example, there are 112 different initial velocity values that meet these criteria:

    23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
    25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
    8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
    26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
    20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
    25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
    25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
    8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
    24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
    7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
    23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
    27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
    8,-2    27,-8   30,-5   24,-7
    How many distinct initial velocity values cause the probe to be within the target area after any step?
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
    let target_x_max = max(target.x.0.abs(), target.x.1.abs());
    let target_y_max = max(target.y.0.abs(), target.y.1.abs());
    for x in -target_x_max..=target_x_max {
        for y in -target_y_max..=target_y_max {
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
    let target_x_max = max(target.x.0.abs(), target.x.1.abs());
    let target_y_max = max(target.y.0.abs(), target.y.1.abs());
    for x in -target_x_max..=target_x_max {
        for y in -target_y_max..=target_y_max {
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
