use serde::Deserialize;
use serde_json::{Map, Value};
use std::{fs::File, path::Path};

use super::*;
use anyhow::{anyhow, Context, Result};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BnysConfig {
    enabled: bool,
    title: String,
    burrows: Vec<BnysBurrow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BnysBurrow {
    directory: String,
    name: String,
    indicator: String,
    has_surface_entry: bool,
    depth: usize,
    links: BnysLinks,
    #[serde(default)]
    elevator_depths: Vec<usize>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields, default)]
struct BnysLinks {
    left: String,
    up: String,
    right: String,
    down: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase", default)]
struct BnysLevel {
    name: String,
    tools: BnysTools,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields, default)]
struct BnysTools {
    traps: u8,
    pickaxes: u8,
    carrots: u8,
    shovels: u8,
}

pub fn load_worlds(path: impl AsRef<Path>) -> Result<Vec<World>> {
    let path: &Path = path.as_ref();
    let dir = path
        .read_dir()
        .with_context(|| format!("Opening {}", path.display()))?;
    let mut worlds = Vec::new();
    for dir in dir {
        let dir = dir.with_context(|| format!("Opening {}", path.display()))?;
        if dir.file_type()?.is_dir() {
            worlds.push(self::load_world(dir.path().as_path())?);
        }
    }
    Ok(worlds)
}

fn load_world(world_dir: impl AsRef<Path>) -> Result<World> {
    let world_dir = world_dir.as_ref();
    let config_path = world_dir.join("config.json");
    let config = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Reading {}.", config_path.display()))?;
    let config: BnysConfig = serde_json::from_str(&config)
        .and_then(serde_json::from_value)
        .with_context(|| format!("Parsing {}.", config_path.display()))?;

    let mut burrows: Vec<Rc<RefCell<Burrow>>> = Vec::new();
    burrows.resize_with(config.burrows.len(),Default::default);

    // get names for backlinks
    let mut burrows_by_name: HashMap<String, Weak<RefCell<Burrow>>> = Default::default();
    for (rc, burrow) in std::iter::zip(burrows.iter(), config.burrows.iter()) {
        let inserted = burrows_by_name
            .insert(burrow.name.clone(), Rc::downgrade(rc))
            .is_none();
        if !inserted {
            return Err(anyhow!(
                "Duplicate burrow {} in {}",
                burrow.name,
                config_path.display()
            ));
        }
    }

    // fill burrow data
    for (rc, burrow) in std::iter::zip(burrows.iter(), config.burrows.iter()) {
        let mut to_fill = RefCell::borrow_mut(rc);

        *to_fill = Burrow {
            has_surface_entry: burrow.has_surface_entry,
            links: links_to_array(&burrow.links, &burrows_by_name)?,
            levels: load_levels(
                world_dir.join(burrow.directory.clone()).as_path(),
                burrow.depth,
            )?,
        }
    }

    Ok(World {
        title: config.title,
        burrows,
    })
}

fn load_levels(level_dir: impl AsRef<Path>, depth: usize) -> Result<Vec<Option<Level>>> {
    let level_dir = level_dir.as_ref();
    let mut levels = Vec::new();
    levels.resize_with(depth + 1, || None);
    for id in 1..=depth {
        let path_json = level_dir.join(format!("{id}.json"));
        let path_level = level_dir.join(format!("{id}.level"));
        let both_exist = path_json.as_path()
            .try_exists()
            .and_then(|b1| path_level.try_exists().map(|b2| (b1, b2)))
            .with_context(|| {
                format!(
                    "Accessing files {} and {}.",
                    path_json.display(),
                    path_level.display()
                )
            })?;
        if both_exist == (false, false) {
            continue;
        } else if both_exist != (true, true) {
            return Err(anyhow!(
                "Could only find one of the files {} and {}.",
                path_json.display(),
                path_level.display()
            ))?;
        }
        let json = std::fs::read_to_string(&path_json)
            .with_context(|| format!("Accessing file {}.", path_json.display()))?;
        let json: BnysLevel = serde_json::from_str(&json)
            .with_context(|| format!("Parsing file {}.", path_json.display()))?;
        let level_data = std::fs::read_to_string(&path_level)
            .with_context(|| format!("Accessing file {}.", path_level.display()))?;

        levels[id] = Some(Level {
            name: json.name,
            data: level_data,
            tools: tools_to_array(&json.tools),
        })
        //let Ok(str = std::fs::read_to_string(level_dir.join(format!("{id}.level")))
    }
    Ok(levels)
}

fn links_to_array(
    links: &BnysLinks,
    burrows_by_name: &HashMap<String, Weak<RefCell<Burrow>>>,
) -> Result<[Option<Weak<RefCell<Burrow>>>; Direction::COUNT]> {
    let mut res: [Option<Weak<RefCell<Burrow>>>; Direction::COUNT] = Default::default();
    let mut work = |dir, link: &str| {
        let target = burrows_by_name.get(link);
        let Some(target) = target else {
            if !["", "__UNLINKED__"].contains(&link) {
                return Err(anyhow!("Unknown burrow in `Link`: {link}"))
            }
            return Ok(());
        };
        res[dir as usize] = Some(target.clone());
        Ok(())
    };
    work(Direction::Left, &links.left)?;
    work(Direction::Right, &links.right)?;
    work(Direction::Up, &links.up)?;
    work(Direction::Down, &links.down)?;
    Ok(res)
}

fn tools_to_array(tools: &BnysTools) -> [u8; Item::COUNT] {
    let mut res : [u8; Item::COUNT]= Default::default();
    let mut work = |item, amount: u8| {
        res[item as usize] = amount;
    };
    work(Item::Trap, tools.traps);
    work(Item::Pickaxe, tools.pickaxes);
    work(Item::Carrot, tools.carrots);
    work(Item::Shovel, tools.shovels);
    res
}
