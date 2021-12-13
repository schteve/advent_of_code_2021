/*
    --- Day 13: Transparent Origami ---
    You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.

    Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:

    Congratulations on your purchase! To activate this infrared thermal imaging
    camera system, please enter the code found on page 1 of the manual.
    Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:

    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
    The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:

    ...#..#..#.
    ....#......
    ...........
    #..........
    ...#....#.#
    ...........
    ...........
    ...........
    ...........
    ...........
    .#....#.##.
    ....#......
    ......#...#
    #..........
    #.#........
    Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):

    ...#..#..#.
    ....#......
    ...........
    #..........
    ...#....#.#
    ...........
    ...........
    -----------
    ...........
    ...........
    .#....#.##.
    ....#......
    ......#...#
    #..........
    #.#........
    Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:

    #.##..#..#.
    #...#......
    ......#...#
    #...#......
    .#.#..#.###
    ...........
    ...........
    Now, only 17 dots are visible.

    Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.

    Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.

    The second fold instruction is fold along x=5, which indicates this line:

    #.##.|#..#.
    #...#|.....
    .....|#...#
    #...#|.....
    .#.#.|#.###
    .....|.....
    .....|.....
    Because this is a vertical line, fold left:

    #####
    #...#
    #...#
    #...#
    #####
    .....
    .....
    The instructions made a square!

    The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.

    How many dots are visible after completing just the first fold instruction on your transparent paper?

    --- Part Two ---
    Finish folding the transparent paper according to the instructions. The manual says the code is always eight capital letters.

    What code do you use to activate the infrared thermal imaging camera system?
*/

use crate::common::{unsigned, Point2};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (_, _, dim, _, val)): (_, (_, _, _, _, i32)) =
            tuple((multispace0, tag("fold along "), alpha1, char('='), unsigned))(input)?;

        let fold = match dim {
            "x" => Self::X(val as i32),
            "y" => Self::Y(val as i32),
            _ => panic!("Invalid dimension: {}", dim),
        };

        Ok((input, fold))
    }
}

#[derive(Clone)]
pub struct Paper {
    dots: HashSet<Point2>,
    folds: Vec<Fold>,
    blank_char: char,
}

impl Paper {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, dots_list) = many1(preceded(multispace0, Point2::parser))(input)?;
        let (input, folds) = many1(Fold::parser)(input)?;

        let dots = dots_list.into_iter().collect();

        Ok((
            input,
            Self {
                dots,
                folds,
                blank_char: '.',
            },
        ))
    }

    fn fold_next(&mut self) -> bool {
        let mut add: HashSet<Point2> = HashSet::new();
        let mut remove: HashSet<Point2> = HashSet::new();
        if self.folds.is_empty() == false {
            let fold = self.folds.remove(0);
            for p in self.dots.iter() {
                match fold {
                    Fold::X(x) if p.x > x => {
                        add.insert(Point2 {
                            x: x - (p.x - x),
                            y: p.y,
                        });
                        remove.insert(*p);
                    }
                    Fold::Y(y) if p.y > y => {
                        add.insert(Point2 {
                            x: p.x,
                            y: y - (p.y - y),
                        });
                        remove.insert(*p);
                    }
                    _ => (),
                }
            }
            self.dots.extend(add.iter());
            self.dots.retain(|p| remove.contains(p) == false);
            true
        } else {
            false
        }
    }

    fn fold_all(&mut self) {
        while self.fold_next() == true {}
    }

    fn count_dots(&self) -> usize {
        self.dots.len()
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = Point2::get_range(&self.dots).unwrap();
        writeln!(f)?;
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                if self.dots.contains(&Point2 { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, "{}", self.blank_char)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Paper {
    Paper::parser(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn part1(input: &Paper) -> usize {
    let mut paper = input.clone();
    paper.fold_next();
    let count = paper.count_dots();
    assert_eq!(count, 755);
    count
}

#[aoc(day13, part2)]
pub fn part2(input: &Paper) -> String {
    let mut paper = input.clone();
    paper.fold_all();
    // input.blank_char = ' '; // Use this to read it more easily
    let code = paper.to_string();
    assert_eq!(
        code,
        "
###..#....#..#...##.###..###...##...##.
#..#.#....#.#.....#.#..#.#..#.#..#.#..#
###..#....##......#.#..#.###..#..#.#...
#..#.#....#.#.....#.###..#..#.####.#.##
#..#.#....#.#..#..#.#.#..#..#.#..#.#..#
###..####.#..#..##..#..#.###..#..#..###
"
    );
    code
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn test_fold() {
        let mut input = input_generator(EXAMPLE_INPUT);
        input.blank_char = '.';
        assert_eq!(
            input.to_string().trim(),
            "\
...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........"
        );

        input.fold_next();
        assert_eq!(
            input.to_string().trim(),
            "\
#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###"
        );
        assert_eq!(input.count_dots(), 17);

        input.fold_next();
        assert_eq!(
            input.to_string().trim(),
            "\
#####
#...#
#...#
#...#
#####"
        );

        let mut input = input_generator(EXAMPLE_INPUT);
        input.blank_char = '.';
        input.fold_all();
        assert_eq!(
            input.to_string().trim(),
            "\
#####
#...#
#...#
#...#
#####"
        );
    }
}
