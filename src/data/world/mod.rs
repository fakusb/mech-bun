pub mod bnys_loader;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use anyhow::anyhow;

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
    levels: Vec<Option<Level>>, // indexing starts with 1, so 0 is always supposed to be None,
}

#[derive(Debug)]
struct Level {
    name: String,
    data: String,
    tools: [u8; Item::COUNT],
}

pub struct WorldState<'a> {
    world: &'a World,
    borrow: Rc<RefCell<Burrow>>,
    depth: usize,
    pub level_state: super::LevelState,
}

impl World {
    pub fn enter(&self) -> anyhow::Result<WorldState> {
        for rc in &self.burrows {
            let b = rc.borrow();
            if b.levels.len() <= 1 || !b.levels[1].is_some() {
                continue;
            }
            if b.has_surface_entry {
                let mut state = LevelState::new();
                state.parse_level(&b.levels[1].as_ref().unwrap().data).map_err(|_|anyhow!("Parsing error"));
                return Ok(WorldState {
                    world: self,
                    borrow: rc.clone(),
                    depth: 1,
                    level_state: state,
                });
            }
        }
        Err(anyhow!("Could not find borrow accessible from top"))
    }
}
