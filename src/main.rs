mod menu;
mod gamefiles {
    pub mod game;
    pub mod store;
}
mod lib {
    pub mod input;
}
fn main() {
    let tmane = String::from("Test");
    let tscore = 28;
    gamefiles::store::store(tmane, tscore);
    // menu::start_menu();
}
