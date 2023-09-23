use std::fmt::{Display, Formatter};
use either::Either;
use super::grid::{GroundTile, Position, TileItem};

pub trait LevelState {
    fn width(&self) -> i8;
    fn height(&self) -> i8;
    fn get_at_pos(&self, p: Position) -> Option<Either<GroundTile, TileItem>>;

    fn to_string(&self) -> String {
        let cap: usize = (self.width() as usize + 1) * self.height() as usize * 4 /* 4 bytes per unicode symbol */;
        let mut res = String::with_capacity(cap);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.get_at_pos((x,y).into()).expect("was in range");
                let char = tile.either(GroundTile::to_unicode,TileItem::to_unicode);
                res.push(char);
            }
            res.push('\n');
        }
        assert_eq!(res.capacity(),cap,"Computed capacity should suffice");
        return res;
    }
}

const LEVEL_WIDTH: i8 = 15;
const LEVEL_HEIGHT: i8 = 11;
struct LevelStateNaive {
    data: [[GroundTile; LEVEL_WIDTH as usize]; LEVEL_HEIGHT as usize],
    paquerette: Position,
    bunny: Option<Position>,
}

impl LevelState for LevelStateNaive {
    fn width(&self) -> i8 {
        return LEVEL_WIDTH;
    }
    fn height(&self) -> i8 {
        return LEVEL_HEIGHT;
    }

    fn get_at_pos(&self, p: Position) -> Option<Either<GroundTile, TileItem>> {
        todo!()
    }
}
