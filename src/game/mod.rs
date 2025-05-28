mod rng;

use crate::game::rng::RngPlugin;
use bevy::app::PluginGroupBuilder;
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
    }
}

fn default_plugins() -> PluginGroupBuilder {
    let default_plugins = DefaultPlugins.build();
    #[cfg(feature = "dev_frame_count_log")]
    let default_plugins = default_plugins.disable::<LogPlugin>();
    default_plugins
}
