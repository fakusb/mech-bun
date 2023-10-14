pub mod bnys_loader;

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use anyhow::anyhow;

use super::level_state::MoveEffect;
use super::{Direction, Item, LevelState};

use strum::EnumCount;

#[derive(Debug)]
pub struct World {
    title: String,
    burrows: Vec<Rc<RefCell<Burrow>>>,
}

#[derive(Debug, Default)]
struct Burrow {
    has_surface_entry: bool,
    links: [Option<Weak<RefCell<Burrow>>>; Direction::COUNT],
    levels: Vec<Option<LevelTemplate>>, // indexing starts with 1, so 0 is always supposed to be None,
}

type BurrowRef = Rc<RefCell<Burrow>>;

impl Burrow {
    pub fn get_link(&self, dir: Direction) -> anyhow::Result<BurrowRef> {
        self.links[dir]
            .clone()
            .ok_or_else(|| anyhow!("Can't go there. Yet?!?"))
            .and_then(|p| {
                p.upgrade()
                    .ok_or(anyhow!("Weak pointer was already deleted"))
            })
    }
}

#[derive(Debug)]
struct LevelTemplate {
    name: String,
    data: String,
    tools: [u8; Item::COUNT],
}

pub struct WorldState<'a> {
    world: &'a World,
    burrow: BurrowRef,
    depth: usize,
    pub level_state: super::LevelState,
}

impl World {
    pub fn enter(&self) -> anyhow::Result<WorldState> {
        for rc in &self.burrows {
            let b = rc.borrow();
            if b.levels.len() <= 1 || b.levels[1].is_none() {
                continue;
            }
            if b.has_surface_entry {
                let mut state = LevelState::new();
                state
                    .parse_level(&b.levels[1].as_ref().unwrap().data)
                    .map_err(|_| anyhow!("Parsing error"));
                return Ok(WorldState {
                    world: self,
                    burrow: rc.clone(),
                    depth: 1,
                    level_state: state,
                });
            }
        }
        Err(anyhow!("Could not find borrow accessible from top"))
    }
}

impl<'a> WorldState<'a> {
    pub(crate) fn apply_move_effect(&mut self, e: MoveEffect) -> anyhow::Result<()> {
        match e {
            MoveEffect::DropHole() => {
                todo!()
            }
            MoveEffect::MoveAdjacent(dir) => {
                let target_burrow = self.burrow.as_ref().borrow().get_link(dir)?;
                let target_level = target_burrow.borrow().levels[self.depth]
                    .ok_or_else(|| anyhow!("Level was not declared"))?;
                self.
                self.burrow = target_burrow;
                self.Ok(())
            }
        }
    }
}
