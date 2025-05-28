use bevy::prelude::*;
use bevy_game_jam_6::game::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(GamePlugin);
    app.run();
}
