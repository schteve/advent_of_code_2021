/*
    --- Day 25: Sea Cucumber ---
    This is it: the bottom of the ocean trench, the last place the sleigh keys could be. Your submarine's experimental antenna still isn't boosted enough to detect the keys, but they must be here. All you need to do is reach the seafloor and find them.

    At least, you'd touch down on the seafloor if you could; unfortunately, it's completely covered by two large herds of sea cucumbers, and there isn't an open space large enough for your submarine.

    You suspect that the Elves must have done this before, because just then you discover the phone number of a deep-sea marine biologist on a handwritten note taped to the wall of the submarine's cockpit.

    "Sea cucumbers? Yeah, they're probably hunting for food. But don't worry, they're predictable critters: they move in perfectly straight lines, only moving forward when there's space to do so. They're actually quite polite!"

    You explain that you'd like to predict when you could land your submarine.

    "Oh that's easy, they'll eventually pile up and leave enough space for-- wait, did you say submarine? And the only place with that many sea cucumbers would be at the very bottom of the Mariana--" You hang up the phone.

    There are two herds of sea cucumbers sharing the same region; one always moves east (>), while the other always moves south (v). Each location can contain at most one sea cucumber; the remaining locations are empty (.). The submarine helpfully generates a map of the situation (your puzzle input). For example:

    v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>
    Every step, the sea cucumbers in the east-facing herd attempt to move forward one location, then the sea cucumbers in the south-facing herd attempt to move forward one location. When a herd moves forward, every sea cucumber in the herd first simultaneously considers whether there is a sea cucumber in the adjacent location it's facing (even another sea cucumber facing the same direction), and then every sea cucumber facing an empty location simultaneously moves into that location.

    So, in a situation like this:

    ...>>>>>...
    After one step, only the rightmost sea cucumber would have moved:

    ...>>>>.>..
    After the next step, two sea cucumbers move:

    ...>>>.>.>.
    During a single step, the east-facing herd moves first, then the south-facing herd moves. So, given this situation:

    ..........
    .>v....v..
    .......>..
    ..........
    After a single step, of the sea cucumbers on the left, only the south-facing sea cucumber has moved (as it wasn't out of the way in time for the east-facing cucumber on the left to move), but both sea cucumbers on the right have moved (as the east-facing sea cucumber moved out of the way of the south-facing sea cucumber):

    ..........
    .>........
    ..v....v>.
    ..........
    Due to strong water currents in the area, sea cucumbers that move off the right edge of the map appear on the left edge, and sea cucumbers that move off the bottom edge of the map appear on the top edge. Sea cucumbers always check whether their destination location is empty before moving, even if that destination is on the opposite side of the map:

    Initial state:
    ...>...
    .......
    ......>
    v.....>
    ......>
    .......
    ..vvv..

    After 1 step:
    ..vv>..
    .......
    >......
    v.....>
    >......
    .......
    ....v..

    After 2 steps:
    ....v>.
    ..vv...
    .>.....
    ......>
    v>.....
    .......
    .......

    After 3 steps:
    ......>
    ..v.v..
    ..>v...
    >......
    ..>....
    v......
    .......

    After 4 steps:
    >......
    ..v....
    ..>.v..
    .>.v...
    ...>...
    .......
    v......
    To find a safe place to land your submarine, the sea cucumbers need to stop moving. Again consider the first example:

    Initial state:
    v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>

    After 1 step:
    ....>.>v.>
    v.v>.>v.v.
    >v>>..>v..
    >>v>v>.>.v
    .>v.v...v.
    v>>.>vvv..
    ..v...>>..
    vv...>>vv.
    >.v.v..v.v

    After 2 steps:
    >.v.v>>..v
    v.v.>>vv..
    >v>.>.>.v.
    >>v>v.>v>.
    .>..v....v
    .>v>>.v.v.
    v....v>v>.
    .vv..>>v..
    v>.....vv.

    After 3 steps:
    v>v.v>.>v.
    v...>>.v.v
    >vv>.>v>..
    >>v>v.>.v>
    ..>....v..
    .>.>v>v..v
    ..v..v>vv>
    v.v..>>v..
    .v>....v..

    After 4 steps:
    v>..v.>>..
    v.v.>.>.v.
    >vv.>>.v>v
    >>.>..v>.>
    ..v>v...v.
    ..>>.>vv..
    >.v.vv>v.v
    .....>>vv.
    vvv>...v..

    After 5 steps:
    vv>...>v>.
    v.v.v>.>v.
    >.v.>.>.>v
    >v>.>..v>>
    ..v>v.v...
    ..>.>>vvv.
    .>...v>v..
    ..v.v>>v.v
    v.v.>...v.

    ...

    After 10 steps:
    ..>..>>vv.
    v.....>>.v
    ..v.v>>>v>
    v>.>v.>>>.
    ..v>v.vv.v
    .v.>>>.v..
    v.v..>v>..
    ..v...>v.>
    .vv..v>vv.

    ...

    After 20 steps:
    v>.....>>.
    >vv>.....v
    .>v>v.vv>>
    v>>>v.>v.>
    ....vv>v..
    .v.>>>vvv.
    ..v..>>vv.
    v.v...>>.v
    ..v.....v>

    ...

    After 30 steps:
    .vv.v..>>>
    v>...v...>
    >.v>.>vv.>
    >v>.>.>v.>
    .>..v.vv..
    ..v>..>>v.
    ....v>..>v
    v.v...>vv>
    v.v...>vvv

    ...

    After 40 steps:
    >>v>v..v..
    ..>>v..vv.
    ..>>>v.>.v
    ..>>>>vvv>
    v.....>...
    v.v...>v>>
    >vv.....v>
    .>v...v.>v
    vvv.v..v.>

    ...

    After 50 steps:
    ..>>v>vv.v
    ..v.>>vv..
    v.>>v>>v..
    ..>>>>>vv.
    vvv....>vv
    ..v....>>>
    v>.......>
    .vv>....v>
    .>v.vv.v..

    ...

    After 55 steps:
    ..>>v>vv..
    ..v.>>vv..
    ..>>v>>vv.
    ..>>>>>vv.
    v......>vv
    v>v....>>v
    vvv...>..>
    >vv.....>.
    .>v.vv.v..

    After 56 steps:
    ..>>v>vv..
    ..v.>>vv..
    ..>>v>>vv.
    ..>>>>>vv.
    v......>vv
    v>v....>>v
    vvv....>.>
    >vv......>
    .>v.vv.v..

    After 57 steps:
    ..>>v>vv..
    ..v.>>vv..
    ..>>v>>vv.
    ..>>>>>vv.
    v......>vv
    v>v....>>v
    vvv.....>>
    >vv......>
    .>v.vv.v..

    After 58 steps:
    ..>>v>vv..
    ..v.>>vv..
    ..>>v>>vv.
    ..>>>>>vv.
    v......>vv
    v>v....>>v
    vvv.....>>
    >vv......>
    .>v.vv.v..
    In this example, the sea cucumbers stop moving after 58 steps.

    Find somewhere safe to land your submarine. What is the first step on which no sea cucumbers move?

    --- Part Two ---
    Suddenly, the experimental antenna control console lights up:

    Sleigh keys detected!
    According to the console, the keys are directly under the submarine. You landed right on them! Using a robotic arm on the submarine, you move the sleigh keys into the airlock.

    Now, you just need to get them to Santa in time to save Christmas! You check your clock - it is Christmas. There's no way you can get them back to the surface in time.

    Just as you start to lose hope, you notice a button on the sleigh keys: remote start. You can start the sleigh from the bottom of the ocean! You just need some way to boost the signal from the keys so it actually reaches the sleigh. Good thing the submarine has that experimental antenna! You'll definitely need 50 stars to boost it that far, though.

    The experimental antenna control console lights up again:

    Energy source detected.
    Integrating energy source from device "sleigh keys"...done.
    Installing device drivers...done.
    Recalibrating experimental antenna...done.
    Boost strength due to matching signal phase: 1 star
    Only 49 stars to go.
*/

