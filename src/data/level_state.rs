
use super::{grid::{GroundTile, TileItem, Tunnels, LEVEL_WIDTH, LEVEL_HEIGHT, Direction}, Position};


type TileContent = (GroundTile, Option<TileItem>);
pub trait LevelState {
    fn new() -> Self;
    fn get_at(&self, p: Position) -> Option<TileContent>;
    fn set_tile_at(&mut self, p: Position, t : GroundTile) -> ();
    fn set_paquerette(&mut self, p: Position) -> ();
    fn set_bunny(&mut self, p: Position) -> ();

    fn get_paquerette(&self) -> Position;
    //fn get_bunies(&mut self, p: Position) -> impl Iterator<Item = Position>;

    fn content(&self) -> impl Iterator<Item = (Position, TileContent)> {
        Position::iter().map(|p|(p,self.get_at(p).expect("should be inbound")))
    }

    fn to_unicode_string(&self) -> String {
        let cap: usize = (LEVEL_WIDTH as usize + 1) * LEVEL_HEIGHT as usize * 4 /* 4 bytes per unicode symbol */;
        let mut res = String::with_capacity(cap);
        for (p,(tile,item)) in self.content() {
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

    fn parse_level<I>(input : &str) -> Result<Self,()>
    where 
        Self : Sized
    {
        let mut res = Self::new();
        let mut segs = input.split(|c: char|c.is_whitespace() || c == ',').filter(|str|!str.is_empty());
        for p in Position::iter() {
            let c = segs.next().ok_or(())?;
            let (c,rem) = c.split_at(1);
            if rem != "" {
                eprintln!("Extra Tile attributes not yet implemented");
                return Err(())
            }
            let mut tile : GroundTile = GroundTile::Floor;
            let tunnels : Tunnels =   Default::default();
            if c == "W" {
                tile = GroundTile::Wall { breakable: false , tunnels};
            } else if c == "R" {
                tile = GroundTile::Wall { breakable: true, tunnels};
            } else if c == "T" {
                tile = GroundTile::Floor;
            } else if c == "E" {
                tile = GroundTile::Hole;
            } else if c == "S" {
                res.set_paquerette(p);
            } else if c == "B" {
                res.set_bunny(p);
            } else {
                eprintln!("Level object {c} not yet implemented");
                return Err(());
            }
            res.set_tile_at(p, tile);
            
        }
        Ok(res)
    }

    fn index_for(&self, p:Position) -> Option<(usize,usize)>{
        p.into_clamped_usize(LEVEL_WIDTH,LEVEL_HEIGHT)
    }

    fn move_to(&mut self, d: Direction) -> Result<(),()>{
        let new_pos = (self.get_paquerette() + d).ok_or(())?;
        if self.get_at(new_pos).ok_or(())?.0.is_solid() {
            return Err(());
        }
        self.set_paquerette(new_pos);
        Ok(())
    }
}



#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LevelStateNaive {
    data: Box<[[GroundTile; LEVEL_HEIGHT as usize]; LEVEL_WIDTH as usize]>,
    paquerette: Position,
    bunnys: Vec<Position>,
}

impl LevelState for LevelStateNaive {

    fn get_at(&self, p: Position) -> Option<(GroundTile, Option<TileItem>)> {

        let (x,y) = self.index_for(p)?;
        let mut res_item = None;
        if self.paquerette == p {
            res_item = Some(TileItem::Paquerette);
        } else if self.bunnys.contains(&p) {
            res_item = Some(TileItem::Bun);
        }
        Some ((self.data[x][y],res_item))
    }



    fn set_tile_at(&mut self, p: Position, t : GroundTile) -> () {
        let (x,y) = self.index_for(p).expect("in range");
        self.data[x][y] = t;
    }

    fn new() -> Self {
        Default::default()
    }

    fn set_paquerette(&mut self, p: Position) -> () {
        self.paquerette = p
    }

    fn set_bunny(&mut self, p: Position) -> () {
        self.bunnys.push(p)
    }

    fn get_paquerette(&self) -> Position {
        self.paquerette
    }

}
