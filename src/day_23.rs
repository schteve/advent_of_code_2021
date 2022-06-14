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

use crate::common::{Mode, Point2};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum AmphiKind {
    A,
    B,
    C,
    D,
}

impl AmphiKind {
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
            _ => unreachable!(),
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
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Amphipod {
    pos: Point2,
    kind: AmphiKind,
}

#[derive(Debug, PartialEq)]
enum Space {
    Room(AmphiKind),
    Hallway,
    HallwayCantStop,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Node(u32, Vec<Amphipod>);

pub struct Burrow {
    map: HashMap<Point2, Space>,
    start: Vec<Amphipod>,
    rooms: HashMap<AmphiKind, Vec<Point2>>,
}

impl Burrow {
    fn from_string(input: &str, mode: Mode) -> Self {
        let mut input_string = String::new();
        for (i, line) in input.lines().enumerate() {
            if mode == Mode::M2 && i == 3 {
                input_string.push_str("  #D#C#B#A#\n");
                input_string.push_str("  #D#B#A#C#\n");
            }
            input_string.push_str(line);
            input_string.push('\n');
        }

        let mut start = Vec::new();
        let mut map = HashMap::new();
        for (y, line) in input_string.lines().enumerate() {
            let mut room = 0;
            for (x, c) in line.chars().enumerate() {
                let p = Point2 {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' | ' ' => (),
                    '.' => {
                        if [3, 5, 7, 9].contains(&p.x) == true {
                            map.insert(p, Space::HallwayCantStop);
                        } else {
                            map.insert(p, Space::Hallway);
                        }
                    }
                    'A'..='D' => {
                        let amphipod = Amphipod {
                            pos: p,
                            kind: AmphiKind::from_char(c),
                        };
                        start.push(amphipod);

                        let room_type = match room {
                            0 => AmphiKind::A,
                            1 => AmphiKind::B,
                            2 => AmphiKind::C,
                            3 => AmphiKind::D,
                            _ => panic!("Invalid room {}", room),
                        };
                        map.insert(p, Space::Room(room_type));
                        room += 1;
                    }
                    _ => panic!("Unknown character: {}", c),
                }
            }
        }

        let mut rooms = HashMap::new();
        for (p, space) in &map {
            if let Space::Room(a) = space {
                let entry = rooms.entry(*a).or_insert_with(Vec::new);
                entry.push(*p);
            }
        }
        for room in rooms.values_mut() {
            room.sort_unstable();
        }

        Self { map, start, rooms }
    }

    fn organize(&self) -> u32 {
        let mut end_state: Vec<Amphipod> = self
            .map
            .iter()
            .filter_map(|(p, space)| match space {
                Space::Room(r) => Some(Amphipod { pos: *p, kind: *r }),
                Space::Hallway => None,
                Space::HallwayCantStop => None,
            })
            .collect();
        end_state.sort_unstable_by_key(|a| a.pos);

        let mut best_states: HashMap<Vec<Amphipod>, u32> = HashMap::new();
        best_states.insert(self.start.clone(), 0);

        let mut states: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        states.push(Reverse(Node(0, self.start.clone())));
        while let Some(Reverse(Node(curr_cost, curr_state))) = states.pop() {
            //println!("States: {}", states.len());
            //println!("Try cost {}:", curr_cost);
            //println!("{}", BurrowDisplay(self, &curr_state));
            if curr_state == end_state {
                continue;
            }
            if let Some(best_cost) = best_states.get(&end_state) {
                if curr_cost >= *best_cost {
                    continue;
                }
            }

            // Get all possible moves for all amphipods
            let mut all_moves: Vec<(usize, Point2, u32)> = Vec::new();
            for (i, amphipod) in curr_state.iter().enumerate() {
                //println!("    Moves for [{}] @ {:?}:", i, amphipod);
                for (p, cost) in self.moves(&curr_state, amphipod) {
                    all_moves.push((i, p, cost));
                }
            }

            // Find the subset of moves which lead to a room
            let mut room_moves: Vec<(usize, Point2, u32)> = Vec::new();
            for (i, p, cost) in all_moves.iter().copied() {
                if matches!(self.map.get(&p).unwrap(), Space::Room(_)) {
                    room_moves.push((i, p, cost));
                }
            }

            // If there are moves that go to rooms, only consider those options. This improves the efficiency of the search
            // by discarding many intermediate states. If there aren't any moves that go to rooms, consider all options.
            let moves_iter = if room_moves.is_empty() == false {
                room_moves.into_iter()
            } else {
                all_moves.into_iter()
            };

            for (i, p, cost) in moves_iter {
                //println!("        {} cost {}", m.0, m.1);
                let mut next_state = curr_state.clone();
                next_state[i].pos = p;
                next_state.sort_unstable_by_key(|a| a.pos);
                let next_cost = curr_cost + cost;
                if let Some(best_cost) = best_states.get(&next_state) {
                    if next_cost >= *best_cost {
                        // Next isn't better so don't bother with it
                    } else {
                        //println!("        Push next cost {}: {:?}", next_cost, next_state);
                        states.push(Reverse(Node(next_cost, next_state.clone())));
                        best_states.insert(next_state, next_cost);
                    }
                } else {
                    //println!("        Push next cost {}: {:?}", next_cost, next_state);
                    states.push(Reverse(Node(next_cost, next_state.clone())));
                    best_states.insert(next_state, next_cost);
                }
            }
        }

        best_states[&end_state]
    }

