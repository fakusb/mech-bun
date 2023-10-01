use super::level_state::LevelState;


macro_rules! level_path {
  ($rel_path:literal) => {
      concat!("../../burrows/Demo Remake v1.1.2/", $rel_path)
  }
}


pub fn level() -> LevelState {
  let mut res = LevelState::new();
  res.parse_level(include_str!(level_path!("DemoCenter/2.level"))).expect("valid");
  res
}