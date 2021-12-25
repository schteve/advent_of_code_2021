/*
    --- Day 24: Arithmetic Logic Unit ---
    Magic smoke starts leaking from the submarine's arithmetic logic unit (ALU). Without the ability to perform basic arithmetic and logic functions, the submarine can't produce cool patterns with its Christmas lights!

    It also can't navigate. Or run the oxygen system.

    Don't worry, though - you probably have enough oxygen left to give you enough time to build a new ALU.

    The ALU is a four-dimensional processing unit: it has integer variables w, x, y, and z. These variables all start with the value 0. The ALU also supports six instructions:

    inp a - Read an input value and write it to variable a.
    add a b - Add the value of a to the value of b, then store the result in variable a.
    mul a b - Multiply the value of a by the value of b, then store the result in variable a.
    div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
    eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
    In all of these instructions, a and b are placeholders; a will always be the variable where the result of the operation is stored (one of w, x, y, or z), while b can be either a variable or a number. Numbers can be positive or negative, but will always be integers.

    The ALU has no jump instructions; in an ALU program, every instruction is run exactly once in order from top to bottom. The program halts after the last instruction has finished executing.

    (Program authors should be especially cautious; attempting to execute div with b=0 or attempting to execute mod with a<0 or b<=0 will cause the program to crash and might even damage the ALU. These operations are never intended in any serious ALU program.)

    For example, here is an ALU program which takes an input number, negates it, and stores it in x:

    inp x
    mul x -1
    Here is an ALU program which takes two input numbers, then sets z to 1 if the second input number is three times larger than the first input number, or sets z to 0 otherwise:

    inp z
    inp x
    mul z 3
    eql z x
    Here is an ALU program which takes a non-negative integer as input, converts it into binary, and stores the lowest (1's) bit in z, the second-lowest (2's) bit in y, the third-lowest (4's) bit in x, and the fourth-lowest (8's) bit in w:

    inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2
    Once you have built a replacement ALU, you can install it in the submarine, which will immediately resume what it was doing when the ALU failed: validating the submarine's model number. To do this, the ALU will run the MOdel Number Automatic Detector program (MONAD, your puzzle input).

    Submarine model numbers are always fourteen-digit numbers consisting only of digits 1 through 9. The digit 0 cannot appear in a model number.

    When MONAD checks a hypothetical fourteen-digit model number, it uses fourteen separate inp instructions, each expecting a single digit of the model number in order of most to least significant. (So, to check the model number 13579246899999, you would give 1 to the first inp instruction, 3 to the second inp instruction, 5 to the third inp instruction, and so on.) This means that when operating MONAD, each input instruction should only ever be given an integer value of at least 1 and at most 9.

    Then, after MONAD has finished running all of its instructions, it will indicate that the model number was valid by leaving a 0 in variable z. However, if the model number was invalid, it will leave some other non-zero value in z.

    MONAD imposes additional, mysterious restrictions on model numbers, and legend says the last copy of the MONAD documentation was eaten by a tanuki. You'll need to figure out what MONAD does some other way.

    To enable as many submarine features as possible, find the largest valid fourteen-digit model number that contains no 0 digits. What is the largest model number accepted by MONAD?

    --- Part Two ---
    As the submarine starts booting up things like the Retro Encabulator, you realize that maybe you don't need all these submarine features after all.

    What is the smallest model number accepted by MONAD?
*/

use crate::common::signed;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, one_of},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    fn from_char(c: char) -> Self {
        match c {
            'w' => Self::W,
            'x' => Self::X,
            'y' => Self::Y,
            'z' => Self::Z,
            _ => panic!("Invalid reg: {}", c),
        }
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, me) = map(one_of("wxyz"), Self::from_char)(input)?;
        Ok((input, me))
    }
}

pub enum Operand {
    Reg(Reg),
    Num(i64),
}

