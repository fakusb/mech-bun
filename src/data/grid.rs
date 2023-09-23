use std::ops::Add;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i8,
    y: i8,
}

impl<N: Into<i8>> From<(N, N)> for Position {
    fn from(t: (N, N)) -> Self {
        Position { x: t.0.into(), y: t.1.into() }
    }
}

impl Into<(i8, i8)> for Position {
    fn into(self) -> (i8, i8) {
        (self.x, self.y)
    }
}

impl Add<Position> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Position) -> Self::Output {
        match (self.x.checked_add(rhs.x), self.y.checked_add(rhs.y)) {
            (Some(x), Some(y)) => Some((x, y).into()),
            _ => None,
        }
    }
}

impl Position {
    pub fn moveTo(self, d: Direction) -> Option<Self> {
        self + d.offset()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroundTile {
    Hole,
    Wall,
    Floor,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileItem {
    Paquerette,
    Bun,
    Bunstack,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GroundTile {
    pub fn to_unicode(self) -> char {
        match self {
            GroundTile::Hole => '🕳',
            GroundTile::Wall => '▮',
            GroundTile::Floor => '.',
        }
    }
}

impl TileItem {
    pub fn to_unicode(self) -> char {
        match self {
            TileItem::Paquerette => '👧',
            TileItem::Bun => '🐇',
            TileItem::Bunstack => '🗼',
        }
    }
}
impl From<char> for TileItem {
    fn from(c: char) -> Self {
        if(['b','B'].contains(&c)) {
            return TileItem::Bun;
        } else if (['p','P'].contains(&c)) {
            return TileItem::Paquerette
        } else {
            panic!("Unrecognised character: {c}");
        }
    }
}

impl Direction {
    pub fn offset(self) -> Position {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
        .into()
    }
}
