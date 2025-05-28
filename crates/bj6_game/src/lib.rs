use bevy::prelude::*;
use bj6_rng::RngPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(RngPlugin);
    }
}
