/*
    --- Day 18: Snailfish ---
    You descend into the ocean trench and encounter some snailfish. They say they saw the sleigh keys! They'll even tell you which direction the keys went if you help one of the smaller snailfish with his math homework.

    Snailfish numbers aren't like regular numbers. Instead, every snailfish number is a pair - an ordered list of two elements. Each element of the pair can be either a regular number or another pair.

    Pairs are written as [x,y], where x and y are the elements within the pair. Here are some example snailfish numbers, one snailfish number per line:

    [1,2]
    [[1,2],3]
    [9,[8,7]]
    [[1,9],[8,5]]
    [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
    [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
    [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
    This snailfish homework is about addition. To add two snailfish numbers, form a pair from the left and right parameters of the addition operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].

    There's only one problem: snailfish numbers must always be reduced, and the process of adding two snailfish numbers can result in snailfish numbers that need to be reduced.

    To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish number:

    If any pair is nested inside four pairs, the leftmost such pair explodes.
    If any regular number is 10 or greater, the leftmost such regular number splits.
    Once no action in the above list applies, the snailfish number is reduced.

    During reduction, at most one action applies, after which the process returns to the top of the list of actions. For example, if split produces a pair that meets the explode criteria, that pair explodes before other splits occur.

    To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair (if any), and the pair's right value is added to the first regular number to the right of the exploding pair (if any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair is replaced with the regular number 0.

    Here are some examples of a single explode action:

    [[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4] (the 9 has no regular number to its left, so it is not added to any regular number).
    [7,[6,[5,[4,[3,2]]]]] becomes [7,[6,[5,[7,0]]]] (the 2 has no regular number to its right, and so it is not added to any regular number).
    [[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].
    [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action).
    [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]].
    To split a regular number, replace it with a pair; the left element of the pair should be the regular number divided by two and rounded down, while the right element of the pair should be the regular number divided by two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.

    Here is the process of finding the reduced result of [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]:

    after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
    after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
    after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
    after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
    after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
    after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
    Once no reduce actions apply, the snailfish number that remains is the actual result of the addition operation: [[[[0,7],4],[[7,8],[6,0]]],[8,1]].

    The homework assignment involves adding up a list of snailfish numbers (your puzzle input). The snailfish numbers are each listed on a separate line. Add the first snailfish number and the second, then add that result and the third, then add that result and the fourth, and so on until all numbers in the list have been used once.

    For example, the final sum of this list is [[[[1,1],[2,2]],[3,3]],[4,4]]:

    [1,1]
    [2,2]
    [3,3]
    [4,4]
    The final sum of this list is [[[[3,0],[5,3]],[4,4]],[5,5]]:

    [1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    The final sum of this list is [[[[5,0],[7,4]],[5,5]],[6,6]]:

    [1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    [6,6]
    Here's a slightly larger example:

    [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]
    The final sum [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] is found after adding up the above snailfish numbers:

    [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    + [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    = [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]

    [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
    + [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    = [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]

    [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
    + [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    = [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]

    [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
    + [7,[5,[[3,8],[1,4]]]]
    = [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]

    [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
    + [[2,[2,2]],[8,[8,1]]]
    = [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]

    [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
    + [2,9]
    = [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]

    [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
    + [1,[[[9,3],9],[[9,0],[0,7]]]]
    = [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

    [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
    + [[[5,[7,4]],7],1]
    = [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]

    [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
    + [[[[4,2],2],6],[8,7]]
    = [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
    To check whether it's the right answer, the snailfish teacher only checks the magnitude of the final sum. The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element. The magnitude of a regular number is just that number.

    For example, the magnitude of [9,1] is 3*9 + 2*1 = 29; the magnitude of [1,9] is 3*1 + 2*9 = 21. Magnitude calculations are recursive: the magnitude of [[9,1],[1,9]] is 3*29 + 2*21 = 129.

    Here are a few more magnitude examples:

    [[1,2],[[3,4],5]] becomes 143.
    [[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384.
    [[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445.
    [[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791.
    [[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137.
    [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488.
    So, given this example homework assignment:

    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    The final sum is:

    [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
    The magnitude of this final sum is 4140.

    Add up all of the snailfish numbers from the homework assignment in the order they appear. What is the magnitude of the final sum?

    --- Part Two ---
    You notice a second question on the back of the homework assignment:

    What is the largest magnitude you can get from adding only two of the snailfish numbers?

    Note that snailfish addition is not commutative - that is, x + y and y + x can produce different results.

    Again considering the last example homework assignment above:

    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    The largest magnitude of the sum of any two snailfish numbers in this list is 3993. This is the magnitude of [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]] + [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]], which reduces to [[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]].

    What is the largest magnitude of any sum of two different snailfish numbers from the homework assignment?
*/

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Number(u32),
    PairOpen,
    PairSep,
    PairClose,
}

