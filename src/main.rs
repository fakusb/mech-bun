#![feature(return_position_impl_trait_in_trait)]



mod data;
mod tui;

fn main() {
    let mut state = data::levels::level1();
    let _ = tui::run_level(&mut state);
}