    fn is_room_ready(&self, curr_state: &[Amphipod], kind: AmphiKind) -> Option<Point2> {
        let mut first_free = None;

        let room = &self.rooms[&kind];
        for p in room.iter().rev() {
            let mut any = false;
            for a in curr_state {
                if a.pos == *p {
                    if a.kind != kind {
                        return None;
                    } else {
                        any = true;
                        break;
                    }
                }
            }
            if any == false && first_free.is_none() {
                first_free = Some(*p);
            }
        }

        first_free
    }

    fn moves(&self, curr_state: &[Amphipod], amphipod: &Amphipod) -> Vec<(Point2, u32)> {
        /*
            Find all valid moves for a given amphipod.
            Return a list of the points along with their energy costs.

            Valid moves are:
            1. Go from a room that isn't ready to the hallway or to a ready room
            2. Go from the hallway to a ready room
        */
        let mut moves = Vec::new();

        let reachable = self.find_reachable(curr_state, &amphipod.pos);
        //println!("        Reachable: {:?}", reachable);
        for (p, steps) in reachable {
            // They never move from hallway to hallway due to the 3rd rule. Check (from, to) pairs.
            match (
                self.map.get(&amphipod.pos).unwrap(),
                self.map.get(&p).unwrap(),
            ) {
                (Space::Room(from), Space::Room(to)) => {
                    // TODO: try removing this as a possibility. Things should still work as Room -> Hallway -> Room
                    // but this lets us calculate cost directly as the manhattan distance, which should also make
                    // find_reachable() simpler. Not clear if this is a timesaver as it does increase the search space.
                    /*
                        This is a valid move if ALL of these are true:
                        1. The 'from' room is not a matching room
                        2. The 'to' room is matching
                        3. The 'to' room is ready
                        4. The 'to' point is the next ready space
                    */
                    if from != &amphipod.kind && to == &amphipod.kind {
                        if let Some(next_ready) = self.is_room_ready(curr_state, amphipod.kind) {
                            if next_ready == p {
                                let cost = steps * amphipod.kind.cost();
                                moves.push((p, cost));
                            }
                        }
                    }
                }
                (Space::Hallway, Space::Room(to)) => {
                    /*
                        This is a valid move if ALL of these are true:
                        1. The 'to' room is matching and ready
                        2. The 'to' point is the next ready space
                    */
                    if to == &amphipod.kind {
                        if let Some(next_ready) = self.is_room_ready(curr_state, amphipod.kind) {
                            if next_ready == p {
                                let cost = steps * amphipod.kind.cost();
                                moves.push((p, cost));
                            }
                        }
                    }
                }
                (Space::Room(from), Space::Hallway) => {
                    /*
                        This is a valid move if ANY of these are true:
                        1. The 'from' room is not matching
                        2. The 'from' room is matching but not ready
                    */
                    #[allow(clippy::if_same_then_else)] // False positive
                    if from != &amphipod.kind {
                        let cost = steps * amphipod.kind.cost();
                        moves.push((p, cost));
                    } else if self.is_room_ready(curr_state, amphipod.kind).is_none() == true {
                        let cost = steps * amphipod.kind.cost();
                        moves.push((p, cost));
                    } else {
                        // Do nothing
                    }
                }
                _ => (), // No other options are valid
            }
        }

        moves
    }

