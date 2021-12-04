/*
    --- Day 4: Giant Squid ---
    You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

    Maybe it wants to play bingo?

    Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

    The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
    After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

    22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
     8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
     6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
     1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:

    22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
     8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
     6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
     1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    Finally, 24 is drawn:

    22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
     8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
    21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
     6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
     1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
    At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).

    The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.

    To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?

    --- Part Two ---
    On the other hand, it might be wise to try a different strategy: let the giant squid win.

    You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.

    In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.

    Figure out which board will win last. Once it wins, what would its final score be?
*/

use crate::common::{trim_start, unsigned};
use nom::{
    character::complete::char,
    multi::{many1, many_m_n, separated_list1},
    IResult,
};

#[derive(Clone)]
pub struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl BingoGame {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, numbers) = separated_list1(trim_start(char(',')), unsigned)(input)?;
        let (input, boards) = many1(Board::parser)(input)?;

        Ok((input, Self { numbers, boards }))
    }

    fn play_to_win(&mut self) -> u32 {
        for n in &self.numbers {
            for b in &mut self.boards {
                if let Some(score) = b.draw_number(*n) {
                    return score;
                }
            }
        }
        panic!("Error: all numbers drawn without any boards winning!");
    }

    fn play_to_lose(&mut self) -> u32 {
        for n in &self.numbers {
            let mut retain = vec![true; self.boards.len()];
            let mut last_score = None;
            for (b, r) in self.boards.iter_mut().zip(retain.iter_mut()) {
                if let Some(score) = b.draw_number(*n) {
                    *r = false;
                    last_score = Some(score);
                }
            }

            let mut retain_iter = retain.into_iter();
            self.boards.retain(|_| retain_iter.next().unwrap());

            if self.boards.is_empty() == true {
                return last_score.unwrap();
            }
        }
        panic!("Error: all numbers drawn without all boards winning!");
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    grid: [[u32; 5]; 5],
    state: [[bool; 5]; 5],
}

impl Board {
    fn parser(input: &str) -> IResult<&str, Self> {
        // TODO: pretty sure nom's fill can be used here, but ran into some incompatibilities between Fn and FnMut parser functions
        // let mut buf = [0u32; 5];
        // let (input, ()) = fill(trim_start(unsigned), &mut buf)(input)?;
        let (input, mut grid_vec) = many_m_n(5, 5, many_m_n(5, 5, trim_start(unsigned)))(input)?;
        let grid = [
            grid_vec.remove(0).try_into().unwrap(),
            grid_vec.remove(0).try_into().unwrap(),
            grid_vec.remove(0).try_into().unwrap(),
            grid_vec.remove(0).try_into().unwrap(),
            grid_vec.remove(0).try_into().unwrap(),
        ];

        Ok((
            input,
            Self {
                grid,
                state: [[false; 5]; 5],
            },
        ))
    }

    fn mark_number(&mut self, number: u32) {
        for row_idx in 0..5 {
            for col_idx in 0..5 {
                if self.grid[row_idx][col_idx] == number {
                    assert!(self.state[row_idx][col_idx] == false); // Bingo shouldn't draw the same number twice
                    self.state[row_idx][col_idx] = true;
                    return;
                }
            }
        }
    }

    fn winner(&self) -> bool {
        for row_idx in 0..5 {
            if (0..5).all(|x| self.state[row_idx][x] == true) {
                return true;
            }
        }
        for col_idx in 0..5 {
            if (0..5).all(|x| self.state[x][col_idx] == true) {
                return true;
            }
        }

        false
    }

    fn calc_score(&self, number: u32) -> u32 {
        let mut sum = 0;
        for row_idx in 0..5 {
            for col_idx in 0..5 {
                if self.state[row_idx][col_idx] == false {
                    sum += self.grid[row_idx][col_idx];
                }
            }
        }
        sum * number
    }

