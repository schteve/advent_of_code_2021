/*
    --- Day 23: Amphipod ---
    A group of amphipods notice your fancy submarine and flag you down. "With such an impressive shell," one amphipod says, "surely you can help us with a question that has stumped our best scientists."

    They go on to explain that a group of timid, stubborn amphipods live in a nearby burrow. Four types of amphipods live there: Amber (A), Bronze (B), Copper (C), and Desert (D). They live in a burrow that consists of a hallway and four side rooms. The side rooms are initially full of amphipods, and the hallway is initially empty.

    They give you a diagram of the situation (your puzzle input), including locations of each amphipod (A, B, C, or D, each of which is occupying an otherwise open space), walls (#), and open space (.).

    For example:

    #############
    #...........#
    ###B#C#B#D###
    #A#D#C#A#
    #########
    The amphipods would like a method to organize every amphipod into side rooms so that each side room contains one type of amphipod and the types are sorted A-D going left to right, like this:

    #############
    #...........#
    ###A#B#C#D###
    #A#B#C#D#
    #########
    Amphipods can move up, down, left, or right so long as they are moving into an unoccupied open space. Each type of amphipod requires a different amount of energy to move one step: Amber amphipods require 1 energy per step, Bronze amphipods require 10 energy, Copper amphipods require 100, and Desert ones require 1000. The amphipods would like you to find a way to organize the amphipods that requires the least total energy.

    However, because they are timid and stubborn, the amphipods have some extra rules:

    Amphipods will never stop on the space immediately outside any room. They can move into that space so long as they immediately continue moving. (Specifically, this refers to the four open spaces in the hallway that are directly above an amphipod starting position.)
    Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination. If an amphipod's starting room is not its destination room, it can stay in that room until it leaves the room. (For example, an Amber amphipod will not move from the hallway into the right three rooms, and will only move into the leftmost room if that room is empty or if it only contains other Amber amphipods.)
    Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room. (That is, once any amphipod starts moving, any other amphipods currently in the hallway are locked in place and will not move again until they can move fully into a room.)
    In the above example, the amphipods can be organized using a minimum of 12521 energy. One way to do this is shown below.

    Starting configuration:

    #############
    #...........#
    ###B#C#B#D###
    #A#D#C#A#
    #########
    One Bronze amphipod moves into the hallway, taking 4 steps and using 40 energy:

    #############
    #...B.......#
    ###B#C#.#D###
    #A#D#C#A#
    #########
    The only Copper amphipod not in its side room moves there, taking 4 steps and using 400 energy:

    #############
    #...B.......#
    ###B#.#C#D###
    #A#D#C#A#
    #########
    A Desert amphipod moves out of the way, taking 3 steps and using 3000 energy, and then the Bronze amphipod takes its place, taking 3 steps and using 30 energy:

    #############
    #.....D.....#
    ###B#.#C#D###
    #A#B#C#A#
    #########
    The leftmost Bronze amphipod moves to its room using 40 energy:

    #############
    #.....D.....#
    ###.#B#C#D###
    #A#B#C#A#
    #########
    Both amphipods in the rightmost room move into the hallway, using 2003 energy in total:

    #############
    #.....D.D.A.#
    ###.#B#C#.###
    #A#B#C#.#
    #########
    Both Desert amphipods move into the rightmost room using 7000 energy:

    #############
    #.........A.#
    ###.#B#C#D###
    #A#B#C#D#
    #########
    Finally, the last Amber amphipod moves into its room, using 8 energy:

    #############
    #...........#
    ###A#B#C#D###
    #A#B#C#D#
    #########
    What is the least energy required to organize the amphipods?

    --- Part Two ---
    As you prepare to give the amphipods your solution, you notice that the diagram they handed you was actually folded up. As you unfold it, you discover an extra part of the diagram.

    Between the first and second lines of text that contain amphipod starting positions, insert the following lines:

    #D#C#B#A#
    #D#B#A#C#
    So, the above example now becomes:

    #############
    #...........#
    ###B#C#B#D###
    #D#C#B#A#
    #D#B#A#C#
    #A#D#C#A#
    #########
    The amphipods still want to be organized into rooms similar to before:

    #############
    #...........#
    ###A#B#C#D###
    #A#B#C#D#
    #A#B#C#D#
    #A#B#C#D#
    #########
    In this updated example, the least energy required to organize these amphipods is 44169:

    #############
    #...........#
    ###B#C#B#D###
    #D#C#B#A#
    #D#B#A#C#
    #A#D#C#A#
    #########

    #############
    #..........D#
    ###B#C#B#.###
    #D#C#B#A#
    #D#B#A#C#
    #A#D#C#A#
    #########

    #############
    #A.........D#
    ###B#C#B#.###
    #D#C#B#.#
    #D#B#A#C#
    #A#D#C#A#
    #########

    #############
    #A........BD#
    ###B#C#.#.###
    #D#C#B#.#
    #D#B#A#C#
    #A#D#C#A#
    #########

    #############
    #A......B.BD#
    ###B#C#.#.###
    #D#C#.#.#
    #D#B#A#C#
    #A#D#C#A#
    #########

    #############
    #AA.....B.BD#
    ###B#C#.#.###
    #D#C#.#.#
    #D#B#.#C#
    #A#D#C#A#
    #########

    #############
    #AA.....B.BD#
    ###B#.#.#.###
    #D#C#.#.#
    #D#B#C#C#
    #A#D#C#A#
    #########

    #############
    #AA.....B.BD#
    ###B#.#.#.###
    #D#.#C#.#
    #D#B#C#C#
    #A#D#C#A#
    #########

    #############
    #AA...B.B.BD#
    ###B#.#.#.###
    #D#.#C#.#
    #D#.#C#C#
    #A#D#C#A#
    #########

    #############
    #AA.D.B.B.BD#
    ###B#.#.#.###
    #D#.#C#.#
    #D#.#C#C#
    #A#.#C#A#
    #########

    #############
    #AA.D...B.BD#
    ###B#.#.#.###
    #D#.#C#.#
    #D#.#C#C#
    #A#B#C#A#
    #########

    #############
    #AA.D.....BD#
    ###B#.#.#.###
    #D#.#C#.#
    #D#B#C#C#
    #A#B#C#A#
    #########

    #############
    #AA.D......D#
    ###B#.#.#.###
    #D#B#C#.#
    #D#B#C#C#
    #A#B#C#A#
    #########

    #############
    #AA.D......D#
    ###B#.#C#.###
    #D#B#C#.#
    #D#B#C#.#
    #A#B#C#A#
    #########

    #############
    #AA.D.....AD#
    ###B#.#C#.###
    #D#B#C#.#
    #D#B#C#.#
    #A#B#C#.#
    #########

    #############
    #AA.......AD#
    ###B#.#C#.###
    #D#B#C#.#
    #D#B#C#.#
    #A#B#C#D#
    #########

    #############
    #AA.......AD#
    ###.#B#C#.###
    #D#B#C#.#
    #D#B#C#.#
    #A#B#C#D#
    #########

    #############
    #AA.......AD#
    ###.#B#C#.###
    #.#B#C#.#
    #D#B#C#D#
    #A#B#C#D#
    #########

    #############
    #AA.D.....AD#
    ###.#B#C#.###
    #.#B#C#.#
    #.#B#C#D#
    #A#B#C#D#
    #########

    #############
    #A..D.....AD#
    ###.#B#C#.###
    #.#B#C#.#
    #A#B#C#D#
    #A#B#C#D#
    #########

    #############
    #...D.....AD#
    ###.#B#C#.###
    #A#B#C#.#
    #A#B#C#D#
    #A#B#C#D#
    #########

    #############
    #.........AD#
    ###.#B#C#.###
    #A#B#C#D#
    #A#B#C#D#
    #A#B#C#D#
    #########

    #############
    #..........D#
    ###A#B#C#.###
    #A#B#C#D#
    #A#B#C#D#
    #A#B#C#D#
    #########

    #############
    #...........#
    ###A#B#C#D###
    #A#B#C#D#
    #A#B#C#D#
    #A#B#C#D#
    #########
    Using the initial configuration from the full diagram, what is the least energy required to organize the amphipods?
*/

