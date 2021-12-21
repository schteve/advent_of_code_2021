/*
    --- Day 21: Dirac Dice ---
    There's not much to do as you slowly descend to the bottom of the ocean. The submarine computer challenges you to a nice game of Dirac Dice.

    This game consists of a single die, two pawns, and a game board with a circular track containing ten spaces marked 1 through 10 clockwise. Each player's starting space is chosen randomly (your puzzle input). Player 1 goes first.

    Players take turns moving. On each player's turn, the player rolls the die three times and adds up the results. Then, the player moves their pawn that many times forward around the track (that is, moving clockwise on spaces in order of increasing value, wrapping back around to 1 after 10). So, if a player is on space 7 and they roll 2, 2, and 1, they would move forward 5 times, to spaces 8, 9, 10, 1, and finally stopping on 2.

    After each player moves, they increase their score by the value of the space their pawn stopped on. Players' scores start at 0. So, if the first player starts on space 7 and rolls a total of 5, they would stop on space 2 and add 2 to their score (for a total score of 2). The game immediately ends as a win for any player whose score reaches at least 1000.

    Since the first game is a practice game, the submarine opens a compartment labeled deterministic dice and a 100-sided die falls out. This die always rolls 1 first, then 2, then 3, and so on up to 100, after which it starts over at 1 again. Play using this die.

    For example, given these starting positions:

    Player 1 starting position: 4
    Player 2 starting position: 8
    This is how the game would go:

    Player 1 rolls 1+2+3 and moves to space 10 for a total score of 10.
    Player 2 rolls 4+5+6 and moves to space 3 for a total score of 3.
    Player 1 rolls 7+8+9 and moves to space 4 for a total score of 14.
    Player 2 rolls 10+11+12 and moves to space 6 for a total score of 9.
    Player 1 rolls 13+14+15 and moves to space 6 for a total score of 20.
    Player 2 rolls 16+17+18 and moves to space 7 for a total score of 16.
    Player 1 rolls 19+20+21 and moves to space 6 for a total score of 26.
    Player 2 rolls 22+23+24 and moves to space 6 for a total score of 22.
    ...after many turns...

    Player 2 rolls 82+83+84 and moves to space 6 for a total score of 742.
    Player 1 rolls 85+86+87 and moves to space 4 for a total score of 990.
    Player 2 rolls 88+89+90 and moves to space 3 for a total score of 745.
    Player 1 rolls 91+92+93 and moves to space 10 for a final score, 1000.
    Since player 1 has at least 1000 points, player 1 wins and the game ends. At this point, the losing player had 745 points and the die had been rolled a total of 993 times; 745 * 993 = 739785.

    Play a practice game using the deterministic 100-sided die. The moment either player wins, what do you get if you multiply the score of the losing player by the number of times the die was rolled during the game?

    The first half of this puzzle is complete! It provides one gold star: *

    --- Part Two ---
    Now that you're warmed up, it's time to play the real game.

    A second compartment opens, this time labeled Dirac dice. Out of it falls a single three-sided die.

    As you experiment with the die, you feel a little strange. An informational brochure in the compartment explains that this is a quantum die: when you roll it, the universe splits into multiple copies, one copy for each possible outcome of the die. In this case, rolling the die always splits the universe into three copies: one where the outcome of the roll was 1, one where it was 2, and one where it was 3.

    The game is played the same as before, although to prevent things from getting too far out of hand, the game now ends when either player's score reaches at least 21.

    Using the same starting positions as in the example above, player 1 wins in 444356092776315 universes, while player 2 merely wins in 341960390180808 universes.

    Using your given starting positions, determine every possible outcome. Find the player that wins in more universes; in how many universes does that player win?
*/

use crate::common::{modulo, unsigned};
use nom::{bytes::complete::tag, character::complete::multispace0, sequence::tuple, IResult};
use std::{cmp::max, collections::HashMap};

struct DeterministicGame {
    start: (u8, u8),
    p1: (u8, u32),    // Current pawn, score
    p2: (u8, u32),    // Current pawn, score
    whose_turn: bool, // false = p1, true = p2
    next_roll: u8,
    rolled_count: u32,
}

impl DeterministicGame {
    fn from_starting(start: &(u8, u8)) -> Self {
        Self {
            start: *start,
            p1: (start.0, 0),
            p2: (start.1, 0),
            whose_turn: false,
            next_roll: 1,
            rolled_count: 0,
        }
    }

    fn get_3_rolls(&mut self) -> u8 {
        let r1 = self.next_roll;
        let r2 = self.next_roll + 1;
        let r3 = self.next_roll + 2;
        self.next_roll = modulo(self.next_roll + 2, 100) + 1;
        self.rolled_count += 3;
        modulo(r1 as u32 + r2 as u32 + r3 as u32 - 1, 100) as u8 + 1
    }

