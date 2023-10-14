use std::ops::{Add, AddAssign};

use super::grid::{Direction, LEVEL_HEIGHT, LEVEL_WIDTH};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Position {
    y: i8,
    x: i8,
}

impl Position {
    pub fn x(&self) -> i8 {
        self.x
    }
    pub fn y(&self) -> i8 {
        self.y
    }

    #[allow(dead_code)]
    pub fn is_start_of_inner_row(&self) -> bool {
        self.x == 0 && self.y > 0
    }
    #[allow(dead_code)]
    pub fn is_inner(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < LEVEL_WIDTH && self.y < LEVEL_HEIGHT
    }
}

/// Iterate over inner positions of a level
impl Iterator for Position {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x >= LEVEL_WIDTH {
            self.x = 0;
            self.y += 1;
            if self.y >= LEVEL_HEIGHT {
                return None;
            }
        };
        Some(*self)
    }
}

impl Position {
    /// Iterate over inner positions of a level
    pub fn iter() -> impl Iterator<Item = Position> {
        Position { x: -1, y: 0 }
    }
}

impl<N: TryInto<i8>> TryFrom<(N, N)> for Position {
    type Error = N::Error;

    fn try_from(t: (N, N)) -> Result<Position, <N as TryInto<i8>>::Error> {
        Ok(Position {
            x: t.0.try_into()?,
            y: t.1.try_into()?,
        })
    }
}

impl<N: TryFrom<i8>> TryInto<(N, N)> for Position {
    type Error = N::Error;

    fn try_into(self) -> Result<(N, N), <N as TryFrom<i8>>::Error> {
        let x = self.x.try_into()?;
        let y = self.y.try_into()?;
        Ok((x, y))
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        match (self.x.checked_add(rhs.x), self.y.checked_add(rhs.y)) {
            (Some(x), Some(y)) => (x, y).try_into().unwrap(),
            _ => unreachable!(),
        }
    }
}

impl Position {
    pub fn into_clamped_usize<N: TryInto<usize>>(
        self,
        max_x: N,
        max_y: N,
    ) -> Option<(usize, usize)> {
        let (x, y) = TryInto::<(usize, usize)>::try_into(self).ok()?;
        let max_x: usize = max_x.try_into().ok()?;
        let max_y: usize = max_y.try_into().ok()?;
        if x >= max_x || y >= max_y {
            return None;
        }
        Some((x, y))
    }

    /// distance and direction same position gest arbitrary direction, and distance 0
    ///
    pub fn distance_to_straight_line(self, p: Position) -> Option<(Direction, u8)> {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        if dx == 0 {
            if dy < 0 {
                Some((Direction::Down, -dy as u8))
            } else {
                Some((Direction::Up, dy as u8))
            }
        } else if self.y == p.y {
            if dx < 0 {
                Some((Direction::Right, -dx as u8))
            } else {
                Some((Direction::Left, dx as u8))
            }
        } else {
            None
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.offset()
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = self.add(rhs)
    }
}

impl Add<Option<Direction>> for Position {
    type Output = Position;

    fn add(self, rhs: Option<Direction>) -> Self::Output {
        rhs.map_or(self, |d|{self+d})
    }
}


#[cfg(test)]
mod tests {
    use crate::data::grid::Direction;

    use super::Position;
    use strum::IntoEnumIterator;

    #[test]
    fn test_position_order() {
        assert!(Position { x: 1, y: 1 } >= Position { x: 2, y: 0 });
    }

    #[test]
    fn test_distance_to_straight_line() {
        let p = Position::default();
        for d in Direction::iter() {
            assert_eq!(p.distance_to_straight_line(p + d), Some((d, 1)))
        }
    }
}