    fn find_reachable(&self, curr_state: &[Amphipod], from: &Point2) -> Vec<(Point2, u32)> {
        // Find all reachable positions.
        // A space is reachable if it is not blocked by an amphipod.
        // Do not consider whether the space is valid (i.e. if it's a room of the right type).
        let mut reachable = Vec::new();

        let mut paths: Vec<(Point2, u32)> = vec![(*from, 0)];
        let mut visited = vec![*from];
        while paths.is_empty() == false {
            let (curr_p, curr_step) = paths.pop().unwrap();
            for next_p in curr_p.orthogonals() {
                if visited.contains(&next_p) == false && self.map.get(&next_p).is_some() == true {
                    let any_blocker = curr_state.iter().any(|a| a.pos == next_p);
                    if any_blocker == false {
                        paths.push((next_p, curr_step + 1));
                        reachable.push((next_p, curr_step + 1));
                        visited.push(next_p);
                    }
                }
            }
            paths.sort_unstable_by_key(|x| Reverse(x.1));
            paths.dedup();
        }

        reachable
    }
}

struct BurrowDisplay<'a, 'b>(&'a Burrow, &'b Vec<Amphipod>);

impl<'a, 'b> std::fmt::Display for BurrowDisplay<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = Point2::get_range(self.0.map.keys()).unwrap();
        for y in range.y.0..=range.y.1 {
            for x in range.x.0..=range.x.1 {
                let p = Point2 { x, y };
                if let Some(a) = self.1.iter().find(|a| a.pos == p) {
                    write!(f, "{}", a.kind.to_char())?;
                } else if self.0.map.get(&p).is_some() == true {
                    write!(f, ".")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u32 {
    let burrow = Burrow::from_string(input, Mode::M1);
    let cost = burrow.organize();
    assert_eq!(cost, 12240);
    cost
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> u32 {
    let burrow = Burrow::from_string(input, Mode::M2);
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
        let burrow = Burrow::from_string(EXAMPLE_INPUT, Mode::M1);
        let cost = burrow.organize();
        assert_eq!(cost, 12521);
    }

    #[test]
    fn test_organize2() {
        let burrow = Burrow::from_string(EXAMPLE_INPUT, Mode::M2);
        let cost = burrow.organize();
        assert_eq!(cost, 44169);
    }

    #[test]
    fn test_is_room_ready() {
        let burrow = Burrow::from_string(EXAMPLE_INPUT, Mode::M1);
        assert_eq!(burrow.is_room_ready(&burrow.start, AmphiKind::A), None);
        assert_eq!(burrow.is_room_ready(&burrow.start, AmphiKind::B), None);
        assert_eq!(burrow.is_room_ready(&burrow.start, AmphiKind::C), None);
        assert_eq!(burrow.is_room_ready(&burrow.start, AmphiKind::D), None);

        #[rustfmt::skip]
        let curr_state = vec![
            Amphipod { pos: Point2 { x: 3, y: 2 }, kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 3, y: 3 },  kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 5, y: 2 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 5, y: 3 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 7, y: 2 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 7, y: 3 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 9, y: 2 }, kind: AmphiKind::D },
            Amphipod { pos: Point2 { x: 9, y: 3 }, kind: AmphiKind::D },
        ];
        assert_eq!(burrow.is_room_ready(&curr_state, AmphiKind::A), None);
        assert_eq!(burrow.is_room_ready(&curr_state, AmphiKind::B), None);
        assert_eq!(burrow.is_room_ready(&curr_state, AmphiKind::C), None);
        assert_eq!(burrow.is_room_ready(&curr_state, AmphiKind::D), None);

        #[rustfmt::skip]
        let curr_state = vec![
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::D },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::D },
        ];
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::A),
            Some(Point2 { x: 3, y: 3 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::B),
            Some(Point2 { x: 5, y: 3 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::C),
            Some(Point2 { x: 7, y: 3 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::D),
            Some(Point2 { x: 9, y: 3 })
        );

        #[rustfmt::skip]
        let curr_state = vec![
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 3, y: 3 }, kind: AmphiKind::A },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 5, y: 3 }, kind: AmphiKind::B },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 7, y: 3 }, kind: AmphiKind::C },
            Amphipod { pos: Point2 { x: 0, y: 0 }, kind: AmphiKind::D },
            Amphipod { pos: Point2 { x: 9, y: 3 }, kind: AmphiKind::D },
        ];
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::A),
            Some(Point2 { x: 3, y: 2 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::B),
            Some(Point2 { x: 5, y: 2 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::C),
            Some(Point2 { x: 7, y: 2 })
        );
        assert_eq!(
            burrow.is_room_ready(&curr_state, AmphiKind::D),
            Some(Point2 { x: 9, y: 2 })
        );
    }
}
