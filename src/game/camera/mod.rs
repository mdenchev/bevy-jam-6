use bevy::pbr::ShadowFilteringMethod;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component)]
#[require(Camera3d)]
#[require(ShadowFilteringMethod::Hardware2x2)]
pub struct MainCamera;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(AmbientLight {
        brightness: 2.0,
        ..Default::default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}
