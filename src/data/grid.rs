use std::ops::{Index, IndexMut, Neg};
use strum::EnumCount;
use strum_macros::{EnumIter, FromRepr, EnumCount};

use super::Position;

pub const LEVEL_WIDTH: i8 = 15;
pub const LEVEL_HEIGHT: i8 = 9;

pub(crate) type Tunnels = [bool;Direction::COUNT];

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GroundTile {
    Hole,
    Wall { breakable: bool, tunnels: Tunnels },
    Floor { is_entry: bool },
}

impl Default for GroundTile {
    fn default() -> Self {
        Self::Floor { is_entry: false }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TileItem {
    Paquerette,
    Bun,
    #[allow(dead_code)]
    Bunstack,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        Direction::from_repr((self as usize + 1) % 4).unwrap()
    }

    pub fn turn_right(self) -> Direction {
        Direction::from_repr((self as usize + 3) % 4).unwrap()
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction::from_repr((self as usize + 2) % 4).unwrap()
    }
}

impl<T> Index<Direction> for [T; Direction::COUNT] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<Direction> for [T; Direction::COUNT] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl GroundTile {
    #[allow(unused)]
    pub fn to_unicode(self) -> char {
        match self {
            GroundTile::Hole => 'o',
            GroundTile::Wall { breakable: b, .. } => {
                if b {
                    '░'
                } else {
                    '▓'
                }
            }
            GroundTile::Floor { .. } => ' ',
        }
    }

    pub fn is_solid(self) -> bool {
        match self {
            GroundTile::Wall { .. } => true,
            GroundTile::Floor { .. } | GroundTile::Hole => false,
        }
    }
    pub fn is_hole(self) -> bool {
        match self {
            GroundTile::Hole { .. } => true,
            GroundTile::Floor { .. } | GroundTile::Wall { .. } => false,
        }
    }
    pub fn is_solid_for_bun_from(self, _: Direction) -> bool {
        match self {
            GroundTile::Wall { .. } => true,
            GroundTile::Floor { .. } | GroundTile::Hole => false,
        }
    }
}

impl TileItem {
    #[allow(unused)]
    pub fn to_unicode(self) -> char {
        match self {
            TileItem::Paquerette => 'P',
            TileItem::Bun => 'b',
            TileItem::Bunstack => '🗼',
        }
    }
}
impl From<char> for TileItem {
    fn from(c: char) -> Self {
        if ['b', 'B'].contains(&c) {
            TileItem::Bun
        } else if ['p', 'P'].contains(&c) {
            TileItem::Paquerette
        } else {
            panic!("Unrecognised character: {c}")
        }
    }
}

impl Direction {
    pub fn offset(self) -> Position {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
        .try_into()
        .expect("constants")
    }
}
