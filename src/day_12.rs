/*
    --- Day 12: Passage Pathing ---
    With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.

    Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:

    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.

    So, the above cave system looks roughly like this:

        start
        /   \
    c--A-----b--d
        \   /
        end
    Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.

    Given these rules, there are 10 paths through this example cave system:

    start,A,b,A,c,A,end
    start,A,b,A,end
    start,A,b,end
    start,A,c,A,b,A,end
    start,A,c,A,b,end
    start,A,c,A,end
    start,A,end
    start,b,A,c,A,end
    start,b,A,end
    start,b,end
    (Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)

    Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.

    Here is a slightly larger example:

    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    The 19 paths through it are as follows:

    start,HN,dc,HN,end
    start,HN,dc,HN,kj,HN,end
    start,HN,dc,end
    start,HN,dc,kj,HN,end
    start,HN,end
    start,HN,kj,HN,dc,HN,end
    start,HN,kj,HN,dc,end
    start,HN,kj,HN,end
    start,HN,kj,dc,HN,end
    start,HN,kj,dc,end
    start,dc,HN,end
    start,dc,HN,kj,HN,end
    start,dc,end
    start,dc,kj,HN,end
    start,kj,HN,dc,HN,end
    start,kj,HN,dc,end
    start,kj,HN,end
    start,kj,dc,HN,end
    start,kj,dc,end
    Finally, this even larger example has 226 paths through it:

    fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    How many paths through this cave system are there that visit small caves at most once?

    --- Part Two ---
    After reviewing the available paths, you realize you might have time to visit a single small cave twice. Specifically, big caves can be visited any number of times, a single small cave can be visited at most twice, and the remaining small caves can be visited at most once. However, the caves named start and end can only be visited exactly once each: once you leave the start cave, you may not return to it, and once you reach the end cave, the path must end immediately.

    Now, the 36 possible paths through the first example above are:

    start,A,b,A,b,A,c,A,end
    start,A,b,A,b,A,end
    start,A,b,A,b,end
    start,A,b,A,c,A,b,A,end
    start,A,b,A,c,A,b,end
    start,A,b,A,c,A,c,A,end
    start,A,b,A,c,A,end
    start,A,b,A,end
    start,A,b,d,b,A,c,A,end
    start,A,b,d,b,A,end
    start,A,b,d,b,end
    start,A,b,end
    start,A,c,A,b,A,b,A,end
    start,A,c,A,b,A,b,end
    start,A,c,A,b,A,c,A,end
    start,A,c,A,b,A,end
    start,A,c,A,b,d,b,A,end
    start,A,c,A,b,d,b,end
    start,A,c,A,b,end
    start,A,c,A,c,A,b,A,end
    start,A,c,A,c,A,b,end
    start,A,c,A,c,A,end
    start,A,c,A,end
    start,A,end
    start,b,A,b,A,c,A,end
    start,b,A,b,A,end
    start,b,A,b,end
    start,b,A,c,A,b,A,end
    start,b,A,c,A,b,end
    start,b,A,c,A,c,A,end
    start,b,A,c,A,end
    start,b,A,end
    start,b,d,b,A,c,A,end
    start,b,d,b,A,end
    start,b,d,b,end
    start,b,end
    The slightly larger example above now has 103 paths through it, and the even larger example now has 3509 paths through it.

    Given these new rules, how many paths through this cave system are there?
*/

use crate::common::{to_owned, trim_start, Mode};
use nom::{
    character::complete::{alpha1, char},
    combinator::map,
    multi::many1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Cave {
    Big(String),
    Small(String),
}

impl Cave {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, cave) = map(to_owned(alpha1), |x: String| {
            if x.chars().all(|c| c.is_ascii_uppercase()) {
                Self::Big(x)
            } else if x.chars().all(|c| c.is_ascii_lowercase()) {
                Self::Small(x)
            } else {
                panic!("Invalid cave string found: \"{}\"", x)
            }
        })(input)?;
        Ok((input, cave))
    }
}

pub struct CaveSystem {
    connections: Vec<(Cave, Cave)>,
}