    fn turn(&mut self) -> Option<bool> {
        let this_roll = self.get_3_rolls();
        if self.whose_turn == false {
            // P1 turn
            self.p1.0 = modulo(self.p1.0 + this_roll - 1, 10) + 1;
            self.p1.1 += self.p1.0 as u32;
            if self.p1.1 >= 1000 {
                // Winner!
                return Some(false);
            }
        } else {
            // P2 turn
            self.p2.0 = modulo(self.p2.0 + this_roll - 1, 10) + 1;
            self.p2.1 += self.p2.0 as u32;
            if self.p2.1 >= 1000 {
                // Winner!
                return Some(true);
            }
        }

        self.whose_turn = !self.whose_turn;
        None
    }

    fn play(&mut self) -> u32 {
        loop {
            let winner = self.turn();
            if let Some(who) = winner {
                if who == false {
                    return self.p2.1 * self.rolled_count;
                } else {
                    return self.p1.1 * self.rolled_count;
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct DiracState {
    p1: (u8, u32),    // Current pawn, score
    p2: (u8, u32),    // Current pawn, score
    whose_turn: bool, // false = p1, true = p2
}

impl DiracState {
    fn turn(&mut self, roll: u8) -> Option<bool> {
        if self.whose_turn == false {
            // P1 turn
            self.p1.0 = modulo(self.p1.0 + roll - 1, 10) + 1;
            self.p1.1 += self.p1.0 as u32;
            if self.p1.1 >= 21 {
                // Winner!
                return Some(false);
            }
        } else {
            // P2 turn
            self.p2.0 = modulo(self.p2.0 + roll - 1, 10) + 1;
            self.p2.1 += self.p2.0 as u32;
            if self.p2.1 >= 21 {
                // Winner!
                return Some(true);
            }
        }

        self.whose_turn = !self.whose_turn;
        None
    }
}

struct DiracGame {
    start: (u8, u8),
    universes: HashMap<DiracState, u64>,
    p1_wins: u64,
    p2_wins: u64,
}

impl DiracGame {
    fn from_starting(start: &(u8, u8)) -> Self {
        let mut universes = HashMap::new();
        universes.insert(
            DiracState {
                p1: (start.0, 0),
                p2: (start.1, 0),
                whose_turn: false,
            },
            1,
        );

        Self {
            start: *start,
            universes,
            p1_wins: 0,
            p2_wins: 0,
        }
    }

    fn split(&mut self) {
        let mut next = HashMap::new();
        for (curr_state, curr_count) in self.universes.iter() {
            for (roll, roll_count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let mut next_state = curr_state.clone();
                if let Some(winner) = next_state.turn(roll) {
                    if winner == false {
                        self.p1_wins += curr_count * roll_count;
                    } else {
                        self.p2_wins += curr_count * roll_count;
                    }
                } else {
                    let entry = next.entry(next_state).or_insert(0);
                    *entry += curr_count * roll_count;
                }
            }
        }
        self.universes = next;
    }

    fn play(&mut self) -> u64 {
        loop {
            if self.universes.is_empty() == true {
                return max(self.p1_wins, self.p2_wins);
            } else {
                self.split();
            }
        }
    }
}

fn start_parser(input: &str) -> IResult<&str, (u8, u8)> {
    let (_, (_, p1, _, _, p2)) = tuple((
        tag("Player 1 starting position: "),
        unsigned,
        multispace0,
        tag("Player 2 starting position: "),
        unsigned,
    ))(input)?;

    Ok((input, (p1, p2)))
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (u8, u8) {
    start_parser(input).unwrap().1
}

#[aoc(day21, part1)]
pub fn part1(input: &(u8, u8)) -> u32 {
    let mut game = DeterministicGame::from_starting(input);
    let score = game.play();
    assert_eq!(score, 711480);
    score
}

#[aoc(day21, part2)]
pub fn part2(input: &(u8, u8)) -> u64 {
    let mut game = DiracGame::from_starting(input);
    let score = game.play();
    assert_eq!(score, 265845890886828);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_turn() {
        let starting = input_generator(EXAMPLE_INPUT);
        let mut game = DeterministicGame::from_starting(&starting);

        game.turn();
        assert_eq!(game.p1, (10, 10));

        game.turn();
        assert_eq!(game.p2, (3, 3));

        game.turn();
        assert_eq!(game.p1, (4, 14));

        game.turn();
        assert_eq!(game.p2, (6, 9));

        game.turn();
        assert_eq!(game.p1, (6, 20));

        game.turn();
        assert_eq!(game.p2, (7, 16));

        game.turn();
        assert_eq!(game.p1, (6, 26));

        game.turn();
        assert_eq!(game.p2, (6, 22));
    }

    #[test]
    fn test_play() {
        let starting = input_generator(EXAMPLE_INPUT);
        let mut game = DeterministicGame::from_starting(&starting);

        let score = game.play();
        assert_eq!(game.p1, (10, 1000));
        assert_eq!(game.p2, (3, 745));
        assert_eq!(game.rolled_count, 993);
        assert_eq!(score, 739785)
    }

    #[test]
    fn test_dirac() {
        let starting = input_generator(EXAMPLE_INPUT);
        let mut game = DiracGame::from_starting(&starting);
        let score = game.play();
        assert_eq!(game.p1_wins, 444356092776315);
        assert_eq!(game.p2_wins, 341960390180808);
        assert_eq!(score, 444356092776315);
    }
}
