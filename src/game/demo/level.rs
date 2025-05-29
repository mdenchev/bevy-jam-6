//! Spawn the main level.

use avian3d::prelude::{Collider, DistanceJoint, ExternalImpulse, GravityScale, Joint, RigidBody};
use bevy::color;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::camera::CameraTarget;
use crate::game::{asset_tracking::LoadResource, audio::music, screens::Screen};

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct DemoObj;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct DemoFloor;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct DemoLight;

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let demo_obj_mesh = Sphere::new(100.0);
    let demo_obj_collider = Collider::sphere(demo_obj_mesh.radius);
    let demo_obj_transform = Transform::from_translation(Vec3::NEG_Z * 300.0);
    let demo_obj = commands
        .spawn((
            DemoObj,
            CameraTarget,
            RigidBody::Static,
            demo_obj_collider,
            Mesh3d(meshes.add(demo_obj_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(color::palettes::css::RED),
                perceptual_roughness: 0.1,
                ..Default::default()
            })),
            demo_obj_transform,
        ))
        .id();

    let x_len = 500.0;
    let y_len = 5.0;
    let z_len = 500.0;
    let demo_floor_mesh = Cuboid::new(x_len, y_len, z_len);
    let demo_floor_collider = Collider::cuboid(x_len, y_len, z_len);
    let demo_floor = (
        DemoFloor,
        RigidBody::Static,
        demo_floor_collider,
        Mesh3d(meshes.add(demo_floor_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.25,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 300.0 + Vec3::Y * -100.0),
    );

    let demo_light_mesh = Sphere::new(5.0);
    let demo_light_collider = Collider::sphere(demo_light_mesh.radius);
    let demo_light_transform = Transform::from_translation(Vec3::new(50.0, 50.0, -180.0));
    let demo_light = commands
        .spawn((
            DemoLight,
            RigidBody::Dynamic,
            ExternalImpulse::new(Vec3::new(6000.0, 150.0, 3000.0)).with_persistence(true),
            GravityScale(0.0),
            demo_light_collider,
            PointLight {
                intensity: 10000000.0,
                range: 1000.0,
                radius: 100.0,
                shadows_enabled: true,
                ..Default::default()
            },
            demo_light_transform,
            children![(
                Mesh3d(meshes.add(demo_light_mesh)),
                MeshMaterial3d(materials.add(StandardMaterial {
                    emissive: Color::WHITE.to_linear(),
                    unlit: true,
                    ..Default::default()
                })),
            )],
        ))
        .id();

    let distance_min_max =
        (demo_obj_transform.translation - demo_light_transform.translation).length();

    commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                ),
                demo_floor
            ],
        ))
        .add_child(demo_light)
        .add_child(demo_obj)
        .with_child(
            DistanceJoint::new(demo_obj, demo_light)
                .with_limits(distance_min_max, distance_min_max),
        );
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}
