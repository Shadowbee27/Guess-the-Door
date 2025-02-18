mod menu;
mod gamefiles {
    pub mod game;
    pub mod store;
}
mod lib {
    pub mod check_for_scoreboard;
    pub mod decode;
    pub mod input;
    pub mod remove_access_scores;
}
fn main() {
    menu::start_menu();
}
