use avian3d::prelude::{
    PhysicsDebugPlugin, PhysicsGizmos, PhysicsInterpolationPlugin, PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::extrapolate_all()));
    #[cfg(feature = "dev")]
    {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.world_mut()
            .resource_mut::<GizmoConfigStore>()
            .config_mut::<PhysicsGizmos>()
            .0
            .enabled = false;
    }
}
