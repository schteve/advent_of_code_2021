use crate::common::Point;
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct TileSet {
    tiles: HashSet<Point>,
    active_char: char,
}

impl TileSet {
    pub fn new() -> Self {
        Self {
            tiles: HashSet::new(),
            active_char: '#',
        }
    }

    pub fn with_active_char(self, active_char: char) -> Self {
        Self {
            active_char,
            ..self
        }
    }

    pub fn with_tiles<'a, I>(self, tiles: I) -> Self
    where
        I: std::iter::IntoIterator<Item = &'a Point>,
    {
        Self {
            tiles: tiles.into_iter().copied().collect(),
            ..self
        }
    }

    pub fn from_string<const ACTIVE_CHAR: char>(input: &str) -> Self {
        Self::parser::<ACTIVE_CHAR>()(input).unwrap().1
    }

    pub fn parser<const ACTIVE_CHAR: char>() -> impl Fn(&str) -> IResult<&str, Self> {
        move |input: &str| {
            let (pixels, input) = if let Some(idx) = input.find("\n\n") {
                input.split_at(idx)
            } else {
                (input, "")
            };

            let mut tiles = HashSet::new();
            for (y, line) in pixels.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == ACTIVE_CHAR {
                        let p = (x as i32, y as i32).into();
                        tiles.insert(p);
                    }
                }
            }
            Ok((input, Self { tiles, active_char: ACTIVE_CHAR }))
        }
    }

    pub fn get_range(&self) -> Option<((i32, i32), (i32, i32))> {
        Point::get_range(&self.tiles)
    }
}

impl std::fmt::Display for TileSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.get_range().unwrap();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if self.tiles.contains(&Point { x, y }) {
                    write!(f, "{}", self.active_char)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for TileSet {
    type Target = HashSet<Point>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl std::ops::DerefMut for TileSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

pub trait TileChar
where
    Self: Sized,
{
    fn to_char(&self) -> char;
    fn from_char(c: char) -> Option<Self>;
    fn all_chars() -> Vec<char>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileMap<T> {
    tiles: HashMap<Point, T>,
}

impl<T: TileChar> TileMap<T> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn with_tiles<'a, I>(self, tiles: I) -> Self
    where
        I: std::iter::IntoIterator<Item = (&'a Point, T)>,
    {
        Self {
            tiles: tiles.into_iter().map(|(p, t)| (*p, t)).collect(),
        }
    }

    pub fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    pub fn parser(input: &str) -> IResult<&str, Self> {
        let (pixels, input) = if let Some(idx) = input.find("\n\n") {
            input.split_at(idx)
        } else {
            (input, "")
        };

        let mut tiles = HashMap::new();
        for (y, line) in pixels.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(t) = T::from_char(c) {
                    let p = (x as i32, y as i32).into();
                    tiles.insert(p, t);
                }
            }
        }
        Ok((input, Self { tiles }))
    }

    pub fn get_range(&self) -> Option<((i32, i32), (i32, i32))> {
        Point::get_range(self.tiles.keys())
    }
}

impl<T: TileChar> std::fmt::Display for TileMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.get_range().unwrap();
        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if let Some(t) = self.tiles.get(&Point { x, y }) {
                    write!(f, "{}", t.to_char())?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: TileChar> std::ops::Deref for TileMap<T> {
    type Target = HashMap<Point, T>;
    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl<T: TileChar> std::ops::DerefMut for TileMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tileset_from_string() {
        let input = ".";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(
            tileset,
            TileSet {
                tiles: HashSet::new(),
                active_char: '#'
            }
        );

        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        let expected: HashSet<Point> = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .into_iter()
        .map(|t| t.into())
        .collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: '#'
            }
        );

        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        let expected: HashSet<Point> = [(0, 0), (1, 1), (2, 2)].iter().map(|&t| t.into()).collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: 'X'
            }
        );

        let input = "\
X.
.X

