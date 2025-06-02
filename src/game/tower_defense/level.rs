use std::f32::consts::PI;
use std::time::Duration;

use crate::game::camera::CameraTarget;
use crate::game::screens::Screen;
use crate::game::tower_defense::tower::Tower;
use crate::game::tower_defense::wizard::Wizard;
use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use super::behaviors::target_ent::TargetEnt;
use super::enemy::Enemy;
use super::lightning_ball::LightningBall;
use super::spawner::Spawner;

#[auto_register_type]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct LevelRoot;

#[auto_plugin(app=_app)]
pub(crate) fn plugin(_app: &mut App) {}

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_ent = commands
        .spawn((
            Name::new("Level"),
            LevelRoot,
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Visibility::default(),
            children![
                (
                    PointLight {
                        color: Color::srgba(0.933, 0.966, 0.806, 1.000),
                        intensity: 9999999999.0,
                        range: 100000.0,
                        radius: 999.0,
                        shadows_enabled: true,
                        ..Default::default()
                    },
                    Transform::from_xyz(30.0, 300.0, 80.0),
                ),
                (
                    Name::new("Grass"),
                    Mesh3d(meshes.add(Cuboid::new(1000.0, 10.0, 1000.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::from(GREEN),
                        perceptual_roughness: 1.0,
                        reflectance: 0.0,
                        ..Default::default()
                    })),
                ),
            ],
        ))
        .id();
    let tower_ent = commands
        .entity(level_ent)
        .with_child((
            Tower,
            Transform::from_xyz(0.0, 50.0, 0.0),
            children![
                (
                    Wizard,
                    Transform::from_xyz(0.0, 80.0, 0.0).with_scale(Vec3::splat(10.0)),
                ),
                (
                    LightningBall,
                    CameraTarget,
                    Transform::from_xyz(0.0, 3.1 * 10.0 + 100.0, 0.8 * 10.0),
                ),
            ],
        ))
        .id();
    // Spawners
    for (x, y) in equidistant_points_on_circle(300., 5) {
        commands.entity(level_ent).with_child((
            Spawner {
                spawns: Enemy::BaseSkele,
                spawn_duration: Duration::from_secs_f32(4.0),
                time_to_next_spawn: Duration::from_secs_f32(0.0),
                spawn_left: 4,
            },
            Transform::from_xyz(x, 10.0, y),
        ));
    }
    // Prespawned skele
    commands.entity(level_ent).with_child((
        Name::new("Skele"),
        Enemy::BaseSkele,
        Transform::from_xyz(100.0, 10.0, 100.0).with_scale(Vec3::splat(15.0)),
        TargetEnt {
            target_ent: tower_ent,
            within_distance: 20.0,
        },
    ));
}

/// Generate points along a circle.
// TODO move to a more appropriate place
pub fn equidistant_points_on_circle(radius: f32, num_points: usize) -> Vec<(f32, f32)> {
    if radius < 0.0 {
        panic!("Radius cannot be negative.");
    }

    if num_points == 0 {
        return Vec::new();
    }

    (0..num_points)
        .map(|i| {
            let angle = (i as f32) * (2.0 * PI) / (num_points as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            (x, y)
        })
        .collect()
}
