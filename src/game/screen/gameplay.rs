use crate::game::demo::spawn_level;
use crate::game::screen::Screen;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level).chain());
}
