#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn from_arrow(c: char) -> Self {
        match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => panic!("Cardinal: invalid input '{}'", c),
        }
    }

    pub fn to_arrow(&self) -> char {
        match *self {
            Self::North => '^',
            Self::South => 'v',
            Self::East => '>',
            Self::West => '<',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'N' | 'n' => Self::North,
            'S' | 's' => Self::South,
            'E' | 'e' => Self::East,
            'W' | 'w' => Self::West,
            _ => panic!("Invalid cardinal character: {}", c),
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::North => 'N',
            Self::South => 'S',
            Self::East => 'E',
            Self::West => 'W',
        }
    }

    pub fn turn(&self, dir: Turn) -> Self {
        match dir {
            Turn::Left => match *self {
                Self::North => Self::West,
                Self::South => Self::East,
                Self::East => Self::North,
                Self::West => Self::South,
            },

            Turn::Right => match *self {
                Self::North => Self::East,
                Self::South => Self::West,
                Self::East => Self::South,
                Self::West => Self::North,
            },
        }
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

impl std::fmt::Display for Cardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::North => write!(f, "North"),
            Self::South => write!(f, "South"),
            Self::East => write!(f, "East"),
            Self::West => write!(f, "West"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}