impl Element {
    fn from_char(c: char) -> Self {
        match c {
            '0'..='9' => Self::Number(c.to_digit(10).unwrap()),
            '[' => Self::PairOpen,
            ']' => Self::PairClose,
            ',' => Self::PairSep,
            _ => panic!("Unknown character: {}", c),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::PairOpen => write!(f, "["),
            Self::PairSep => write!(f, ","),
            Self::PairClose => write!(f, "]"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    data: Vec<Element>,
}

impl Line {
    fn parse(input: &str) -> Self {
        Self {
            data: input.chars().map(Element::from_char).collect(),
        }
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for idx in 0..self.data.len() {
            match self.data[idx] {
                Element::Number(_) => (),
                Element::PairOpen => depth += 1,
                Element::PairSep => (),
                Element::PairClose => depth -= 1,
            }

            if depth > 4 {
                // EXPLODE!

                // Get the numbers in the pair, by definition
                let left_pair_idx = idx + 1;
                let right_pair_idx = idx + 3;
                let left_pair_n = if let Element::Number(n) = self.data[left_pair_idx] {
                    n
                } else {
                    panic!("Pair left is not a number");
                };
                let right_pair_n = if let Element::Number(n) = self.data[right_pair_idx] {
                    n
                } else {
                    panic!("Pair right is not a number");
                };

                // Find the next number to the left, if any, and add it with the left pair value
                for left_idx in (0..left_pair_idx).rev() {
                    if let Element::Number(ref mut n) = self.data[left_idx] {
                        *n += left_pair_n;
                        break;
                    }
                }

                // Find the next number to the right, if any, and add it with the right pair value
                for right_idx in (right_pair_idx + 1)..self.data.len() {
                    if let Element::Number(ref mut n) = self.data[right_idx] {
                        *n += right_pair_n;
                        break;
                    }
                }

                // Replace pair with 0. Overwrite the open bracket, then drain the left, comma, right, and end bracket.
                self.data[left_pair_idx - 1] = Element::Number(0);
                self.data.drain(left_pair_idx..=(right_pair_idx + 1));

                return true;
            }
        }
        false // Nothing to explode
    }

    fn split(&mut self) -> bool {
        for idx in 0..self.data.len() {
            if let Element::Number(n) = self.data[idx] {
                if n >= 10 {
                    // SPLIT!

                    // Calculate new left and right numbers
                    let left_n = n / 2;
                    let right_n = n - left_n;

                    // Insert new pair
                    let new = [
                        Element::PairOpen,
                        Element::Number(left_n),
                        Element::PairSep,
                        Element::Number(right_n),
                        Element::PairClose,
                    ];
                    self.data.splice(idx..idx + 1, new.into_iter());
                    return true;
                }
            }
        }
        false // Nothing to split
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() == true {
                // Go back and check for more explodes
                continue;
            }

            if self.split() == true {
                // Go back and check for more explodes or splits
                continue;
            }

            // No more explodes or splits to do
            break;
        }
    }

    fn add(&mut self, mut other: Line) {
        if self.data.is_empty() == true {
            self.data.append(&mut other.data);
        } else {
            self.data.insert(0, Element::PairOpen);
            self.data.push(Element::PairSep);
            self.data.append(&mut other.data);
            self.data.push(Element::PairClose);
        }
    }

    fn magnitude(&mut self) -> u32 {
        // Replace [n,m] with Number(3*n + 2*m)
        'top: loop {
            let max_len = if self.data.len() > 4 {
                self.data.len() - 4
            } else {
                0
            };
            for idx in 0..max_len {
                if self.data[idx] == Element::PairOpen
                    && self.data[idx + 2] == Element::PairSep
                    && self.data[idx + 4] == Element::PairClose
                {
                    let n = if let Element::Number(n) = self.data[idx + 1] {
                        n
                    } else {
                        panic!("n is not a number");
                    };
                    let m = if let Element::Number(m) = self.data[idx + 3] {
                        m
                    } else {
                        panic!("m is not a number");
                    };
                    let mag = 3 * n + 2 * m;

                    // Replace first index in pair with magnitude and drain the rest
                    self.data[idx] = Element::Number(mag);
                    self.data.drain(idx + 1..=idx + 4);
                    continue 'top;
                }
            }
            break;
        }

        if let Element::Number(n) = self.data[0] {
            n
        } else {
            panic!("Magnitude calculation failed");
        }
    }
}

fn sum_list(lines: Vec<Line>) -> Line {
    let mut combined = Line::parse("");
    for line in lines.into_iter() {
        combined.add(line);
        combined.reduce();
    }
    combined
}

fn do_homework(lines: Vec<Line>) -> u32 {
    let mut combined = sum_list(lines);
    combined.magnitude()
}

fn sum_of_two(lines: Vec<Line>) -> u32 {
    let mut max = 0;

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let mut a = lines[i].clone();
            let b = lines[j].clone();
            a.add(b);
            a.reduce();
            let mag = a.magnitude();

            if mag > max {
                max = mag;
            }
        }
    }

    max
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.data {
            write!(f, "{}", e)?;
        }
        Ok(())
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input.lines().map(Line::parse).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Line]) -> u32 {
    let lines = input.to_vec();
    let ans = do_homework(lines);
    assert_eq!(ans, 4347);
    ans
}

