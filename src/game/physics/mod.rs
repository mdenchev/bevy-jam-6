use crate::game::pause_controller::Pause;
use avian3d::prelude::{
    Physics, PhysicsDebugPlugin, PhysicsGizmos, PhysicsInterpolationPlugin, PhysicsPickingPlugin,
    PhysicsPlugins, PhysicsTime,
};
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::extrapolate_all()));
    app.add_plugins(PhysicsPickingPlugin);
    #[cfg(feature = "dev")]
    {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.world_mut()
            .resource_mut::<GizmoConfigStore>()
            .config_mut::<PhysicsGizmos>()
            .0
            .enabled = false;
    }
    app.add_systems(OnEnter(Pause(false)), |mut time: ResMut<Time<Physics>>| {
        time.unpause();
    });
    app.add_systems(OnEnter(Pause(true)), |mut time: ResMut<Time<Physics>>| {
        time.pause();
    });
}