impl Operand {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, me) = alt((map(Reg::parser, Self::Reg), map(signed, Self::Num)))(input)?;
        Ok((input, me))
    }
}

pub enum Op {
    Input(Reg),
    Add(Reg, Operand),
    Mul(Reg, Operand),
    Div(Reg, Operand),
    Mod(Reg, Operand),
    Eql(Reg, Operand),
}

impl Op {
    fn many_from_string(input: &str) -> Vec<Self> {
        let mut output = Vec::new();
        for line in input.lines() {
            let mut parts = line.split(' ');
            let kind = parts.next().unwrap();
            let reg = parts.next().unwrap();
            let operand = parts.next();

            let reg = Reg::from_char(reg.chars().next().unwrap());
            let operand = if let Some(o) = operand {
                if let Ok(x) = o.parse::<i64>() {
                    Some(Operand::Num(x))
                } else {
                    Some(Operand::Reg(Reg::from_char(o.chars().next().unwrap())))
                }
            } else {
                None
            };

            let me = match kind {
                "inp" => Self::Input(reg),
                "add" => Self::Add(reg, operand.unwrap()),
                "mul" => Self::Mul(reg, operand.unwrap()),
                "div" => Self::Div(reg, operand.unwrap()),
                "mod" => Self::Mod(reg, operand.unwrap()),
                "eql" => Self::Eql(reg, operand.unwrap()),
                _ => panic!("Invalid op: \"{}\"", kind),
            };
            output.push(me);
        }
        output
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, me) = preceded(
            multispace0,
            alt((
                preceded(tag("inp "), map(Reg::parser, Self::Input)),
                preceded(
                    tag("add "),
                    map(
                        separated_pair(Reg::parser, char(' '), Operand::parser),
                        |(r, o)| Self::Add(r, o),
                    ),
                ),
                preceded(
                    tag("mul "),
                    map(
                        separated_pair(Reg::parser, char(' '), Operand::parser),
                        |(r, o)| Self::Mul(r, o),
                    ),
                ),
                preceded(
                    tag("div "),
                    map(
                        separated_pair(Reg::parser, char(' '), Operand::parser),
                        |(r, o)| Self::Div(r, o),
                    ),
                ),
                preceded(
                    tag("mod "),
                    map(
                        separated_pair(Reg::parser, char(' '), Operand::parser),
                        |(r, o)| Self::Mod(r, o),
                    ),
                ),
                preceded(
                    tag("eql "),
                    map(
                        separated_pair(Reg::parser, char(' '), Operand::parser),
                        |(r, o)| Self::Eql(r, o),
                    ),
                ),
            )),
        )(input)?;

        Ok((input, me))
    }
}

struct Alu {
    regs: [i64; 4],
}

impl Alu {
    fn new() -> Self {
        Self { regs: [0; 4] }
    }

    fn reg(&self, reg: &Reg) -> i64 {
        match reg {
            Reg::W => self.regs[0],
            Reg::X => self.regs[1],
            Reg::Y => self.regs[2],
            Reg::Z => self.regs[3],
        }
    }

    fn reg_mut(&mut self, reg: &Reg) -> &mut i64 {
        match reg {
            Reg::W => &mut self.regs[0],
            Reg::X => &mut self.regs[1],
            Reg::Y => &mut self.regs[2],
            Reg::Z => &mut self.regs[3],
        }
    }

