/*
    --- Day 8: Seven Segment Search ---
    You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

    As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

    Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

      0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....

      5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg
    So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

    The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

    So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

    For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

    For example, here is what you might see in a single entry in your notes:

    acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
    cdfeb fcadb cdfeb cdbaf
    (The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

    Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

    Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

    For now, focus on the easy digits. Consider this larger example:

    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
    fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
    cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
    efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
    gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
    gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
    cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
    ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
    gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
    fgae cfgab fg bagce
    Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

    In the output values, how many times do digits 1, 4, 7, or 8 appear?

    --- Part Two ---
    Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

    acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
    cdfeb fcadb cdfeb cdbaf
    After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

     dddd
    e    a
    e    a
     ffff
    g    b
    g    b
     cccc
    So, the unique signal patterns would correspond to the following digits:

    acedgfb: 8
    cdfbe: 5
    gcdfa: 2
    fbcad: 3
    dab: 7
    cefabd: 9
    cdfgeb: 6
    eafb: 4
    cagedb: 0
    ab: 1
    Then, the four digits of the output value can be decoded:

    cdfeb: 5
    fcadb: 3
    cdfeb: 5
    cdbaf: 3
    Therefore, the output value for this entry is 5353.

    Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

    fdgacbe cefdb cefbgd gcbe: 8394
    fcgedb cgb dgebacf gc: 9781
    cg cg fdcagb cbg: 1197
    efabcd cedba gadfec cb: 9361
    gecf egdcabf bgf bfgea: 4873
    gebdcfa ecba ca fadegcb: 8418
    cefg dcbef fcge gbcadfe: 4548
    ed bcgafe cdgba cbgef: 1625
    gbdfcae bgc cg cgb: 8717
    fgae cfgab fg bagce: 4315
    Adding all of the output values in this larger example produces 61229.

    For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
*/

use crate::common::trim_start;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;

type Signal = HashSet<char>;

pub struct Entry {
    patterns: Vec<Signal>,
    outputs: Vec<Signal>,
}

impl Entry {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (patterns_str, outputs_str)) = trim_start(separated_pair(
            separated_list1(char(' '), alpha1),
            tag(" | "),
            separated_list1(char(' '), alpha1),
        ))(input)?;

        let patterns = patterns_str
            .into_iter()
            .map(|x| x.chars().collect())
            .collect();
        let outputs = outputs_str
            .into_iter()
            .map(|x| x.chars().collect())
            .collect();

        Ok((input, Self { patterns, outputs }))
    }

    fn value(&self, digits: &Digits) -> usize {
        let values: Vec<usize> = self.outputs.iter().map(|o| digits.value(o)).collect();
        values[0] * 1000 + values[1] * 100 + values[2] * 10 + values[3]
    }
}

fn count_1478(entries: &[Entry]) -> usize {
    let mut count = 0;
    for entry in entries {
        for o in &entry.outputs {
            if o.len() != 5 && o.len() != 6 {
                count += 1;
            }
        }
    }
    count
}

struct Lens {
    len2: Signal,
    len3: Signal,
    len4: Signal,
    len5: Vec<Signal>,
    len6: Vec<Signal>,
    len7: Signal,
}

impl Lens {
    fn from_entry(entry: &Entry) -> Self {
        let mut len2 = Vec::new();
        let mut len3 = Vec::new();
        let mut len4 = Vec::new();
        let mut len5 = Vec::new();
        let mut len6 = Vec::new();
        let mut len7 = Vec::new();

        for pattern in entry.patterns.iter().cloned() {
            match pattern.len() {
                2 => len2.push(pattern),
                3 => len3.push(pattern),
                4 => len4.push(pattern),
                5 => len5.push(pattern),
                6 => len6.push(pattern),
                7 => len7.push(pattern),
                x => panic!("Invalid pattern length of {} for {:?}", x, pattern),
            }
        }

        assert_eq!(len2.len(), 1);
        assert_eq!(len3.len(), 1);
        assert_eq!(len4.len(), 1);
        assert_eq!(len5.len(), 3);
        assert_eq!(len6.len(), 3);
        assert_eq!(len7.len(), 1);

        Self {
            len2: len2.remove(0),
            len3: len3.remove(0),
            len4: len4.remove(0),
            len5,
            len6,
            len7: len7.remove(0),
        }
    }
}

struct Digits {
    d0: Signal,
    d1: Signal,
    d2: Signal,
    d3: Signal,
    d4: Signal,
    d5: Signal,
    d6: Signal,
    d7: Signal,
    d8: Signal,
    d9: Signal,
}