    fn draw_number(&mut self, number: u32) -> Option<u32> {
        self.mark_number(number);
        if self.winner() == true {
            Some(self.calc_score(number))
        } else {
            None
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> BingoGame {
    BingoGame::parser(input).unwrap().1
}

#[aoc(day4, part1)]
pub fn part1(input: &BingoGame) -> u32 {
    let mut game = input.clone();
    let score = game.play_to_win();
    assert_eq!(score, 49686);
    score
}

#[aoc(day4, part2)]
pub fn part2(input: &BingoGame) -> u32 {
    let mut game = input.clone();
    let score = game.play_to_lose();
    assert_eq!(score, 26878);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_draw_numbers() {
        fn str_to_ary(s: &str) -> [bool; 5] {
            let mut s_iter = s.chars();
            [
                s_iter.next().unwrap() == 'T',
                s_iter.next().unwrap() == 'T',
                s_iter.next().unwrap() == 'T',
                s_iter.next().unwrap() == 'T',
                s_iter.next().unwrap() == 'T',
            ]
        }

        let mut game = input_generator(EXAMPLE_INPUT);
        let mut numbers_iter = game.numbers.iter();
        for &n in numbers_iter.by_ref().take(5) {
            game.boards[0].draw_number(n);
            game.boards[1].draw_number(n);
            game.boards[2].draw_number(n);
        }

        assert_eq!(
            game.boards[0].state,
            [
                str_to_ary("FFFTF"),
                str_to_ary("FFFTF"),
                str_to_ary("FTFFT"),
                str_to_ary("FFFFT"),
                str_to_ary("FFFFF"),
            ]
        );
        assert_eq!(
            game.boards[1].state,
            [
                str_to_ary("FFFFF"),
                str_to_ary("TFFFT"),
                str_to_ary("FFTFF"),
                str_to_ary("FTFFT"),
                str_to_ary("FFFFF"),
            ]
        );
        assert_eq!(
            game.boards[2].state,
            [
                str_to_ary("FFFFT"),
                str_to_ary("FFFTF"),
                str_to_ary("FFFFF"),
                str_to_ary("FTFFT"),
                str_to_ary("FFFFT"),
            ]
        );

        for &n in numbers_iter.by_ref().take(6) {
            game.boards[0].draw_number(n);
            game.boards[1].draw_number(n);
            game.boards[2].draw_number(n);
        }

        assert_eq!(
            game.boards[0].state,
            [
                str_to_ary("FFTTT"),
                str_to_ary("FTTTF"),
                str_to_ary("TTTFT"),
                str_to_ary("FFFFT"),
                str_to_ary("FFFFF"),
            ]
        );
        assert_eq!(
            game.boards[1].state,
            [
                str_to_ary("FFTTF"),
                str_to_ary("TFFTT"),
                str_to_ary("FFTFT"),
                str_to_ary("FTFFT"),
                str_to_ary("TTFFF"),
            ]
        );
        assert_eq!(
            game.boards[2].state,
            [
                str_to_ary("TTTFT"),
                str_to_ary("FFFTF"),
                str_to_ary("FFTFF"),
                str_to_ary("FTFFT"),
                str_to_ary("TTFFT"),
            ]
        );

        for &n in numbers_iter.by_ref().take(1) {
            game.boards[0].draw_number(n);
            game.boards[1].draw_number(n);
            game.boards[2].draw_number(n);
        }

        assert_eq!(
            game.boards[0].state,
            [
                str_to_ary("FFTTT"),
                str_to_ary("FTTTT"),
                str_to_ary("TTTFT"),
                str_to_ary("FFFFT"),
                str_to_ary("FFFFF"),
            ]
        );
        assert_eq!(
            game.boards[1].state,
            [
                str_to_ary("FFTTF"),
                str_to_ary("TFFTT"),
                str_to_ary("FFTFT"),
                str_to_ary("FTFTT"),
                str_to_ary("TTFFF"),
            ]
        );
        assert_eq!(
            game.boards[2].state,
            [
                str_to_ary("TTTTT"),
                str_to_ary("FFFTF"),
                str_to_ary("FFTFF"),
                str_to_ary("FTFFT"),
                str_to_ary("TTFFT"),
            ]
        );
    }

    #[test]
    fn test_play_to_win() {
        let mut game = input_generator(EXAMPLE_INPUT);
        let score = game.play_to_win();
        assert_eq!(score, 4512);
    }

    #[test]
    fn test_play_to_lose() {
        let mut game = input_generator(EXAMPLE_INPUT);
        let score = game.play_to_lose();
        assert_eq!(score, 1924);
    }
}
