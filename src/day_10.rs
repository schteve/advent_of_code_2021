/*

*/

#[derive(Clone, Debug, PartialEq)]
enum LineState {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
}

impl LineState {
    fn validate(s: &str) -> Self {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop();
                    if let Some(t) = top {
                        let opposite = match c {
                            ')' => '(',
                            ']' => '[',
                            '}' => '{',
                            '>' => '<',
                            _ => unreachable!(),
                        };
                        if t != opposite {
                            return Self::Corrupted(c);
                        }
                    } else {
                        return Self::Corrupted(c);
                    }
                }
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => panic!("Invalid character found: {}", c),
            }
        }

        if stack.is_empty() == true {
            Self::Valid
        } else {
            Self::Incomplete(stack)
        }
    }

    fn validate_many(input: &[String]) -> Vec<Self> {
        input.iter().map(|x| LineState::validate(x)).collect()
    }

    fn score(&self) -> u64 {
        match self {
            Self::Valid => unimplemented!(),
            Self::Corrupted(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Invalid character scored: {}", c),
            },
            Self::Incomplete(stack) => stack.iter().rev().fold(0, |score, top| {
                score * 5
                    + match top {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            }),
        }
    }
}

fn corrupted_scores(states: &[LineState]) -> impl Iterator<Item = u64> + '_ {
    states
        .iter()
        .filter(|x| matches!(x, LineState::Corrupted(_)))
        .map(LineState::score)
}

fn incomplete_scores(states: &[LineState]) -> impl Iterator<Item = u64> + '_ {
    states
        .iter()
        .filter(|x| matches!(x, LineState::Incomplete(_)))
        .map(LineState::score)
}

fn total_corrupted(states: &[LineState]) -> u64 {
    corrupted_scores(states).sum()
}

fn middle_incomplete(states: &[LineState]) -> u64 {
    let mut scores: Vec<u64> = incomplete_scores(states).collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_owned).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> u64 {
    let states = LineState::validate_many(input);
    let total_score = total_corrupted(&states);
    assert_eq!(total_score, 318099);
    total_score
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> u64 {
    let states = LineState::validate_many(input);
    let middle_score = middle_incomplete(&states);
    assert_eq!(middle_score, 2389738699);
    middle_score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_validate() {
        let input = input_generator(EXAMPLE_INPUT);
        let states = LineState::validate_many(&input);
        assert_eq!(
            states,
            vec![
                LineState::Incomplete(vec!['[', '(', '{', '(', '[', '[', '{', '{']),
                LineState::Incomplete(vec!['(', '{', '[', '<', '{', '(']),
                LineState::Corrupted('}'),
                LineState::Incomplete(vec!['(', '(', '(', '(', '<', '{', '<', '{', '{']),
                LineState::Corrupted(')'),
                LineState::Corrupted(']'),
                LineState::Incomplete(vec!['<', '{', '[', '{', '[', '{', '{', '[', '[']),
                LineState::Corrupted(')'),
                LineState::Corrupted('>'),
                LineState::Incomplete(vec!['<', '{', '(', '[']),
            ]
        );

        let scores: Vec<_> = states
            .iter()
            .filter(|x| matches!(x, LineState::Corrupted(_)))
            .map(LineState::score)
            .collect();
        assert_eq!(scores, vec![1197, 3, 57, 3, 25137]);
    }

    #[test]
    fn test_corrupted_scores() {
        let input = input_generator(EXAMPLE_INPUT);
        let states = LineState::validate_many(&input);

        let scores: Vec<_> = corrupted_scores(&states).collect();
        assert_eq!(scores, vec![1197, 3, 57, 3, 25137]);

        let total_score = total_corrupted(&states);
        assert_eq!(total_score, 26397);
    }

    #[test]
    fn test_incomplete_scores() {
        let input = input_generator(EXAMPLE_INPUT);
        let states = LineState::validate_many(&input);

        let scores: Vec<_> = incomplete_scores(&states).collect();
        assert_eq!(scores, vec![288957, 5566, 1480781, 995444, 294]);

        let middle_score = middle_incomplete(&states);
        assert_eq!(middle_score, 288957);
    }
}
