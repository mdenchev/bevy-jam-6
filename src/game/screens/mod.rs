//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

/// The game's main screen states.
#[auto_init_state]
#[auto_register_state_type]
#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}