impl Digits {
    fn from_lens(lens: &Lens) -> Self {
        // Digits 1, 4, 7, 8 are freebies
        let d1 = lens.len2.clone();
        let d4 = lens.len4.clone();
        let d7 = lens.len3.clone();
        let d8 = lens.len7.clone();

        // To tell which length 5 signal is a 2, add the segments from a 4 and it should make an 8
        let d2 = lens
            .len5
            .iter()
            .find(|x| x.union(&d4).count() == 7)
            .unwrap()
            .clone();

        // To tell which length 5 signal is a 3, find the one which is a superset of 7
        let d3 = lens
            .len5
            .iter()
            .find(|x| x.is_superset(&d7))
            .unwrap()
            .clone();

        // 5 is the remaining length 5 signal
        let d5 = lens
            .len5
            .iter()
            .find(|&x| x != &d2 && x != &d3)
            .unwrap()
            .clone();

        // To tell which length 6 signal is a 6, add the segments from a 1 and it should make an 8
        let d6 = lens
            .len6
            .iter()
            .find(|x| x.union(&d1).count() == 7)
            .unwrap()
            .clone();

        // To tell which length 6 signal is a 9, find the one which is a superset of 4
        let d9 = lens
            .len6
            .iter()
            .find(|x| x.is_superset(&d4))
            .unwrap()
            .clone();

        // 0 is the remaining length 6 signal
        let d0 = lens
            .len6
            .iter()
            .find(|&x| x != &d6 && x != &d9)
            .unwrap()
            .clone();

        Self {
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
            d8,
            d9,
        }
    }

    fn value(&self, signal: &Signal) -> usize {
        if signal == &self.d0 {
            0
        } else if signal == &self.d1 {
            1
        } else if signal == &self.d2 {
            2
        } else if signal == &self.d3 {
            3
        } else if signal == &self.d4 {
            4
        } else if signal == &self.d5 {
            5
        } else if signal == &self.d6 {
            6
        } else if signal == &self.d7 {
            7
        } else if signal == &self.d8 {
            8
        } else if signal == &self.d9 {
            9
        } else {
            panic!("Invalid signal: {:?}", signal)
        }
    }
}

fn solve_entry(entry: &Entry) -> usize {
    let lens = Lens::from_entry(entry);
    let digits = Digits::from_lens(&lens);
    entry.value(&digits)
}

fn solve_entries(entries: &[Entry]) -> usize {
    entries.iter().map(solve_entry).sum()
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    many1(Entry::parser)(input).unwrap().1
}

#[aoc(day8, part1)]
pub fn part1(input: &[Entry]) -> usize {
    let count = count_1478(input);
    assert_eq!(count, 261);
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &[Entry]) -> usize {
    let value_sum = solve_entries(input);
    assert_eq!(value_sum, 987553);
    value_sum
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    static EXAMPLE_INPUT2: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_count_1478() {
        let input = input_generator(EXAMPLE_INPUT1);
        let count = count_1478(&input);
        assert_eq!(count, 0);

        let input = input_generator(EXAMPLE_INPUT2);
        let count = count_1478(&input);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_solve_entry() {
        let input = input_generator(EXAMPLE_INPUT1);
        let value = solve_entry(&input[0]);
        assert_eq!(value, 5353);

        let input = input_generator(EXAMPLE_INPUT2);
        assert_eq!(solve_entry(&input[0]), 8394);
        assert_eq!(solve_entry(&input[1]), 9781);
        assert_eq!(solve_entry(&input[2]), 1197);
        assert_eq!(solve_entry(&input[3]), 9361);
        assert_eq!(solve_entry(&input[4]), 4873);
        assert_eq!(solve_entry(&input[5]), 8418);
        assert_eq!(solve_entry(&input[6]), 4548);
        assert_eq!(solve_entry(&input[7]), 1625);
        assert_eq!(solve_entry(&input[8]), 8717);
        assert_eq!(solve_entry(&input[9]), 4315);
    }

    #[test]
    fn test_solve_entries() {
        let input = input_generator(EXAMPLE_INPUT1);
        let value_sum = solve_entries(&input);
        assert_eq!(value_sum, 5353);

        let input = input_generator(EXAMPLE_INPUT2);
        let value_sum = solve_entries(&input);
        assert_eq!(value_sum, 61229);
    }
}