use crate::common::Point2;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(&self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            x => panic!("Invalid amphipod char: {}", x),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
        }
    }

    fn room_idx(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }

    const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];
    fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State<const N: usize>(u32, Burrow<N>);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Burrow<const N: usize> {
    hallway: [Option<Amphipod>; 7],
    rooms: [[Option<Amphipod>; N]; 4],
}

impl<const N: usize> Burrow<N> {
    const END_STATE: Self = Self {
        hallway: [None; 7],
        rooms: [
            [Some(Amphipod::A); N],
            [Some(Amphipod::B); N],
            [Some(Amphipod::C); N],
            [Some(Amphipod::D); N],
        ],
    };

    fn from_string(input: &str) -> Self {
        let mut input_string = String::new();
        for (i, line) in input.lines().enumerate() {
            if N == 4 && i == 3 {
                input_string.push_str("  #D#C#B#A#\n");
                input_string.push_str("  #D#B#A#C#\n");
            }
            input_string.push_str(line);
            input_string.push('\n');
        }

        let mut burrow = Self::new();
        for (y, line) in input_string.lines().enumerate() {
            let mut room = 0;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' | ' ' => (),
                    '.' => {
                        if y >= 2 {
                            room += 1;
                        }
                    }
                    'A'..='D' => {
                        let amph = Some(Amphipod::from_char(c));
                        if y == 1 {
                            let idx = Self::x_to_hall_idx(x as i32);
                            burrow.hallway[idx] = amph;
                        } else if y >= 2 {
                            burrow.rooms[room][y - 2] = amph;
                            room += 1;
                        }
                    }
                    _ => panic!("Unknown character: {}", c),
                }
            }
        }
        burrow
    }

    fn new() -> Self {
        Self {
            hallway: [None; 7],
            rooms: [[None; N]; 4],
        }
    }

    fn hall_idx_to_p(idx: usize) -> Point2 {
        // Origin is at hallway idx 0
        match idx {
            0 => (0, 0),
            1 => (1, 0),
            2 => (3, 0),
            3 => (5, 0),
            4 => (7, 0),
            5 => (9, 0),
            6 => (10, 0),
            x => panic!("Invalid hallway idx: {}", x),
        }
        .into()
    }

    fn x_to_hall_idx(x: i32) -> usize {
        // Some x coordinates (the rooms) fall between hallways indexes; we choose the one to the right
        // because it makes other calculations nice.
        match x {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 2,
            4 => 3,
            5 => 3,
            6 => 4,
            7 => 4,
            8 => 5,
            9 => 5,
            10 => 6,
            _ => panic!("Invalid x coordinate: {}", x),
        }
    }

    fn room_idx_to_p(amph: Amphipod, idx: usize) -> Point2 {
        let x = match amph {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        };
        Point2 {
            x,
            y: (idx + 1) as i32,
        }
    }

    fn rooms_to_hallway_moves(&self) -> Vec<State<N>> {
        let mut states = Vec::new();
        for room in Amphipod::iter() {
            if self.is_room_ready(room) == false {
                // Only move out of non-ready rooms (if they're ready, we should be moving into them)
                if let Some(room_idx) = self.room_output_idx(room) {
                    // Found an amphipod in a room that it doesn't belong in
                    let amph = self.rooms[room.room_idx()][room_idx].unwrap();
                    for hall_idx in 0..self.hallway.len() {
                        // Is the hallway space ready? Can it move to this space?
                        if self.hallway[hall_idx].is_none() == true
                            && self.is_path_open(hall_idx, room) == true
                        {
                            // Yes and yes. Move it there.
                            let mut next = *self;

                            // Remove from the room
                            next.rooms[room.room_idx()][room_idx] = None;

                            // Add to the hallway
                            next.hallway[hall_idx] = Some(amph);

                            // Calculate the cost
                            let hall_p = Self::hall_idx_to_p(hall_idx);
                            let room_p = Self::room_idx_to_p(room, room_idx);
                            let cost = Point2::manhattan(hall_p, room_p) * amph.cost();
                            states.push(State(cost, next));
                        }
                    }
                }
            }
        }
        states
    }

    fn hallway_to_rooms_moves(&self) -> Vec<State<N>> {
        let mut states = Vec::new();
        for (hall_idx, h) in self.hallway.into_iter().enumerate() {
            if let Some(amph) = h {
                // Found an amphipod in the hall. Is its room ready for it? Can it move to its room?
                if self.is_room_ready(amph) == true && self.is_path_open(hall_idx, amph) == true {
                    // Yes and yes. Move it there.
                    let mut next = *self;

                    // Remove from the hallway
                    next.hallway[hall_idx] = None;

                    // Add to the room
                    let room_idx = next.room_input_idx(amph);
                    next.rooms[amph.room_idx()][room_idx] = Some(amph);

                    // Calculate the cost
                    let hall_p = Self::hall_idx_to_p(hall_idx);
                    let room_p = Self::room_idx_to_p(amph, room_idx);
                    let cost = Point2::manhattan(hall_p, room_p) * amph.cost();
                    states.push(State(cost, next));
                }
            }
        }
        states
    }

    fn is_room_ready(&self, amph: Amphipod) -> bool {
        self.rooms[amph.room_idx()].iter().all(|r| match r {
            None => true,
            &Some(x) if x == amph => true,
            _ => false,
        })
    }

    fn is_path_open(&self, hall_idx: usize, amph: Amphipod) -> bool {
        let room_p = Self::room_idx_to_p(amph, 0);
        let room_idx = Self::x_to_hall_idx(room_p.x);
        let range = if hall_idx < room_idx {
            &self.hallway[hall_idx + 1..room_idx]
        } else {
            &self.hallway[room_idx..hall_idx]
        };

        range.iter().all(Option::is_none)
    }

    fn room_input_idx(&self, amph: Amphipod) -> usize {
        // Assumes this room is ready! Do your due diligence.
        let mut next_available = None;
        for (i, a) in self.rooms[amph.room_idx()].iter().enumerate() {
            if a.is_none() == true {
                next_available = Some(i);
            }
        }
        next_available.expect("No space in the room!")
    }

    fn room_output_idx(&self, amph: Amphipod) -> Option<usize> {
        for (i, a) in self.rooms[amph.room_idx()].iter().enumerate() {
            if a.is_none() == false {
                return Some(i);
            }
        }
        None
    }

    fn organize(&self) -> u32 {
        let mut best_states: HashMap<Burrow<N>, u32> = HashMap::new();
        best_states.insert(*self, 0);

        let mut state_queue: BinaryHeap<Reverse<State<N>>> = BinaryHeap::new();
        state_queue.push(Reverse(State(0, *self)));
        while let Some(Reverse(State(curr_cost, curr_state))) = state_queue.pop() {
            //println!("{}", curr_state);
            if curr_state == Burrow::END_STATE {
                continue; // This is the end state, no need to search further
            }
            if let Some(best_cost) = best_states.get(&Burrow::END_STATE) {
                if curr_cost >= *best_cost {
                    continue; // This is already worse than another result we've already found so skip it
                }
            }

            /*
                If there are moves that go to rooms, only consider those options. This improves the efficiency of the
                search by discarding many intermediate states, since every amphipod in the hallway needs to move to its
                room eventually and the order in which they move to the room is not important. If there aren't any moves
                that go to rooms, consider moves to the hallway.
            */
            let mut moves = curr_state.hallway_to_rooms_moves();
            if moves.is_empty() == true {
                let hallway_moves = curr_state.rooms_to_hallway_moves();
                moves.extend(hallway_moves);
            }

            for State(cost, next_burrow) in moves {
                let next_cost = curr_cost + cost;
                if let Some(best_cost) = best_states.get(&next_burrow) {
                    if next_cost >= *best_cost {
                        continue; // Next isn't better so don't bother with it
                    }
                }
                state_queue.push(Reverse(State(next_cost, next_burrow)));
                best_states.insert(next_burrow, next_cost);
            }
        }

        best_states[&Burrow::END_STATE]
    }
}

