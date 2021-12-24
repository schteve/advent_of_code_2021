/*

*/

use crate::common::{Mode, Point2};
use std::{cmp::Reverse, collections::HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

        let mut states: Vec<(Vec<Amphipod>, u32)> = vec![(self.start.clone(), 0)]; // TODO: better representation for state other than a vec?
        while states.is_empty() == false {
            //println!("States: {}", states.len());
            let (curr_state, curr_cost) = states.pop().unwrap(); // TODO: can use while let pop?

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
            for (i, amphipod) in curr_state.iter().enumerate() {
                //println!("    Moves for [{}] @ {:?}:", i, amphipod);
                for m in self.moves(&curr_state, amphipod) {
                    //println!("        {} cost {}", m.0, m.1);
                    let mut next_state = curr_state.clone();
                    next_state[i].pos = m.0;
                    next_state.sort_unstable_by_key(|a| a.pos);
                    let next_cost = curr_cost + m.1;
                    if let Some(best_cost) = best_states.get(&next_state) {
                        if next_cost >= *best_cost {
                            // Next isn't better so don't bother with it
                        } else {
                            //println!("        Push next cost {}: {:?}", next_cost, next_state);
                            states.push((next_state.clone(), next_cost));
                            best_states.insert(next_state, next_cost);
                        }
                    } else {
                        //println!("        Push next cost {}: {:?}", next_cost, next_state);
                        states.push((next_state.clone(), next_cost));
                        best_states.insert(next_state, next_cost);
                    }
                }
            }
            states.sort_unstable_by_key(|x| Reverse(x.1));
            states.dedup();
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