#[aoc(day18, part2)]
pub fn part2(input: &[Line]) -> u32 {
    let lines = input.to_vec();
    let ans = sum_of_two(lines);
    assert_eq!(ans, 4721);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn test_explode() {
        let mut line = Line::parse("[[[[[9,8],1],2],3],4]");
        let res = line.explode();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[[[[0,9],2],3],4]");

        let mut line = Line::parse("[7,[6,[5,[4,[3,2]]]]]");
        let res = line.explode();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut line = Line::parse("[[6,[5,[4,[3,2]]]],1]");
        let res = line.explode();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[[6,[5,[7,0]]],3]");

        let mut line = Line::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let res = line.explode();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let mut line = Line::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let res = line.explode();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_splice() {
        let mut line = Line {
            data: vec![Element::Number(10)],
        };
        let res = line.split();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[5,5]");

        let mut line = Line {
            data: vec![Element::Number(11)],
        };
        let res = line.split();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[5,6]");

        let mut line = Line {
            data: vec![Element::Number(12)],
        };
        let res = line.split();
        assert_eq!(res, true);
        assert_eq!(line.to_string(), "[6,6]");
    }

    #[test]
    fn test_reduce() {
        let mut line = Line::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        line.reduce();
        assert_eq!(line.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add() {
        let mut line = Line::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let other = Line::parse("[1,1]");
        line.add(other);
        assert_eq!(line.to_string(), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        line.reduce();
        assert_eq!(line.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_magnitude() {
        let mut line = Line::parse("[9,1]");
        assert_eq!(line.magnitude(), 29);

        let mut line = Line::parse("[1,9]");
        assert_eq!(line.magnitude(), 21);

        let mut line = Line::parse("[[9,1],[1,9]]");
        assert_eq!(line.magnitude(), 129);

        let mut line = Line::parse("[[1,2],[[3,4],5]]");
        assert_eq!(line.magnitude(), 143);

        let mut line = Line::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(line.magnitude(), 1384);

        let mut line = Line::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(line.magnitude(), 445);

        let mut line = Line::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(line.magnitude(), 791);

        let mut line = Line::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(line.magnitude(), 1137);

        let mut line = Line::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(line.magnitude(), 3488);
    }

    #[test]
    fn test_sum_list() {
        let lines = input_generator(
            "\
[1,1]
[2,2]
[3,3]
[4,4]
",
        );
        let result = sum_list(lines);
        assert_eq!(result.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_do_homework() {
        let lines = input_generator(EXAMPLE_INPUT);
        let result = do_homework(lines);
        assert_eq!(result, 4140);
    }

    #[test]
    fn test_sum_of_two() {
        let lines = input_generator(EXAMPLE_INPUT);
        let max = sum_of_two(lines);
        assert_eq!(max, 3993);
    }
}
