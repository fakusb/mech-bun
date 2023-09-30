use super::level_state::{LevelStateNaive, LevelState};


macro_rules! level_path {
  ($rel_path:literal) => {
      concat!("../../burrows/Demo Remake v1.1.2/", $rel_path)
  }
}


pub fn level1() -> LevelStateNaive {
  LevelState::parse_level::<LevelStateNaive>(include_str!(level_path!("DemoCenter/1.level"))).expect("valid")
}