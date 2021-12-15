/*
    --- Day 14: Extended Polymerization ---
    The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.

    The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.

    For example:

    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
    The first line is the polymer template - this is the starting point of the process.

    The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.

    So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:

    The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
    The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
    The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.
    Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.

    After the first step of this process, the polymer becomes NCNBCHB.

    Here are the results of a few steps using the above rules:

    Template:     NNCB
    After step 1: NCNBCHB
    After step 2: NBCCNBBBCBHCB
    After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
    After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
    This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.

    Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?

    --- Part Two ---
    The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.

    In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.

    Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
*/

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, multispace0},
    combinator::map,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Polymerization {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
    start: (char, char),
    polymers: HashMap<(char, char), u64>,
}

impl Polymerization {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, template): (_, Vec<char>) = map(alpha1, |x: &str| x.chars().collect())(input)?;
        let (input, rules_list) = many1(preceded(
            multispace0,
            map(
                tuple((anychar, anychar, tag(" -> "), anychar)),
                |(a, b, _, d)| ((a, b), d),
            ),
        ))(input)?;

        let rules = rules_list.into_iter().collect();
        let start = (template[0], template[1]);

        let mut polymers: HashMap<(char, char), u64> = HashMap::new();
        for pair in template.windows(2) {
            let entry = polymers.entry((pair[0], pair[1])).or_insert(0);
            *entry += 1;
        }

        Ok((
            input,
            Self {
                template,
                rules,
                start,
                polymers,
            },
        ))
    }

    fn expand(&mut self, steps: u64) {
        for _ in 0..steps {
            let mut next_polymers = HashMap::new();
            for (pair, count) in &self.polymers {
                let new = self
                    .rules
                    .get(pair)
                    .unwrap_or_else(|| panic!("Pair not known in rules: {:?}", pair));
                let entry = next_polymers.entry((pair.0, *new)).or_insert(0);
                *entry += count;
                let entry = next_polymers.entry((*new, pair.1)).or_insert(0);
                *entry += count;

                // Keep track of which pair is first
                self.start = (self.start.0, *self.rules.get(&self.start).unwrap());
            }
            self.polymers = next_polymers;
        }
    }

    fn score(&self) -> u64 {
        // Since we only maintain counts for each pair, most elements are duplicated in the counts.
        // Therefore, we only count the second element in each pair and keep track of what the starting
        // pair is so we can count the very first element.
        let mut counts: HashMap<char, u64> = HashMap::new();
        counts.insert(self.start.0, *self.polymers.get(&(self.start)).unwrap());

        for (pair, count) in self.polymers.iter() {
            let entry = counts.entry(pair.1).or_insert(0);
            *entry += count;
        }

        let most = counts.iter().max_by_key(|x| x.1).unwrap().1;
        let least = counts.iter().min_by_key(|x| x.1).unwrap().1;
        most - least
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Polymerization {
    Polymerization::parser(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn part1(input: &Polymerization) -> u64 {
    let mut poly = input.clone();
    poly.expand(10);
    let score = poly.score();
    assert_eq!(score, 2223);
    score
}

#[aoc(day14, part2)]
pub fn part2(input: &Polymerization) -> u64 {
    let mut poly = input.clone();
    poly.expand(40);
    let score = poly.score();
    assert_eq!(score, 2566282754493);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_poly() {
        let mut poly = input_generator(EXAMPLE_INPUT);
        assert_eq!(poly.template, "NNCB".chars().collect::<Vec<_>>());
        assert_eq!(poly.score(), 1);

        poly.expand(1);
        assert_eq!(poly.score(), 1);

        poly.expand(1);
        assert_eq!(poly.score(), 5);

        poly.expand(1);
        assert_eq!(poly.score(), 7);

        poly.expand(1);
        assert_eq!(poly.score(), 18);

        let mut poly = input_generator(EXAMPLE_INPUT);
        poly.expand(10);
        assert_eq!(poly.score(), 1588);
    }
}
