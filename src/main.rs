
mod data;
mod tui;

fn main() {
    let mut state = data::levels::level();
    let _ = tui::run_level(&mut state);
}