use crate::common::{Point2, Range2, TileChar, TileMap};

#[derive(Clone, Debug, PartialEq)]
enum Cuke {
    East,
    South,
    None,
}

impl TileChar for Cuke {
    fn to_char(&self) -> char {
        match self {
            Self::East => '>',
            Self::South => 'v',
            Self::None => '.',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '>' => Self::East,
            'v' => Self::South,
            '.' => Self::None,
            _ => panic!("Invalid char: {}", c),
        })
    }

    fn all_chars() -> Vec<char> {
        vec!['>', 'v']
    }
}

#[derive(Clone)]
pub struct Floor {
    map: TileMap<Cuke>,
    range: Range2,
}

impl Floor {
    fn from_string(input: &str) -> Self {
        let map = TileMap::<Cuke>::from_string(input);
        let range = map.get_range().unwrap();
        Self { map, range }
    }

    fn step(&mut self) -> bool {
        let mut from_to: Vec<(Point2, Point2)> = Vec::new();

        // First east
        for (p, cuke) in self.map.iter() {
            if cuke == &Cuke::East {
                let mut adj = p + (1, 0);
                if adj.x > self.range.x.1 {
                    adj.x = self.range.x.0;
                }

                if matches!(self.map.get(&adj), Some(Cuke::East | Cuke::South)) == false {
                    from_to.push((*p, adj));
                }
            }
        }

        let east_count = from_to.len();
        for (from, to) in &from_to {
            self.map.remove(from);
            self.map.insert(*to, Cuke::East);
        }

        // Then south
        from_to.clear();
        for (p, cuke) in self.map.iter() {
            if cuke == &Cuke::South {
                let mut adj = p + (0, 1);
                if adj.y > self.range.y.1 {
                    adj.y = self.range.y.0;
                }

                if matches!(self.map.get(&adj), Some(Cuke::East | Cuke::South)) == false {
                    from_to.push((*p, adj));
                }
            }
        }

        let south_count = from_to.len();
        for (from, to) in from_to {
            self.map.remove(&from);
            self.map.insert(to, Cuke::South);
        }

        east_count == 0 && south_count == 0
    }

