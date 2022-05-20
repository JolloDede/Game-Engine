extern crate game_engine_lib;

use std::sync::Arc;

use game_engine_lib::{application::AppTratis, log::info};

fn main() {
    info("test");

    let app = create_application();
    app.run();
}

fn create_application() -> impl AppTratis {
    return Sandbox();
}

struct Sandbox();

impl AppTratis for Sandbox {}