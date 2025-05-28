use crate::game::screen::Screen;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(StateScoped::<Screen>)]
pub struct DemoObj;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(StateScoped::<Screen>)]
pub struct LightMesh;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DemoObj,
        Mesh3d(meshes.add(Sphere::new(100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(RED),
            perceptual_roughness: 0.1,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 300.0),
    ));

    commands.spawn((
        DemoObj,
        Mesh3d(meshes.add(Cuboid::new(500.0, 5.0, 500.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.25,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 300.0 + Vec3::Y * -100.0),
    ));

    commands.spawn((
        LightMesh,
        PointLight {
            intensity: 10000000.0,
            range: 1000.0,
            radius: 100.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(50.0, 50.0, -180.0)),
        children![(
            Mesh3d(meshes.add(Sphere::new(5.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::WHITE.to_linear(),
                unlit: true,
                ..Default::default()
            })),
        )],
    ));
}