impl CaveSystem {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, connections) = many1(trim_start(separated_pair(
            Cave::parser,
            char('-'),
            Cave::parser,
        )))(input)?;

        Ok((input, Self { connections }))
    }

    fn enumerate_paths(&self, mode: Mode) -> Vec<Vec<&Cave>> {
        // First build the set of 'cave exits' - for each cave, what are all the ways out of it.
        // The 'connections' in the input are just each individual exit.
        let mut cave_exits: HashMap<&Cave, Vec<&Cave>> = HashMap::new();
        for (a, b) in &self.connections {
            // Assume there are no duplicates in the connections list
            let entry = cave_exits.entry(a).or_insert_with(Vec::new);
            entry.push(b);
            let entry = cave_exits.entry(b).or_insert_with(Vec::new);
            entry.push(a);
        }

        let mut finished_paths = Vec::new();
        let start = self
            .connections
            .iter()
            .filter_map(|x| {
                let start = Cave::Small("start".into());
                if x.0 == start {
                    Some(&x.0)
                } else if x.1 == start {
                    Some(&x.1)
                } else {
                    None
                }
            })
            .next()
            .unwrap(); // This looks insane, but since it gets inserted in the returned collection it's necessary to reference an existing item rather than create a new one
        let end = Cave::Small("end".into());
        let mut paths_in_progress = vec![vec![start]];
        let mut paths_revisited = vec![false];
        while let Some(curr_path) = paths_in_progress.pop() {
            let revisited = paths_revisited.pop().unwrap();
            let curr_cave = &curr_path[curr_path.len() - 1];
            let exits = &cave_exits[curr_cave];
            for exit in exits {
                let mut revisiting = false; // If we take this exit, is it a revisit?
                if let Cave::Small(name) = exit {
                    if curr_path.contains(exit) == true {
                        match mode {
                            Mode::M1 => continue, // Can't visit small caves twice
                            Mode::M2 => {
                                // Can revisit only one small cave per path, and not start or end. End is implicit since the path is finished when we hit it.
                                if revisited == true || name.as_str() == "start" {
                                    continue;
                                } else {
                                    revisiting = true;
                                }
                            }
                        }
                    }
                }

                let mut next = curr_path.clone();
                next.push(*exit);
                if exit == &&end {
                    finished_paths.push(next);
                } else {
                    paths_in_progress.push(next);
                    paths_revisited.push(revisited || revisiting);
                }
            }
        }

        finished_paths
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> CaveSystem {
    CaveSystem::parser(input).unwrap().1
}

#[aoc(day12, part1)]
pub fn part1(input: &CaveSystem) -> usize {
    let paths = input.enumerate_paths(Mode::M1);
    let len = paths.len();
    assert_eq!(len, 4304);
    len
}

#[aoc(day12, part2)]
pub fn part2(input: &CaveSystem) -> usize {
    let paths = input.enumerate_paths(Mode::M2);
    let len = paths.len();
    assert_eq!(len, 118242);
    len
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    static EXAMPLE_INPUT2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    static EXAMPLE_INPUT3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    fn path_to_string(path: &[&Cave]) -> String {
        let strs: Vec<String> = path
            .iter()
            .map(|cave| match cave {
                Cave::Big(s) => s.clone(),
                Cave::Small(s) => s.clone(),
            })
            .collect();

        strs.join(",".into())
    }

    #[test]
    fn test_enumerate_paths1() {
        let input = input_generator(EXAMPLE_INPUT1);
        let paths = input.enumerate_paths(Mode::M1);
        assert_eq!(paths.len(), 10);
        let mut path_strs: Vec<String> = paths.iter().map(|path| path_to_string(path)).collect();
        path_strs.sort_unstable();
        assert_eq!(
            path_strs,
            vec![
                "start,A,b,A,c,A,end".to_owned(),
                "start,A,b,A,end".to_owned(),
                "start,A,b,end".to_owned(),
                "start,A,c,A,b,A,end".to_owned(),
                "start,A,c,A,b,end".to_owned(),
                "start,A,c,A,end".to_owned(),
                "start,A,end".to_owned(),
                "start,b,A,c,A,end".to_owned(),
                "start,b,A,end".to_owned(),
                "start,b,end".to_owned(),
            ]
        );

        let input = input_generator(EXAMPLE_INPUT2);
        let paths = input.enumerate_paths(Mode::M1);
        assert_eq!(paths.len(), 19);
        let mut path_strs: Vec<String> = paths.iter().map(|path| path_to_string(path)).collect();
        path_strs.sort_unstable();
        assert_eq!(
            path_strs,
            vec![
                "start,HN,dc,HN,end".to_owned(),
                "start,HN,dc,HN,kj,HN,end".to_owned(),
                "start,HN,dc,end".to_owned(),
                "start,HN,dc,kj,HN,end".to_owned(),
                "start,HN,end".to_owned(),
                "start,HN,kj,HN,dc,HN,end".to_owned(),
                "start,HN,kj,HN,dc,end".to_owned(),
                "start,HN,kj,HN,end".to_owned(),
                "start,HN,kj,dc,HN,end".to_owned(),
                "start,HN,kj,dc,end".to_owned(),
                "start,dc,HN,end".to_owned(),
                "start,dc,HN,kj,HN,end".to_owned(),
                "start,dc,end".to_owned(),
                "start,dc,kj,HN,end".to_owned(),
                "start,kj,HN,dc,HN,end".to_owned(),
                "start,kj,HN,dc,end".to_owned(),
                "start,kj,HN,end".to_owned(),
                "start,kj,dc,HN,end".to_owned(),
                "start,kj,dc,end".to_owned(),
            ]
        );

        let input = input_generator(EXAMPLE_INPUT3);
        let paths = input.enumerate_paths(Mode::M1);
        assert_eq!(paths.len(), 226);
    }

    #[test]
    fn test_enumerate_paths2() {
        let input = input_generator(EXAMPLE_INPUT1);
        let paths = input.enumerate_paths(Mode::M2);
        assert_eq!(paths.len(), 36);
        let mut path_strs: Vec<String> = paths.iter().map(|path| path_to_string(path)).collect();
        path_strs.sort_unstable();
        assert_eq!(
            path_strs,
            vec![
                "start,A,b,A,b,A,c,A,end".to_owned(),
                "start,A,b,A,b,A,end".to_owned(),
                "start,A,b,A,b,end".to_owned(),
                "start,A,b,A,c,A,b,A,end".to_owned(),
                "start,A,b,A,c,A,b,end".to_owned(),
                "start,A,b,A,c,A,c,A,end".to_owned(),
                "start,A,b,A,c,A,end".to_owned(),
                "start,A,b,A,end".to_owned(),
                "start,A,b,d,b,A,c,A,end".to_owned(),
                "start,A,b,d,b,A,end".to_owned(),
                "start,A,b,d,b,end".to_owned(),
                "start,A,b,end".to_owned(),
                "start,A,c,A,b,A,b,A,end".to_owned(),
                "start,A,c,A,b,A,b,end".to_owned(),
                "start,A,c,A,b,A,c,A,end".to_owned(),
                "start,A,c,A,b,A,end".to_owned(),
                "start,A,c,A,b,d,b,A,end".to_owned(),
                "start,A,c,A,b,d,b,end".to_owned(),
                "start,A,c,A,b,end".to_owned(),
                "start,A,c,A,c,A,b,A,end".to_owned(),
                "start,A,c,A,c,A,b,end".to_owned(),
                "start,A,c,A,c,A,end".to_owned(),
                "start,A,c,A,end".to_owned(),
                "start,A,end".to_owned(),
                "start,b,A,b,A,c,A,end".to_owned(),
                "start,b,A,b,A,end".to_owned(),
                "start,b,A,b,end".to_owned(),
                "start,b,A,c,A,b,A,end".to_owned(),
                "start,b,A,c,A,b,end".to_owned(),
                "start,b,A,c,A,c,A,end".to_owned(),
                "start,b,A,c,A,end".to_owned(),
                "start,b,A,end".to_owned(),
                "start,b,d,b,A,c,A,end".to_owned(),
                "start,b,d,b,A,end".to_owned(),
                "start,b,d,b,end".to_owned(),
                "start,b,end".to_owned(),
            ]
        );

        let input = input_generator(EXAMPLE_INPUT2);
        let paths = input.enumerate_paths(Mode::M2);
        assert_eq!(paths.len(), 103);

        let input = input_generator(EXAMPLE_INPUT3);
        let paths = input.enumerate_paths(Mode::M2);
        assert_eq!(paths.len(), 3509);
    }
}
