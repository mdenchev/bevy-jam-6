mod rng;

use crate::game::rng::RngPlugin;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    #[auto_plugin(app=app)]
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(RngPlugin);
    }
}
