use std::ops::Add;

use super::grid::{Direction, LEVEL_WIDTH, LEVEL_HEIGHT};

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
        if self.x >= LEVEL_WIDTH as i8 {
            self.x = 0;
            self.y += 1;
            if self.y >= LEVEL_HEIGHT as i8 {
                return None;
            }
        };
        return Some(self.clone());
    }
}

impl Position {
    /// Iterate over inner positions of a level
    pub fn iter() -> impl Iterator<Item = Position> {
        Position{x:-1,y:0}
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
        return Ok((x, y));
    }
}

impl Add<Position> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Position) -> Self::Output {
        match (self.x.checked_add(rhs.x), self.y.checked_add(rhs.y)) {
            (Some(x), Some(y)) => Some((x, y).try_into().ok()?),
            _ => None,
        }
    }
}

impl Position {
    pub fn into_clamped_usize<N: TryInto<usize>>(self, max_x: N, max_y: N) -> Option<(usize, usize)> {
        let (x, y) = TryInto::<(usize, usize)>::try_into(self).ok()?;
        let max_x : usize = max_x.try_into().ok()?;
        let max_y : usize = max_y.try_into().ok()?;
        if x >= max_x || y >= max_y {
            return None;
        }
        return Some((x, y));
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn test_position_order() {
        assert!(Position { x: 1, y: 1 } >= Position { x: 2, y: 0 });
    }
}

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.offset()
    }
}
