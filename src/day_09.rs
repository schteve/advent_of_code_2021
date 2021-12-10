/*
    --- Day 9: Smoke Basin ---
    These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

    If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

    Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

    Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

    In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

    The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

    Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?

    --- Part Two ---
    Next, you need to find the largest basins so you know what areas are most important to avoid.

    A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

    The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

    The top-left basin, size 3:

    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    The top-right basin, size 9:

    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    The middle basin, size 14:

    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    The bottom-right basin, size 9:

    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

    What do you get if you multiply together the sizes of the three largest basins?
*/

use crate::common::Point2;
use nom::IResult;
use std::collections::HashMap;

pub struct Cave {
    height_map: HashMap<Point2, u32>,
}

impl Cave {
    fn parser(input: &str) -> IResult<&str, Self> {
        let mut height_map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap();
                let p = (x as i32, y as i32).into();
                height_map.insert(p, height);
            }
        }
        Ok((input, Self { height_map }))
    }

    fn find_low_points(&self) -> Vec<Point2> {
        let mut lows = Vec::new();
        for (p, height) in self.height_map.iter() {
            let is_low = p.orthogonals().all(|x| {
                if let Some(adj_height) = self.height_map.get(&x) {
                    adj_height > height
                } else {
                    true
                }
            });
            if is_low == true {
                lows.push(*p);
            }
        }
        lows
    }

    fn calc_risk_level(&self) -> u32 {
        let lows = self.find_low_points();
        lows.iter()
            .map(|low| self.height_map.get(low).unwrap() + 1)
            .sum()
    }

    fn find_basin_sizes(&self) -> Vec<u32> {
        let mut basins = Vec::new();

        // For performance reasons, re-use the same collections for each basin
        let mut frontier = Vec::new();
        let mut visited = Vec::new();
        for low in self.find_low_points() {
            frontier.clear();
            frontier.push(low);
            visited.clear();
            visited.push(low);
            while let Some(next) = frontier.pop() {
                for adj in next.orthogonals() {
                    if let Some(height) = self.height_map.get(&adj) {
                        if *height < 9 && visited.contains(&adj) == false {
                            visited.push(adj);
                            frontier.push(adj);
                        }
                    }
                }
            }
            basins.push(visited.len() as u32);
        }

        basins.sort_unstable();
        basins
    }

    fn calc_3_largest_basins(&self) -> u32 {
        let mut basins = self.find_basin_sizes();
        basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Cave {
    Cave::parser(input).unwrap().1
}

#[aoc(day9, part1)]
pub fn part1(input: &Cave) -> u32 {
    let risk_level = input.calc_risk_level();
    assert_eq!(risk_level, 468);
    risk_level
}

#[aoc(day9, part2)]
pub fn part2(input: &Cave) -> u32 {
    let largest = input.calc_3_largest_basins();
    assert_eq!(largest, 1280496);
    largest
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_find_low_points() {
        let input = input_generator(EXAMPLE_INPUT);
        let mut lows = input.find_low_points();
        lows.sort_unstable();
        assert_eq!(
            lows,
            vec![(1, 0).into(), (9, 0).into(), (2, 2).into(), (6, 4).into()]
        );
    }

    #[test]
    fn test_risk_level() {
        let input = input_generator(EXAMPLE_INPUT);
        let risk_level = input.calc_risk_level();
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn test_find_basin_sizes() {
        let input = input_generator(EXAMPLE_INPUT);
        let basins = input.find_basin_sizes();
        assert_eq!(basins, vec![3, 9, 9, 14]);
    }

    #[test]
    fn test_calc_3_largest_basins() {
        let input = input_generator(EXAMPLE_INPUT);
        let largest = input.calc_3_largest_basins();
        assert_eq!(largest, 1134);
    }
}