    fn execute<I>(&mut self, ops: &[Op], mut input: I)
    where
        I: Iterator<Item = i64>,
    {
        for op in ops {
            match op {
                Op::Input(r) => {
                    let a = self.reg_mut(r);
                    *a = input.next().unwrap();
                }
                Op::Add(r, o) => {
                    let b = match o {
                        Operand::Reg(or) => self.reg(or),
                        Operand::Num(num) => *num,
                    };
                    let a = self.reg_mut(r);
                    *a += b;
                }
                Op::Mul(r, o) => {
                    let b = match o {
                        Operand::Reg(or) => self.reg(or),
                        Operand::Num(num) => *num,
                    };
                    let a = self.reg_mut(r);
                    *a *= b;
                }
                Op::Div(r, o) => {
                    let b = match o {
                        Operand::Reg(or) => self.reg(or),
                        Operand::Num(num) => *num,
                    };
                    let a = self.reg_mut(r);
                    assert_ne!(b, 0);
                    *a /= b;
                }
                Op::Mod(r, o) => {
                    let b = match o {
                        Operand::Reg(or) => self.reg(or),
                        Operand::Num(num) => *num,
                    };
                    let a = self.reg_mut(r);
                    assert!(*a >= 0);
                    assert!(b > 0);
                    *a %= b;
                }
                Op::Eql(r, o) => {
                    let b = match o {
                        Operand::Reg(or) => self.reg(or),
                        Operand::Num(num) => *num,
                    };
                    let a = self.reg_mut(r);
                    *a = if *a == b { 1 } else { 0 };
                }
            }
        }
    }
}

fn digits_from_n(mut n: i64) -> Vec<i64> {
    assert!(n > 0);

    let mut digits = Vec::new();
    while n > 0 {
        let d = n % 10;
        n /= 10;
        digits.push(d);
    }
    digits.reverse();
    assert_eq!(digits.len(), 14);
    digits
}

fn digits_to_n(digits: [i64; 14]) -> i64 {
    let s = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        digits[0],
        digits[1],
        digits[2],
        digits[3],
        digits[4],
        digits[5],
        digits[6],
        digits[7],
        digits[8],
        digits[9],
        digits[10],
        digits[11],
        digits[12],
        digits[13]
    );
    s.parse().unwrap()
}

fn monad_is_valid(ops: &[Op], n: i64) -> bool {
    let mut alu = Alu::new();
    alu.execute(ops, digits_from_n(n).into_iter());
    alu.reg(&Reg::Z) == 0
}

