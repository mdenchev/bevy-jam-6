pub(crate) mod fx;
pub(crate) mod level;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(fx::plugin);
    app.add_plugins(level::plugin);
}
