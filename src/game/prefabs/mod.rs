pub mod bowling_ball;
pub mod bowling_pin;
pub mod enemy;
pub mod spawner;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(enemy::plugin);
    app.add_plugins(spawner::plugin);
    app.add_plugins(bowling_pin::plugin);
    app.add_plugins(bowling_ball::plugin);
}
