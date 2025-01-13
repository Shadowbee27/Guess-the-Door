mod menu;
mod gamefiles {
    pub mod game;
    pub mod store;
}
mod lib {
    pub mod input;
}
fn main() {
    menu::start_menu();
}
