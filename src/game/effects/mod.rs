use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

pub mod lightning_ball;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(lightning_ball::plugin);
}
