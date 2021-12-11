/*

*/

use crate::common::Point2;
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct OctoGrid {
    grid: HashMap<Point2, u32>,
}

impl OctoGrid {
    fn parser(input: &str) -> IResult<&str, Self> {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let energy = c.to_digit(10).unwrap();
                let p = (x as i32, y as i32).into();
                grid.insert(p, energy);
            }
        }
        Ok((input, Self { grid }))
    }

    fn step(&mut self, steps: u32) -> (usize, Option<u32>) {
        let mut count_flashes = 0;

        let mut to_flash: Vec<Point2> = Vec::new();
        let mut flashed: HashSet<Point2> = HashSet::new();

        for i in 0..steps {
            to_flash.clear();
            flashed.clear();

            // First, increase energy level
            let range = Point2::get_range(self.grid.keys()).unwrap();
            for x in range.x.0..=range.x.1 {
                for y in range.y.0..=range.y.1 {
                    let p = Point2 { x, y };
                    if let Some(v) = self.grid.get_mut(&p) {
                        *v += 1;
                        if *v > 9 {
                            to_flash.push(p);
                            flashed.insert(p);
                        }
                    } else {
                        panic!("Assume the grid is full");
                    }
                }
            }

            // Flash
            while let Some(p) = to_flash.pop() {
                for adj in p.adjacents() {
                    if let Some(v) = self.grid.get_mut(&adj) {
                        *v += 1;
                        if *v > 9 && flashed.contains(&adj) == false {
                            to_flash.push(adj);
                            flashed.insert(adj);
                        }
                    }
                }
            }

            // Count flashes, and a quick check to see if everything flashed
            count_flashes += flashed.len();
            if flashed.len() == self.grid.len() {
                return (count_flashes, Some(i + 1)); // In the puzzle, steps are 1-based
            }

            // Last, reset energy to zero if they flashed
            for oct in flashed.drain() {
                self.grid.insert(oct, 0);
            }
        }

        (count_flashes, None)
    }
}

impl std::fmt::Display for OctoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = Point2::get_range(self.grid.keys()).unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                if let Some(t) = self.grid.get(&Point2 { x, y }) {
                    write!(f, "{}", t)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> OctoGrid {
    OctoGrid::parser(input).unwrap().1
}

#[aoc(day11, part1)]
pub fn part1(input: &OctoGrid) -> usize {
    let mut grid = input.clone();
    let (flashes, _) = grid.step(100);
    assert_eq!(flashes, 1601);
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &OctoGrid) -> u32 {
    let mut grid = input.clone();
    let (_, sync_step) = grid.step(1000);
    assert_eq!(sync_step, Some(368));
    sync_step.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
11111
19991
19191
19991
11111
";

    static EXAMPLE_INPUT2: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_step_flashes() {
        let mut grid = input_generator(EXAMPLE_INPUT1);
        assert_eq!(
            grid.to_string(),
            "\
11111
19991
19191
19991
11111
"
        );

        let (flashes, _) = grid.step(1);
        assert_eq!(flashes, 9);
        assert_eq!(
            grid.to_string(),
            "\
34543
40004
50005
40004
34543
"
        );

        let (flashes, _) = grid.step(1);
        assert_eq!(flashes, 0);
        assert_eq!(
            grid.to_string(),
            "\
45654
51115
61116
51115
45654
"
        );

        let mut grid = input_generator(EXAMPLE_INPUT2);
        assert_eq!(
            grid.to_string(),
            "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"
        );

        let (flashes, _) = grid.step(1);
        assert_eq!(flashes, 0);
        assert_eq!(
            grid.to_string(),
            "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976
"
        );

        grid.step(1);
        assert_eq!(
            grid.to_string(),
            "\
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
"
        );

        let mut grid = input_generator(EXAMPLE_INPUT2);
        let (flashes, _) = grid.step(10);
        assert_eq!(flashes, 204);
        assert_eq!(
            grid.to_string(),
            "\
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
"
        );

        let mut grid = input_generator(EXAMPLE_INPUT2);
        let (flashes, _) = grid.step(100);
        assert_eq!(flashes, 1656);
        assert_eq!(
            grid.to_string(),
            "\
0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766
"
        );
    }

    #[test]
    fn test_step_sync() {
        let mut grid = input_generator(EXAMPLE_INPUT2);

        let (_, sync) = grid.step(1000);
        assert_eq!(sync, Some(195));
    }
}
