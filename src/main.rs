#![allow(unused)]
use data::world;

mod data;
mod tui;

fn main() -> anyhow::Result<()> {
    let dir_worlds = std::env::current_dir().unwrap().join("burrows");
    let worlds = world::bnys_loader::load_worlds(dir_worlds)?;
    let world = &worlds[0];

    let mut state = world.enter()?;

    let _ = tui::run_world(&mut state);

    // let mut state = data::levels::level();
    // let _ = tui::run_level(&mut state);
    Ok(())
}
