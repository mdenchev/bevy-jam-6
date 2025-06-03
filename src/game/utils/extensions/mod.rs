pub mod vec2;
pub mod vec3;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(vec2::plugin);
    app.add_plugins(vec3::plugin);
}
