pub mod behaviors;
mod enemy;
pub mod level;
mod tower;
mod wizard;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    behaviors::plugin(app);
    enemy::plugin(app);
    tower::plugin(app);
    wizard::plugin(app);
    level::plugin(app);
}
