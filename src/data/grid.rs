use std::ops::{Index, IndexMut};

use super::Position;

pub const LEVEL_WIDTH: i8 = 15;
pub const LEVEL_HEIGHT: i8 = 9;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Tunnels {
    tunnels: [bool; 4],
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GroundTile {
    Hole,
    Wall { breakable: bool, tunnels: Tunnels },
    Floor,
}

impl Default for GroundTile {
    fn default() -> Self {
        Self::Floor
    }
}

impl Default for Tunnels {
    fn default() -> Self {
        Tunnels {
            tunnels: [false; 4],
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TileItem {
    Paquerette,
    Bun,
    #[allow(dead_code)]
    Bunstack,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Index<Direction> for Tunnels {
    type Output = bool;

    fn index(&self, index: Direction) -> &bool {
        return &self.tunnels[index as usize];
    }
}

impl IndexMut<Direction> for Tunnels {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        return &mut self.tunnels[index as usize];
    }
}

impl GroundTile {
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
            GroundTile::Floor => ' ',
        }
    }

    pub fn is_solid(self) -> bool {
        match self {
            GroundTile::Wall { .. } => true,
            GroundTile::Floor | GroundTile::Hole => false,
        }
    }
}

impl TileItem {
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
            return TileItem::Bun;
        } else if ['p', 'P'].contains(&c) {
            return TileItem::Paquerette;
        } else {
            panic!("Unrecognised character: {c}");
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
