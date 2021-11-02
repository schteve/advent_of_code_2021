/*

*/

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    todo!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
";

    #[test]
    fn test_() {

    }
}
