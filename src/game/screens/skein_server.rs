//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use crate::game::screens::Screen;
use crate::game::theme::widget;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

fn spawn_skein_server_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Skein Server Screen"),
        StateScoped(Screen::Loading),
        children![widget::label(
            "Skein Server Running - Use Skein Add-On in Blender to fetch bevy type registry"
        )],
    ));
}
#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::SkeinServer), spawn_skein_server_screen);
}
