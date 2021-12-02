/*
    --- Day 2: Dive! ---
    Now, you need to figure out how to pilot this thing.

    It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.
    Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

    The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.
    After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

    Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

    --- Part Two ---
    Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.

    In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

    down X increases your aim by X units.
    up X decreases your aim by X units.
    forward X does two things:
    It increases your horizontal position by X units.
    It increases your depth by your aim multiplied by X.
    Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.

    Now, the above example does something different:

    forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
    down 5 adds 5 to your aim, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
    up 3 decreases your aim by 3, resulting in a value of 2.
    down 8 adds 8 to your aim, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
    After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)

    Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
*/

use crate::common::{trim_start, unsigned, Point2};
use nom::{
    character::complete::{alpha1, char},
    multi::many1,
    sequence::separated_pair,
    IResult,
};

pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, (cmd, arg)) = separated_pair(trim_start(alpha1), char(' '), unsigned)(input)?;

        let command = match cmd {
            "forward" => Self::Forward(arg),
            "down" => Self::Down(arg),
            "up" => Self::Up(arg),
            _ => panic!("Unknown command"),
        };

        Ok((input, command))
    }
}

struct Submarine {
    pos: Point2,
    aim: u32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            pos: Point2::origin(),
            aim: 0,
        }
    }

    fn follow_commands1(&mut self, commands: &[Command]) {
        for cmd in commands {
            match cmd {
                Command::Forward(d) => self.pos.x += *d as i32,
                Command::Down(d) => self.pos.y += *d as i32,
                Command::Up(d) => self.pos.y -= *d as i32,
            }
        }
    }

    fn follow_commands2(&mut self, commands: &[Command]) {
        for cmd in commands {
            match cmd {
                Command::Forward(d) => {
                    self.pos.x += *d as i32;
                    self.pos.y += (self.aim * d) as i32;
                }
                Command::Down(d) => self.aim += *d,
                Command::Up(d) => self.aim -= *d,
            }
        }
    }

    fn calc_pos_score(&self) -> i32 {
        self.pos.x * self.pos.y
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    many1(Command::parser)(input).unwrap().1
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> i32 {
    let mut sub = Submarine::new();
    sub.follow_commands1(input);
    let score = sub.calc_pos_score();
    assert_eq!(score, 1840243);
    score
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> i32 {
    let mut sub = Submarine::new();
    sub.follow_commands2(input);
    let score = sub.calc_pos_score();
    assert_eq!(score, 1727785422);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_follow_commands1() {
        let mut sub = Submarine::new();
        let input = input_generator(EXAMPLE_INPUT);
        sub.follow_commands1(&input);
        assert_eq!(sub.pos, Point2 { x: 15, y: 10 });

        let score = sub.calc_pos_score();
        assert_eq!(score, 150);
    }

    #[test]
    fn test_follow_commands2() {
        let mut sub = Submarine::new();
        let input = input_generator(EXAMPLE_INPUT);
        sub.follow_commands2(&input);
        assert_eq!(sub.pos, Point2 { x: 15, y: 60 });

        let score = sub.calc_pos_score();
        assert_eq!(score, 900);
    }
}
