use super::{
    grid::{Direction, GroundTile, TileItem, Tunnels, LEVEL_HEIGHT, LEVEL_WIDTH},
    Position,
};

pub type TileContent = (GroundTile, Option<TileItem>);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
pub struct LevelState {
    data: Box<[[GroundTile; LEVEL_HEIGHT as usize]; LEVEL_WIDTH as usize]>,
    paquerette: Position,
    buns: Vec<Option<Position>>,
}

impl LevelState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_at(&self, p: Position) -> Option<(GroundTile, Option<TileItem>)> {
        let (x, y) = self.index_for(p)?;
        let mut res_item = None;
        if self.paquerette == p {
            res_item = Some(TileItem::Paquerette);
        } else if self.buns.contains(&Some(p)) {
            res_item = Some(TileItem::Bun);
        }
        Some((self.data[x][y], res_item))
    }

    pub fn set_tile_at(&mut self, p: Position, t: GroundTile) -> () {
        let (x, y) = self.index_for(p).expect("in range");
        self.data[x][y] = t;
    }

    pub fn set_paquerette(&mut self, p: Position) -> () {
        self.paquerette = p
    }

    pub fn set_bunny(&mut self, p: Position) -> () {
        self.buns.push(Some(p))
    }

    pub fn get_paquerette(&self) -> Position {
        self.paquerette
    }

    pub fn content<'a>(&'a self) -> impl Iterator<Item = (Position, TileContent)> + 'a {
        Position::iter().map(|p| (p, self.get_at(p).expect("should be inbound")))
    }

    pub fn to_unicode_string(&self) -> String {
        let cap: usize = (LEVEL_WIDTH as usize + 1) * LEVEL_HEIGHT as usize * 4 /* 4 bytes per unicode symbol */;
        let mut res = String::with_capacity(cap);
        for (p, (tile, item)) in self.content() {
            if p.x() == 0 && p.y() > 0 {
                res += "\n";
            }
            let char = item.map_or(tile.to_unicode(), TileItem::to_unicode);
            res.push(char);
        }
        if cap != res.capacity() {
            eprintln!("Computed capacity should have suffice.");
        }
        return res;
    }

    pub fn parse_level(&mut self, input: &str) -> Result<(), ()>
    where
        Self: Sized,
    {
        let mut segs = input
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|str| !str.is_empty());
        for p in Position::iter() {
            let c = segs.next().ok_or(())?;
            let (c, rem) = c.split_at(1);
            if rem != "" {
                eprintln!("Extra Tile attributes not yet implemented");
                return Err(());
            }
            let mut tile: GroundTile = GroundTile::Floor { isEntry: false };
            let tunnels: Tunnels = Default::default();
            if c == "W" {
                tile = GroundTile::Wall {
                    breakable: false,
                    tunnels,
                };
            } else if c == "R" {
                tile = GroundTile::Wall {
                    breakable: true,
                    tunnels,
                };
            } else if c == "T" {
                tile = GroundTile::Floor { isEntry: false };
            } else if c == "E" {
                tile = GroundTile::Hole;
            } else if c == "S" {
                tile = GroundTile::Floor { isEntry: true };
                self.set_paquerette(p);
            } else if c == "B" {
                self.set_bunny(p);
            } else {
                eprintln!("Level object {c} not yet implemented");
                return Err(());
            }
            self.set_tile_at(p, tile);
        }
        Ok(())
    }

    pub fn index_for(&self, p: Position) -> Option<(usize, usize)> {
        p.into_clamped_usize(LEVEL_WIDTH, LEVEL_HEIGHT)
    }

    pub fn move_to(&mut self, d: Direction) -> Result<(), ()> {
        let new_pos = self.get_paquerette() + d;
        if self.get_at(new_pos).ok_or(())?.0.is_solid() {
            return Err(());
        }
        self.set_paquerette(new_pos);

        for bun_index in 0..self.buns.len() {
            let Some(mut bun) = self.buns[bun_index] else {
                continue;
            };
            
            // if not in level anymore, skip
            if !bun.is_inner() {
                continue;
            }

            //TODO: loop this for tunnels

            //if can't see paquerette, skip
            let Some((mut dir, mut dist)) = self.paquerette.distance_to_straight_line(bun) else {
                continue;
            };

            if dist == 0 {
                self.buns[bun_index] = None;
                continue;
            }

            //if not close, skip
            if dist > 2 {
                continue;
            }

            //if P can't get to the bun, skip
            if self.is_solid(self.paquerette + dir) || dist == 2 && self.is_solid(self.paquerette + dir + dir){

            }

            let mut started_moving = false;
            let mut last_branch_alternative: Option<(Position, Direction)> = None;
            let mut last_branch_was_after_move = false;
            loop {
                let mut dir_to_move = None;
                let dir_cands = [dir, dir.turn_left(), dir.turn_right()];
                for dir_try in dir_cands {
                    if !self.bun_can_see_deadend(bun, dir_try) {
                        dir_to_move = Some(dir_try);
                        break;
                    }
                }

                if dir_to_move.is_none() {
                    // otherwise, move to first free direction
                    for dir_try in dir_cands {
                        if !self.is_solid_for_bun_from(bun + dir_try, dir_try) {
                            dir_to_move = Some(dir_try);
                            break;
                        }
                    }
                }

                let Some(dir_to_move) = dir_to_move else {
                    break;
                };

                started_moving = true;
                bun += dir_to_move;
                while self
                    .is_solid_for_bun_from(bun + dir_to_move.turn_left(), dir_to_move.turn_left())
                    && self.is_solid_for_bun_from(
                        bun + dir_to_move.turn_right(),
                        dir_to_move.turn_right(),
                    ) && !self.is_solid_for_bun_from(bun + dir_to_move, dir_to_move)
                {
                    bun += dir_to_move;
                }


                // let straight = bun + dir;
                // let left = bun + dir.turn_left();
                // let right: Position = bun + dir.turn_right();
                // let can_straight = !self
                //     .get_at(straight)
                //     .is_some_and(|(t, _)| t.is_solid_for_bun_from(-dir));
                // let can_left = !self
                //     .get_at(left)
                //     .is_some_and(|(t, _)| t.is_solid_for_bun_from(-dir));
                // let can_right = !self
                //     .get_at(right)
                //     .is_some_and(|(t, _)| t.is_solid_for_bun_from(-dir));

                // let can_branch = can_left || can_right;
                // if can_left {
                //     last_branch_alternative = Some((left, dir.turn_left()));
                //     last_branch_was_after_move = started_moving;
                // } else if can_right {
                //     last_branch_alternative = Some((right, dir.turn_right()));
                //     last_branch_was_after_move = started_moving;
                // }
                // //continue straight if in narrow corridor or close to P
                // if can_straight && (dist < 2 || started_moving && !can_branch) {
                //     bun = straight;
                //     dist += 1;
                //     started_moving = true;
                //     continue;
                // }

                // //timewarp if we ran into a dead end
                // if !can_straight && last_branch_was_after_move {
                //     if let Some((bun1, dir1)) = last_branch_alternative {
                //         bun = bun1;
                //         dir = dir1;
                //         started_moving = true;
                //         dist = 100; //hack so we don't see P anymore
                //         continue;
                //     }
                // }
                // // if dist < 2 {
                // //     if can_left {
                // //         bun = left
                // //     } else if can_right {
                // //         bun = right
                // //     }
                // // }
                break;
            }
            self.buns[bun_index] = Some(bun);
        }
        Ok(())
    }

    pub fn bun_can_see_deadend(&self, mut cur_pos: Position, dir: Direction) -> bool {
        cur_pos += dir;
        while !self.is_solid_for_bun_from(cur_pos,dir) && cur_pos.is_inner() {
            //println!("{dir:?}{cur_pos:?}");
            cur_pos += dir;
            let left = dir.turn_left();
            let right = dir.turn_right();
            //todo: add "seing depth"
            if !self.is_solid_for_bun_from(cur_pos + left, left)
                || !self.is_solid_for_bun_from(cur_pos + right, right)
            {
                return false;
            }
        }
        return true;
    }

    pub fn is_solid_for_bun_from(&self, p: Position, dir: Direction) -> bool {
        self
            .get_at(p)
            .is_some_and(|(t, _)| t.is_solid_for_bun_from(dir))
    }

    pub fn is_solid(&self, p: Position) -> bool {
        self
            .get_at(p)
            .is_some_and(|t| t.0.is_solid())
    }

}