/*
    I couldn't figure out how to programmatically reach the answers so instead I analyzed the input program. Through
    observation there are 14 blocks of code which are all nearly identical except for a few specific constants. Each
    block begins with an input statement and ends just before the next input statement. Thus there is one block per
    input digit. Each block looks like this:

        inp w       w = input()
        mul x 0     x = 0
        add x z     x += z
        mod x 26    x %= 26
        div z A     z /= A
        add x B     x += B
        eql x w     x = (x == w)
        eql x 0     x = (x == 0)
        mul y 0     y = 0
        add y 25    y += 25
        mul y x     y *= x
        add y 1     y += 1
        mul z y     z *= y
        mul y 0     y = 0
        add y w     y += w
        add y C     y += C
        mul y x     y *= x
        add z y     z += y

    The equivalent code for each block is something like this:

        x = z % 26 + B
        z /= A
        if w != x {
            z = z * 26 + w + C
        }

    Here, w is the input for each block (along with z from the previous block). z is the output. x and y are used as
    scratch registers so values are not carried between blocks. A, B, and C are constants that vary from block to
    block. Their values are listed below in the table.

    The insight for how to analyze this comes from a few things:
    1. A is only 1 or 26. We either keep z the same or divide it by 26. When dividing, this is like shifting the value
        to the right.
    2. If w does NOT match some value derived from z, z is multiplied by 26 and some value is added to it. When
        multiplying, this is like shifting the value to the left. If w DOES match the value, z is kept the same.
    3. When A is 1, B is only ever > 10. Because the value compared with w starts from z % 26 and is added with
        a value > 10, it won't ever match. Thus when A is 1, z is multiplied by 26 and added with w + C and the
        value of B doesn't matter.
    4. When A is 26, B is only ever negative. Combined with the C value which was shifted left into z previously,
        this means in order to avoid re-shifting left the input value needs to match C from a previous digit
        plus B from this digit.
    5. In order to pass the test, the final output value (z) needs to be 0. This means that each left shift needs
        to be paired with a right shift, which implies that each time that A is 26 there is a matching digit earlier
        where A is 1. When A is 26 the input value is matched against a previous value (plus some offset) and if any
        value doesn't match the final output (z) will not be 0 (since it was shifted left more times than it was
        shifted right).

    This table shows the constant values for each digit as well as the pairings of left shifts and right shifts.

        digit   A       B       C
        ----------------------------
        1       1       11      6   -------\
        2       1       13      14  -----\ |
        3       1       15      14  -\   | |
        4       26      -8      10  -/   | |
        5       1       13      9   ---\ | |
        6       1       15      12  -\ | | |
        7       26      -11     8   -/ | | |
        8       26      -4      13  ---/ | |
        9       26      -15     12  -----/ |
        10      1       14      6   ---\   |
        11      1       14      9   -\ |   |
        12      26      -1      15  -/ |   |
        13      26      -8      4   ---/   |
        14      26      -14     10  -------/

    Based on the values above, the requirements on each digit are as follows:

        w4 == w3 + 14 - 8    => w4 == w3 + 6
        w7 == w6 + 12 - 11   => w7 == w6 + 1
        w8 == w5 + 9 - 4     => w8 == w5 + 5
        w9 == w2 + 14 - 15   => w9 == w2 - 1
        w12 == w11 + 9 - 1   => w12 == w11 + 8
        w13 == w10 + 6 - 8   => w13 == w10 - 2
        w14 == w1 + 6 - 14   => w14 == w1 - 8

    Working backwards, this means we have these possible ranges / requirements for each digit:

        w1  9..=9
        w2  2..=9
        w3  1..=6
        w4  w3 + 6
        w5  1..=4
        w6  1..=8
        w7  w6 + 1
        w8  w5 + 5
        w9  w2 - 1
        w10 3..=9
        w11 1..=1
        w12 w11 + 8
        w13 w10 - 2
        w14 w1 - 8

    In order to find the highest (lowest) valid value, we can then pick the highest (lowest) possible value for each
    digit and assign the others according to the requirements.

    Because the above analysis gives a pre-determined answer, the implementation of the ALU above is not actually used
    except to validate the analysis and its answers.
*/

fn monad_is_valid_shortcut(n: i64) -> bool {
    let digits = digits_from_n(n);
    if digits[3] != digits[2] + 6 // w4 == w3 + 6
        || digits[6] != digits[5] + 1 // w7 == w6 + 1
        || digits[7] != digits[4] + 5 // w8 == w5 + 5
        || digits[8] != digits[1] - 1 // w9 == w2 - 1
        || digits[11] != digits[10] + 8 // w12 == w11 + 8
        || digits[12] != digits[9] - 2 // w13 == w10 - 2
        || digits[13] != digits[0] - 8
    // w14 == w1 - 8
    {
        false
    } else {
        true
    }
}

fn highest_valid() -> i64 {
    let w1 = 9;
    let w2 = 9;
    let w3 = 3;
    let w4 = w3 + 6;
    let w5 = 4;
    let w6 = 8;
    let w7 = w6 + 1;
    let w8 = w5 + 5;
    let w9 = w2 - 1;
    let w10 = 9;
    let w11 = 1;
    let w12 = w11 + 8;
    let w13 = w10 - 2;
    let w14 = w1 - 8;

    digits_to_n([w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14])
}

