mod asset_tracking;
mod camera;
mod demo;
#[cfg(feature = "dev")]
mod dev;
mod rng;
mod screen;

use crate::game::rng::RngPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetMetaCheck;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
#[cfg(feature = "dev_frame_count_log")]
use bevy_frame_count_log_prefix::prelude::FrameCountLogPrefixPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    #[auto_plugin(app=app)]
    fn build(&self, app: &mut App) {
        app.add_plugins(default_plugins());
        #[cfg(feature = "dev_frame_count_log")]
        app.add_plugins(FrameCountLogPrefixPlugin);
        app.add_plugins(RngPlugin);
        app.add_plugins(camera::plugin);
        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);
        app.add_plugins(asset_tracking::plugin);
        app.add_plugins(demo::plugin);
        app.add_plugins(screen::plugin);
    }
}

fn default_plugins() -> PluginGroupBuilder {
    let default_plugins = DefaultPlugins
        .build()
        // TODO: should we conditionally disable this based on wasm feature?
        .set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics on web build on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Window {
                title: "Bevy Jam 6".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }
            .into(),
            ..default()
        });
    #[cfg(feature = "dev_frame_count_log")]
    let default_plugins = default_plugins.disable::<LogPlugin>();
    default_plugins
}