impl<const N: usize> std::fmt::Display for Burrow<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "#############")?;

        write!(f, "#")?;
        for hall_x in 0..11 {
            if [3, 5, 7, 9].contains(&hall_x) == true {
                write!(f, ".")?;
            } else {
                let hall_idx = Self::x_to_hall_idx(hall_x);
                if let Some(amph) = self.hallway[hall_idx] {
                    write!(f, "{}", amph.to_char())?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        writeln!(f, "#")?;

        for row in 0..N {
            if row == 0 {
                write!(f, "##")?;
            } else {
                write!(f, "  ")?;
            }
            write!(f, "#")?;

            for room in 0..self.rooms.len() {
                if let Some(amph) = self.rooms[room][row] {
                    write!(f, "{}", amph.to_char())?;
                } else {
                    write!(f, ".")?;
                }
                write!(f, "#")?;
            }

            if row == 0 {
                write!(f, "#")?;
            }
            writeln!(f, "#")?;
        }

        write!(f, "  #########")?;
        Ok(())
    }
}

impl<const N: usize> std::fmt::Display for State<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cost: {}", self.0)?;
        writeln!(f, "State:{}", self.1)?;
        Ok(())
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u32 {
    let burrow = Burrow::<2>::from_string(input);
    let cost = burrow.organize();
    assert_eq!(cost, 12240);
    cost
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> u32 {
    let burrow = Burrow::<4>::from_string(input);
    let cost = burrow.organize();
    assert_eq!(cost, 44618);
    cost
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_organize1() {
        let burrow = Burrow::<2>::from_string(EXAMPLE_INPUT);
        let cost = burrow.organize();
        assert_eq!(cost, 12521);
    }

    #[test]
    fn test_organize2() {
        let burrow = Burrow::<4>::from_string(EXAMPLE_INPUT);
        let cost = burrow.organize();
        assert_eq!(cost, 44169);
    }

    #[test]
    fn test_is_room_ready() {
        let burrow = Burrow::<2>::from_string(EXAMPLE_INPUT);
        assert_eq!(burrow.is_room_ready(Amphipod::A), false);
        assert_eq!(burrow.is_room_ready(Amphipod::B), false);
        assert_eq!(burrow.is_room_ready(Amphipod::C), false);
        assert_eq!(burrow.is_room_ready(Amphipod::D), false);

        let input = "\
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.is_room_ready(Amphipod::A), true);
        assert_eq!(burrow.is_room_ready(Amphipod::B), true);
        assert_eq!(burrow.is_room_ready(Amphipod::C), true);
        assert_eq!(burrow.is_room_ready(Amphipod::D), true);

        let input = "\
#############
#...........#
###.#.#.#.###
  #A#B#C#D#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.is_room_ready(Amphipod::A), true);
        assert_eq!(burrow.is_room_ready(Amphipod::B), true);
        assert_eq!(burrow.is_room_ready(Amphipod::C), true);
        assert_eq!(burrow.is_room_ready(Amphipod::D), true);

        let input = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.is_room_ready(Amphipod::A), true);
        assert_eq!(burrow.is_room_ready(Amphipod::B), true);
        assert_eq!(burrow.is_room_ready(Amphipod::C), true);
        assert_eq!(burrow.is_room_ready(Amphipod::D), true);

        let input = "\
#############
#...........#
###.#.#.#.###
  #B#A#D#C#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.is_room_ready(Amphipod::A), false);
        assert_eq!(burrow.is_room_ready(Amphipod::B), false);
        assert_eq!(burrow.is_room_ready(Amphipod::C), false);
        assert_eq!(burrow.is_room_ready(Amphipod::D), false);

        let input = "\
#############
#...........#
###A#B#C#D###
  #B#A#D#C#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.is_room_ready(Amphipod::A), false);
        assert_eq!(burrow.is_room_ready(Amphipod::B), false);
        assert_eq!(burrow.is_room_ready(Amphipod::C), false);
        assert_eq!(burrow.is_room_ready(Amphipod::D), false);
    }

    #[test]
    fn test_room_input_idx() {
        let input = "\
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.room_input_idx(Amphipod::A), 1);
        assert_eq!(burrow.room_input_idx(Amphipod::B), 1);
        assert_eq!(burrow.room_input_idx(Amphipod::C), 1);
        assert_eq!(burrow.room_input_idx(Amphipod::D), 1);

        let input = "\
#############
#...........#
###.#.#.#.###
  #A#B#C#D#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.room_input_idx(Amphipod::A), 0);
        assert_eq!(burrow.room_input_idx(Amphipod::B), 0);
        assert_eq!(burrow.room_input_idx(Amphipod::C), 0);
        assert_eq!(burrow.room_input_idx(Amphipod::D), 0);
    }

    #[test]
    fn test_room_output_idx() {
        let input = "\
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.room_output_idx(Amphipod::A), None);
        assert_eq!(burrow.room_output_idx(Amphipod::B), None);
        assert_eq!(burrow.room_output_idx(Amphipod::C), None);
        assert_eq!(burrow.room_output_idx(Amphipod::D), None);

        let input = "\
#############
#...........#
###.#.#.#.###
  #A#B#C#D#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.room_output_idx(Amphipod::A), Some(1));
        assert_eq!(burrow.room_output_idx(Amphipod::B), Some(1));
        assert_eq!(burrow.room_output_idx(Amphipod::C), Some(1));
        assert_eq!(burrow.room_output_idx(Amphipod::D), Some(1));

        let input = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        let burrow = Burrow::<2>::from_string(input);
        assert_eq!(burrow.room_output_idx(Amphipod::A), Some(0));
        assert_eq!(burrow.room_output_idx(Amphipod::B), Some(0));
        assert_eq!(burrow.room_output_idx(Amphipod::C), Some(0));
        assert_eq!(burrow.room_output_idx(Amphipod::D), Some(0));
    }
}
