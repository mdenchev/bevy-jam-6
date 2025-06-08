use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{pause_controller::Pause, scenes::LevelData, screens::Screen};

fn condition(ld: Res<LevelData>, mut next_screen: ResMut<NextState<Screen>>) {
    if ld.temple_health == 0 {
        next_screen.set(Screen::End);
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, condition.run_if(in_state(Pause(false))));
}
