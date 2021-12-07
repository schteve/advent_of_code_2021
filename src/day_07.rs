/*
    --- Day 7: The Treachery of Whales ---
    A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

    Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

    The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

    There's one major catch - crab submarines can only move horizontally.

    You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

    For example, consider the following horizontal positions:

    16,1,2,0,4,2,7,1,2,14
    This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

    Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

    Move from 16 to 2: 14 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 0 to 2: 2 fuel
    Move from 4 to 2: 2 fuel
    Move from 2 to 2: 0 fuel
    Move from 7 to 2: 5 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 14 to 2: 12 fuel
    This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

    Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?

    --- Part Two ---
    The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?

    As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.

    As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:

    Move from 16 to 5: 66 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 0 to 5: 15 fuel
    Move from 4 to 5: 1 fuel
    Move from 2 to 5: 6 fuel
    Move from 7 to 5: 3 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 14 to 5: 45 fuel
    This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.

    Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
*/

use crate::common::{unsigned, Mode};
use nom::{character::complete::char, multi::separated_list1};
use std::cmp::{max, min};

fn crab_fuel(crab: u32, position: u32, mode: Mode) -> u32 {
    let abs_diff = max(crab, position) - min(crab, position);
    match mode {
        Mode::M1 => abs_diff,
        Mode::M2 => (abs_diff * (abs_diff + 1)) / 2,
    }
}

fn total_fuel(crabs: &[u32], position: u32, mode: Mode) -> u32 {
    crabs
        .iter()
        .map(|crab| crab_fuel(*crab, position, mode))
        .sum()
}

fn find_best_position(crabs: &[u32], mode: Mode) -> u32 {
    // There's probably a smarter way to find this. Brute force is cheap though.
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut lowest_fuel = None;
    let mut lowest_idx = None;
    for i in min..max {
        let fuel = total_fuel(crabs, i, mode);
        if let Some(low) = lowest_fuel {
            if fuel < low {
                lowest_fuel = Some(fuel);
                lowest_idx = Some(i);
            }
        } else {
            lowest_fuel = Some(fuel);
            lowest_idx = Some(i);
        }
    }
    lowest_idx.unwrap()
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<u32> {
    separated_list1(char(','), unsigned)(input).unwrap().1
}

#[aoc(day7, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let best = find_best_position(input, Mode::M1);
    let fuel = total_fuel(input, best, Mode::M1);
    assert_eq!(fuel, 355592);
    fuel
}

#[aoc(day7, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let best = find_best_position(input, Mode::M2);
    let fuel = total_fuel(input, best, Mode::M2);
    assert_eq!(fuel, 101618069);
    fuel
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_crab_fuel1() {
        assert_eq!(crab_fuel(16, 2, Mode::M1), 14);
        assert_eq!(crab_fuel(1, 2, Mode::M1), 1);
        assert_eq!(crab_fuel(2, 2, Mode::M1), 0);
        assert_eq!(crab_fuel(0, 2, Mode::M1), 2);
        assert_eq!(crab_fuel(4, 2, Mode::M1), 2);
        assert_eq!(crab_fuel(2, 2, Mode::M1), 0);
        assert_eq!(crab_fuel(7, 2, Mode::M1), 5);
        assert_eq!(crab_fuel(1, 2, Mode::M1), 1);
        assert_eq!(crab_fuel(2, 2, Mode::M1), 0);
        assert_eq!(crab_fuel(14, 2, Mode::M1), 12);
    }

    #[test]
    fn test_calc_fuel1() {
        let input = input_generator(EXAMPLE_INPUT);

        let fuel = total_fuel(&input, 1, Mode::M1);
        assert_eq!(fuel, 41);

        let fuel = total_fuel(&input, 2, Mode::M1);
        assert_eq!(fuel, 37);

        let fuel = total_fuel(&input, 3, Mode::M1);
        assert_eq!(fuel, 39);

        let fuel = total_fuel(&input, 10, Mode::M1);
        assert_eq!(fuel, 71);
    }

    #[test]
    fn test_find_best_position1() {
        let input = input_generator(EXAMPLE_INPUT);
        let best = find_best_position(&input, Mode::M1);
        assert_eq!(best, 2);
    }

    #[test]
    fn test_crab_fuel2() {
        assert_eq!(crab_fuel(16, 5, Mode::M2), 66);
        assert_eq!(crab_fuel(1, 5, Mode::M2), 10);
        assert_eq!(crab_fuel(2, 5, Mode::M2), 6);
        assert_eq!(crab_fuel(0, 5, Mode::M2), 15);
        assert_eq!(crab_fuel(4, 5, Mode::M2), 1);
        assert_eq!(crab_fuel(2, 5, Mode::M2), 6);
        assert_eq!(crab_fuel(7, 5, Mode::M2), 3);
        assert_eq!(crab_fuel(1, 5, Mode::M2), 10);
        assert_eq!(crab_fuel(2, 5, Mode::M2), 6);
        assert_eq!(crab_fuel(14, 5, Mode::M2), 45);
    }

    #[test]
    fn test_calc_fuel2() {
        let input = input_generator(EXAMPLE_INPUT);

        let fuel = total_fuel(&input, 2, Mode::M2);
        assert_eq!(fuel, 206);

        let fuel = total_fuel(&input, 5, Mode::M2);
        assert_eq!(fuel, 168);
    }

    #[test]
    fn test_find_best_position2() {
        let input = input_generator(EXAMPLE_INPUT);
        let best = find_best_position(&input, Mode::M2);
        assert_eq!(best, 5);
    }
}
