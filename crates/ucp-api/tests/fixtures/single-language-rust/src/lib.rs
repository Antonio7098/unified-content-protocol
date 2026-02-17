pub mod util;

pub struct AppState {
    pub name: String,
}

pub fn run() -> AppState {
    AppState { name: util::greet() }
}