    fn find_no_movement(&mut self) -> u32 {
        let mut n = 1; // Start at 1 since we don't increment after the final step
        while self.step() == false {
            n += 1;
        }
        n
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Floor {
    Floor::from_string(input)
}

#[aoc(day25, part1)]
pub fn part1(input: &Floor) -> u32 {
    let mut floor = input.clone();
    let answer = floor.find_no_movement();
    assert_eq!(answer, 482);
    answer
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "...>>>>>...";

    static EXAMPLE_INPUT2: &str = "\
..........
.>v....v..
.......>..
..........";

    static EXAMPLE_INPUT3: &str = "\
...>...
.......
......>
v.....>
......>
.......
..vvv..";

    static EXAMPLE_INPUT4: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_step() {
        let mut floor = input_generator(EXAMPLE_INPUT1);
        floor.step();
        assert_eq!(floor.map.to_string().trim(), "...>>>>.>..");
        floor.step();
        assert_eq!(floor.map.to_string().trim(), "...>>>.>.>.");

        let mut floor = input_generator(EXAMPLE_INPUT2);
        floor.step();
        assert_eq!(
            floor.map.to_string().trim(),
            "\
..........
.>........
..v....v>.
.........."
        );

        let mut floor = input_generator(EXAMPLE_INPUT3);
        floor.step();
        assert_eq!(
            floor.map.to_string().trim(),
            "\
..vv>..
.......
>......
v.....>
>......
.......
....v.."
        );
        floor.step();
        assert_eq!(
            floor.map.to_string().trim(),
            "\
....v>.
..vv...
.>.....
......>
v>.....
.......
......."
        );
        floor.step();
        assert_eq!(
            floor.map.to_string().trim(),
            "\
......>
..v.v..
..>v...
>......
..>....
v......
......."
        );
        floor.step();
        assert_eq!(
            floor.map.to_string().trim(),
            "\
>......
..v....
..>.v..
.>.v...
...>...
.......
v......"
        );

        let mut floor = input_generator(EXAMPLE_INPUT4);
        for _ in 0..58 {
            floor.step();
        }
        assert_eq!(
            floor.map.to_string().trim(),
            "\
..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."
        );
    }

    #[test]
    fn test_find_no_movement() {
        let mut floor = input_generator(EXAMPLE_INPUT4);
        let steps = floor.find_no_movement();
        assert_eq!(steps, 58);
        assert_eq!(
            floor.map.to_string().trim(),
            "\
..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."
        );
    }
}
