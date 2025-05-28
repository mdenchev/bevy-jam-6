use bevy::prelude::*;
use bj6_game::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(GamePlugin);
    app.run();
}
