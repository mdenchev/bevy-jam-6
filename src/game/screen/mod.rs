mod gameplay;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_init_state]
#[auto_register_state_type]
#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Gameplay,
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(gameplay::plugin);
}