.X
X.";
        let tileset = TileSet::from_string::<'X'>(input);
        let expected: HashSet<Point> = [(0, 0), (1, 1)].iter().map(|&t| t.into()).collect();
        assert_eq!(
            tileset,
            TileSet {
                tiles: expected,
                active_char: 'X'
            }
        );
    }

    #[test]
    fn test_tileset_deref() {
        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        assert_eq!(tileset.contains(&(0, 0).into()), true);
        assert_eq!(tileset.contains(&(1, 1).into()), true);
        assert_eq!(tileset.contains(&(2, 2).into()), true);
        assert_eq!(tileset.contains(&(3, 3).into()), false);
        assert_eq!(tileset.contains(&(-1, -1).into()), false);
    }

    #[test]
    fn test_tileset_display() {
        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.to_string().trim(), input);

        let input = "\
X..
.X.
..X";
        let tileset = TileSet::from_string::<'X'>(input);
        assert_eq!(tileset.to_string().trim(), input);
    }

    #[test]
    fn test_tileset_range() {
        let input = ".";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.get_range(), None);

        let input = "\
###
#.#
###";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.get_range(), Some(((0, 2), (0, 2))));

        let input = "#.#.#.#.#.#.#.#.#";
        let tileset = TileSet::from_string::<'#'>(input);
        assert_eq!(tileset.get_range(), Some(((0, 16), (0, 0))));
    }

    #[derive(Debug, PartialEq)]
    enum MyTile {
        A,
        B,
        C,
    }

    impl TileChar for MyTile {
        fn to_char(&self) -> char {
            match self {
                Self::A => 'A',
                Self::B => 'B',
                Self::C => 'C',
            }
        }

        fn from_char(c: char) -> Option<Self> {
            Some(match c {
                'A' => Self::A,
                'B' => Self::B,
                'C' => Self::C,
                _ => return None,
            })
        }

        fn all_chars() -> Vec<char> {
            vec!['A', 'B', 'C']
        }
    }

    #[test]
    fn test_tilemap_from_string() {
        let input = ".";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(
            tilemap,
            TileMap {
                tiles: HashMap::new()
            }
        );

        let input = "\
ABC
A.A
CBA";
        let tilemap = TileMap::from_string(input);
        let expected: HashMap<Point, MyTile> = vec![
            ((0, 0), MyTile::A),
            ((1, 0), MyTile::B),
            ((2, 0), MyTile::C),
            ((0, 1), MyTile::A),
            ((2, 1), MyTile::A),
            ((0, 2), MyTile::C),
            ((1, 2), MyTile::B),
            ((2, 2), MyTile::A),
        ]
        .into_iter()
        .map(|(p, t)| (p.into(), t))
        .collect();
        assert_eq!(tilemap, TileMap { tiles: expected });

        let input = "\
AA
AA

BB
BB";
        let tilemap = TileMap::from_string(input);
        let expected: HashMap<Point, MyTile> = vec![
            ((0, 0), MyTile::A),
            ((1, 0), MyTile::A),
            ((0, 1), MyTile::A),
            ((1, 1), MyTile::A),
        ]
        .into_iter()
        .map(|(p, t)| (p.into(), t))
        .collect();
        assert_eq!(tilemap, TileMap { tiles: expected });
    }

    #[test]
    fn test_tilemap_deref() {
        let input = "\
A..
.B.
..C";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get(&(0, 0).into()).is_some(), true);
        assert_eq!(tilemap.get(&(1, 1).into()).is_some(), true);
        assert_eq!(tilemap.get(&(2, 2).into()).is_some(), true);
        assert_eq!(tilemap.get(&(3, 3).into()).is_some(), false);
        assert_eq!(tilemap.get(&(-1, -1).into()).is_some(), false);
    }

    #[test]
    fn test_tilemap_display() {
        let input = "\
ABC
A.A
CBA";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.to_string().trim(), input);

        let input = "\
A..
.B.
..C";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.to_string().trim(), input);
    }

    #[test]
    fn test_tilemap_range() {
        let input = ".";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get_range(), None);

        let input = "\
ABC
A.A
CBA";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get_range(), Some(((0, 2), (0, 2))));

        let input = "ABCABCABCABCABC";
        let tilemap: TileMap<MyTile> = TileMap::from_string(input);
        assert_eq!(tilemap.get_range(), Some(((0, 14), (0, 0))));
    }
}