fn lowest_valid() -> i64 {
    let w1 = 9;
    let w2 = 2;
    let w3 = 1;
    let w4 = w3 + 6;
    let w5 = 1;
    let w6 = 1;
    let w7 = w6 + 1;
    let w8 = w5 + 5;
    let w9 = w2 - 1;
    let w10 = 3;
    let w11 = 1;
    let w12 = w11 + 8;
    let w13 = w10 - 2;
    let w14 = w1 - 8;

    digits_to_n([w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14])
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Op> {
    many1(Op::parser)(input).unwrap().1
}

#[aoc(day24, part1)]
pub fn part1(input: &[Op]) -> i64 {
    let answer = highest_valid();
    assert!(monad_is_valid(input, answer));
    assert_eq!(
        monad_is_valid(input, answer),
        monad_is_valid_shortcut(answer)
    );
    assert_eq!(answer, 99394899891971);
    answer
}

#[aoc(day24, part2)]
pub fn part2(input: &[Op]) -> i64 {
    let answer = lowest_valid();
    assert!(monad_is_valid(input, answer));
    assert_eq!(
        monad_is_valid(input, answer),
        monad_is_valid_shortcut(answer)
    );
    assert_eq!(answer, 92171126131911);
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
inp x
mul x -1
";

    static EXAMPLE_INPUT2: &str = "\
inp z
inp x
mul z 3
eql z x
";

    static EXAMPLE_INPUT3: &str = "\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";

    #[test]
    fn test_examples() {
        let ops = input_generator(EXAMPLE_INPUT1);
        let mut alu = Alu::new();
        alu.execute(&ops, [123].into_iter());
        assert_eq!(alu.reg(&Reg::X), -123);

        let ops = input_generator(EXAMPLE_INPUT1);
        let mut alu = Alu::new();
        alu.execute(&ops, [-456].into_iter());
        assert_eq!(alu.reg(&Reg::X), 456);

        let ops = input_generator(EXAMPLE_INPUT2);
        let mut alu = Alu::new();
        alu.execute(&ops, [123, 369].into_iter());
        assert_eq!(alu.reg(&Reg::Z), 1);

        let ops = input_generator(EXAMPLE_INPUT2);
        let mut alu = Alu::new();
        alu.execute(&ops, [123, 400].into_iter());
        assert_eq!(alu.reg(&Reg::Z), 0);

        let ops = input_generator(EXAMPLE_INPUT3);
        let mut alu = Alu::new();
        alu.execute(&ops, [0].into_iter());
        assert_eq!(alu.reg(&Reg::W), 0);
        assert_eq!(alu.reg(&Reg::X), 0);
        assert_eq!(alu.reg(&Reg::Y), 0);
        assert_eq!(alu.reg(&Reg::Z), 0);

        let ops = input_generator(EXAMPLE_INPUT3);
        let mut alu = Alu::new();
        alu.execute(&ops, [5].into_iter());
        assert_eq!(alu.reg(&Reg::W), 0);
        assert_eq!(alu.reg(&Reg::X), 1);
        assert_eq!(alu.reg(&Reg::Y), 0);
        assert_eq!(alu.reg(&Reg::Z), 1);

        let ops = input_generator(EXAMPLE_INPUT3);
        let mut alu = Alu::new();
        alu.execute(&ops, [0xF].into_iter());
        assert_eq!(alu.reg(&Reg::W), 1);
        assert_eq!(alu.reg(&Reg::X), 1);
        assert_eq!(alu.reg(&Reg::Y), 1);
        assert_eq!(alu.reg(&Reg::Z), 1);
    }

    #[test]
    fn test_monads() {
        let ops = input_generator(include_str!("../input/2021/day24.txt"));
        for w1 in (9..=9).rev() {
            let w14 = w1 - 8;
            for w2 in (2..=9).rev() {
                let w9 = w2 - 1;
                for w3 in (1..=3).rev() {
                    let w4 = w3 + 6;
                    for w5 in (1..=4).rev() {
                        let w8 = w5 + 5;
                        for w6 in (1..=8).rev() {
                            let w7 = w6 + 1;
                            for w10 in (3..=9).rev() {
                                let w13 = w10 - 2;
                                for w11 in (1..=1).rev() {
                                    let w12 = w11 + 8;
                                    let n = digits_to_n([
                                        w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14,
                                    ]);
                                    assert!(monad_is_valid(&ops, n));
                                    assert!(monad_is_valid_shortcut(n));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
