mod fps;
mod inspector_ui;
mod selection;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(fps::plugin);
    #[cfg(feature = "inspector_ui")]
    {
        app.add_plugins(inspector_ui::plugin);
        app.add_plugins(selection::plugin);
    }
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
