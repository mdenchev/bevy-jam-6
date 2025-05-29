//! Spawn the main level.

use avian3d::prelude::Collider;
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
    let demo_obj1 = (
        DemoObj,
        demo_ob1_collider,
        Mesh3d(meshes.add(demo_ob1_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(color::palettes::css::RED),
            perceptual_roughness: 0.1,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::NEG_Z * 300.0),
    );

    let x_len = 500.0;
    let y_len = 5.0;
    let z_len = 500.0;
    let demo_ob2_mesh = Cuboid::new(x_len, y_len, z_len);
    let demo_ob2_collider = Collider::cuboid(x_len, y_len, z_len);
    let demo_obj2 = (
        DemoObj,
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
    let light = (
        LightMesh,
        light_collider,
        PointLight {
            intensity: 10000000.0,
            range: 1000.0,
            radius: 100.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(50.0, 50.0, -180.0)),
        children![(
            Mesh3d(meshes.add(light_mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::WHITE.to_linear(),
                unlit: true,
                ..Default::default()
            })),
        )],
    );

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            demo_obj1,
            demo_obj2,
            light,
        ],
    ));
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}
