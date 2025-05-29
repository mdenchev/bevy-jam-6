//! The game's menus and transitions between them.

mod credits;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_state_type]
#[auto_init_state]
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
    ));
}
