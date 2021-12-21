/*
    --- Day 20: Trench Map ---
    With the scanners fully deployed, you turn their attention to mapping the floor of the ocean trench.

    When you get back the image from the scanners, it seems to just be random noise. Perhaps you can combine an image enhancement algorithm and the input image (your puzzle input) to clean it up a little.

    For example:

    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
    #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
    .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
    .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
    .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
    ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
    ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
    The first section is the image enhancement algorithm. It is normally given on a single line, but it has been wrapped to multiple lines in this example for legibility. The second section is the input image, a two-dimensional grid of light pixels (#) and dark pixels (.).

    The image enhancement algorithm describes how to enhance an image by simultaneously converting all pixels in the input image into an output image. Each pixel of the output image is determined by looking at a 3x3 square of pixels centered on the corresponding input image pixel. So, to determine the value of the pixel at (5,10) in the output image, nine pixels from the input image need to be considered: (4,9), (4,10), (4,11), (5,9), (5,10), (5,11), (6,9), (6,10), and (6,11). These nine input pixels are combined into a single binary number that is used as an index in the image enhancement algorithm string.

    For example, to determine the output pixel that corresponds to the very middle pixel of the input image, the nine pixels marked by [...] would need to be considered:

    # . . # .
    #[. . .].
    #[# . .]#
    .[. # .].
    . . # # #
    Starting from the top-left and reading across each row, these pixels are ..., then #.., then .#.; combining these forms ...#...#.. By turning dark pixels (.) into 0 and light pixels (#) into 1, the binary number 000100010 can be formed, which is 34 in decimal.

    The image enhancement algorithm string is exactly 512 characters long, enough to match every possible 9-bit binary number. The first few characters of the string (numbered starting from zero) are as follows:

    0         10        20        30  34    40        50        60        70
    |         |         |         |   |     |         |         |         |
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
    In the middle of this first group of characters, the character at index 34 can be found: #. So, the output pixel in the center of the output image should be #, a light pixel.

    This process can then be repeated to calculate every pixel of the output image.

    Through advances in imaging technology, the images being operated on here are infinite in size. Every pixel of the infinite output image needs to be calculated exactly based on the relevant pixels of the input image. The small input image you have is only a small region of the actual infinite input image; the rest of the input image consists of dark pixels (.). For the purposes of the example, to save on space, only a portion of the infinite-sized input and output images will be shown.

    The starting input image, therefore, looks something like this, with more dark pixels (.) extending forever in every direction not shown here:

    ...............
    ...............
    ...............
    ...............
    ...............
    .....#..#......
    .....#.........
    .....##..#.....
    .......#.......
    .......###.....
    ...............
    ...............
    ...............
    ...............
    ...............
    By applying the image enhancement algorithm to every pixel simultaneously, the following output image can be obtained:

    ...............
    ...............
    ...............
    ...............
    .....##.##.....
    ....#..#.#.....
    ....##.#..#....
    ....####..#....
    .....#..##.....
    ......##..#....
    .......#.#.....
    ...............
    ...............
    ...............
    ...............
    Through further advances in imaging technology, the above output image can also be used as an input image! This allows it to be enhanced a second time:

    ...............
    ...............
    ...............
    ..........#....
    ....#..#.#.....
    ...#.#...###...
    ...#...##.#....
    ...#.....#.#...
    ....#.#####....
    .....#.#####...
    ......##.##....
    .......###.....
    ...............
    ...............
    ...............
    Truly incredible - now the small details are really starting to come through. After enhancing the original input image twice, 35 pixels are lit.

    Start with the original input image and apply the image enhancement algorithm twice, being careful to account for the infinite size of the images. How many pixels are lit in the resulting image?

    --- Part Two ---
    You still can't quite make out the details in the image. Maybe you just didn't enhance it enough.

    If you enhance the starting input image in the above example a total of 50 times, 3351 pixels are lit in the final output image.

    Start again with the original input image and apply the image enhancement algorithm 50 times. How many pixels are lit in the resulting image?--- Day 20: Trench Map ---
    With the scanners fully deployed, you turn their attention to mapping the floor of the ocean trench.

    When you get back the image from the scanners, it seems to just be random noise. Perhaps you can combine an image enhancement algorithm and the input image (your puzzle input) to clean it up a little.

    For example:

    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
    #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
    .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
    .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
    .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
    ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
    ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
    The first section is the image enhancement algorithm. It is normally given on a single line, but it has been wrapped to multiple lines in this example for legibility. The second section is the input image, a two-dimensional grid of light pixels (#) and dark pixels (.).

    The image enhancement algorithm describes how to enhance an image by simultaneously converting all pixels in the input image into an output image. Each pixel of the output image is determined by looking at a 3x3 square of pixels centered on the corresponding input image pixel. So, to determine the value of the pixel at (5,10) in the output image, nine pixels from the input image need to be considered: (4,9), (4,10), (4,11), (5,9), (5,10), (5,11), (6,9), (6,10), and (6,11). These nine input pixels are combined into a single binary number that is used as an index in the image enhancement algorithm string.

    For example, to determine the output pixel that corresponds to the very middle pixel of the input image, the nine pixels marked by [...] would need to be considered:

    # . . # .
    #[. . .].
    #[# . .]#
    .[. # .].
    . . # # #
    Starting from the top-left and reading across each row, these pixels are ..., then #.., then .#.; combining these forms ...#...#.. By turning dark pixels (.) into 0 and light pixels (#) into 1, the binary number 000100010 can be formed, which is 34 in decimal.

    The image enhancement algorithm string is exactly 512 characters long, enough to match every possible 9-bit binary number. The first few characters of the string (numbered starting from zero) are as follows:

    0         10        20        30  34    40        50        60        70
    |         |         |         |   |     |         |         |         |
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
    In the middle of this first group of characters, the character at index 34 can be found: #. So, the output pixel in the center of the output image should be #, a light pixel.

    This process can then be repeated to calculate every pixel of the output image.

    Through advances in imaging technology, the images being operated on here are infinite in size. Every pixel of the infinite output image needs to be calculated exactly based on the relevant pixels of the input image. The small input image you have is only a small region of the actual infinite input image; the rest of the input image consists of dark pixels (.). For the purposes of the example, to save on space, only a portion of the infinite-sized input and output images will be shown.

    The starting input image, therefore, looks something like this, with more dark pixels (.) extending forever in every direction not shown here:

    ...............
    ...............
    ...............
    ...............
    ...............
    .....#..#......
    .....#.........
    .....##..#.....
    .......#.......
    .......###.....
    ...............
    ...............
    ...............
    ...............
    ...............
    By applying the image enhancement algorithm to every pixel simultaneously, the following output image can be obtained:

    ...............
    ...............
    ...............
    ...............
    .....##.##.....
    ....#..#.#.....
    ....##.#..#....
    ....####..#....
    .....#..##.....
    ......##..#....
    .......#.#.....
    ...............
    ...............
    ...............
    ...............
    Through further advances in imaging technology, the above output image can also be used as an input image! This allows it to be enhanced a second time:

    ...............
    ...............
    ...............
    ..........#....
    ....#..#.#.....
    ...#.#...###...
    ...#...##.#....
    ...#.....#.#...
    ....#.#####....
    .....#.#####...
    ......##.##....
    .......###.....
    ...............
    ...............
    ...............
    Truly incredible - now the small details are really starting to come through. After enhancing the original input image twice, 35 pixels are lit.

    Start with the original input image and apply the image enhancement algorithm twice, being careful to account for the infinite size of the images. How many pixels are lit in the resulting image?

    --- Part Two ---
    You still can't quite make out the details in the image. Maybe you just didn't enhance it enough.

    If you enhance the starting input image in the above example a total of 50 times, 3351 pixels are lit in the final output image.

    Start again with the original input image and apply the image enhancement algorithm 50 times. How many pixels are lit in the resulting image?
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
