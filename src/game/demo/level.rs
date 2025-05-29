//! Spawn the main level.

use avian3d::prelude::{Collider, DistanceJoint, ExternalImpulse, GravityScale, Joint, RigidBody};
use bevy::color;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

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
pub struct LightMesh;

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let demo_ob1_mesh = Sphere::new(100.0);
    let demo_ob1_collider = Collider::sphere(demo_ob1_mesh.radius);
    let demo_ob1_transform = Transform::from_translation(Vec3::NEG_Z * 300.0);
    let demo_obj1 = commands
        .spawn((
            DemoObj,
            RigidBody::Static,
            demo_ob1_collider,
            Mesh3d(meshes.add(demo_ob1_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(color::palettes::css::RED),
                perceptual_roughness: 0.1,
                ..Default::default()
            })),
            demo_ob1_transform,
        ))
        .id();

    let x_len = 500.0;
    let y_len = 5.0;
    let z_len = 500.0;
    let demo_ob2_mesh = Cuboid::new(x_len, y_len, z_len);
    let demo_ob2_collider = Collider::cuboid(x_len, y_len, z_len);
    let demo_obj2 = (
        DemoObj,
        RigidBody::Static,
        demo_ob2_collider,
        Mesh3d(meshes.add(demo_ob2_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.25,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 300.0 + Vec3::Y * -100.0),
    );

    let light_mesh = Sphere::new(5.0);
    let light_collider = Collider::sphere(light_mesh.radius);
    let light_transform = Transform::from_translation(Vec3::new(50.0, 50.0, -180.0));
    let light = commands
        .spawn((
            LightMesh,
            RigidBody::Dynamic,
            ExternalImpulse::new(Vec3::new(6000.0, 150.0, 3000.0)).with_persistence(true),
            GravityScale(0.0),
            light_collider,
            PointLight {
                intensity: 10000000.0,
                range: 1000.0,
                radius: 100.0,
                shadows_enabled: true,
                ..Default::default()
            },
            light_transform,
            children![(
                Mesh3d(meshes.add(light_mesh)),
                MeshMaterial3d(materials.add(StandardMaterial {
                    emissive: Color::WHITE.to_linear(),
                    unlit: true,
                    ..Default::default()
                })),
            )],
        ))
        .id();

    let distance_min_max = (demo_ob1_transform.translation - light_transform.translation).length();

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
                demo_obj2
            ],
        ))
        .add_child(light)
        .add_child(demo_obj1)
        .with_child(
            DistanceJoint::new(demo_obj1, light).with_limits(distance_min_max, distance_min_max),
        );
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}
