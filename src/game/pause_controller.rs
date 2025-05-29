use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

/// Whether or not the game is paused.
#[auto_register_state_type]
#[auto_init_state]
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[states(scoped_entities)]
pub(super) struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) struct PausableSystems;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
}
