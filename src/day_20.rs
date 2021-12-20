/*

*/

use crate::common::{Point2, Range2};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Image {
    algo: Vec<bool>,
    data: HashSet<Point2>,
    range: Range2,
    infinity_is_lit: bool,
    infinity_flips: bool,
}

impl Image {
    fn from_string(input: &str) -> Self {
        let mut lines = input.lines();
        let algo: Vec<bool> = lines.next().unwrap().chars().map(|c| c == '#').collect();

        let _ = lines.next();

        let mut data = HashSet::new();
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    data.insert(Point2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        let range = Point2::get_range(&data).unwrap();

        let infinity_flips = match (algo[0], algo.last().unwrap()) {
            (false, false) => false,
            (false, true) => false,
            (true, false) => true,
            (true, true) => false,
        };

        Self {
            algo,
            data,
            range,
            infinity_is_lit: false,
            infinity_flips,
        }
    }

    fn value(&self, p: &Point2) -> bool {
        if self.range.contains(*p) == true {
            self.data.get(p).is_some()
        } else if self.infinity_is_lit == true {
            true
        } else {
            false
        }
    }

    fn neighbors_as_int(&self, p: &Point2) -> usize {
        let mut n = 0;
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                n <<= 1;
                n |= if self.value(&(p.x + x, p.y + y).into()) == true {
                    1
                } else {
                    0
                };
            }
        }
        n
    }

    fn enhance(&mut self, n: u32) {
        let mut new_pixels: Vec<Point2> = Vec::new();
        for _ in 0..n {
            new_pixels.clear();

            for y in self.range.y.0 - 1..=self.range.y.1 + 1 {
                for x in self.range.x.0 - 1..=self.range.x.1 + 1 {
                    let pixel = Point2 { x, y };
                    let lookup = self.neighbors_as_int(&pixel);
                    assert!(lookup < self.algo.len());
                    if self.algo[lookup] == true {
                        new_pixels.push(pixel);
                    }
                }
            }

            self.data.clear();
            self.data.extend(new_pixels.drain(..));
            self.range = Point2::get_range(&self.data).unwrap();

            if self.infinity_flips == true {
                self.infinity_is_lit = !self.infinity_is_lit;
            }
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.data.len()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.range.y.0..=self.range.y.1 {
            for x in self.range.x.0..=self.range.x.1 {
                if self.value(&(x, y).into()) == true {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Image {
    Image::from_string(input)
}

#[aoc(day20, part1)]
pub fn part1(input: &Image) -> usize {
    let mut image = input.clone();
    image.enhance(2);
    let lit = image.count_lit_pixels();
    assert_eq!(lit, 5563);
    lit
}

#[aoc(day20, part2)]
pub fn part2(input: &Image) -> usize {
    let mut image = input.clone();
    image.enhance(50);
    let lit = image.count_lit_pixels();
    assert_eq!(lit, 19743);
    lit
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn test_neighbors_as_int() {
        let image = input_generator(EXAMPLE_INPUT);
        let lookup = image.neighbors_as_int(&(2, 2).into());
        assert_eq!(lookup, 34);
        assert_eq!(image.algo[lookup], true);
    }

    #[test]
    fn test_enhance() {
        let mut image = input_generator(EXAMPLE_INPUT);

        image.enhance(1);
        assert_eq!(
            image.to_string().trim(),
            "\
.##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#."
        );

        image.enhance(1);
        assert_eq!(
            image.to_string().trim(),
            "\
.......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###.."
        );
    }

    #[test]
    fn test_count_lit_pixels() {
        let mut image = input_generator(EXAMPLE_INPUT);
        image.enhance(2);
        assert_eq!(image.count_lit_pixels(), 35);

        let mut image = input_generator(EXAMPLE_INPUT);
        image.enhance(50);
        assert_eq!(image.count_lit_pixels(), 3351);
    }
}
