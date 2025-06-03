use crate::game::utils::extensions::vec2::Vec2Ext;
use std::f32::consts::PI;
use std::time::Duration;

use crate::game::camera::CameraTarget;
use crate::game::effects::lightning_ball::{LightningBall, LightningBallConduit};
use crate::game::prefabs::bowling_ball::{BOWLING_BALL_RADIUS, BowlingBall};
use crate::game::prefabs::bowling_pin::{BowlingPin, PIN_HEIGHT, PIN_WIDTH};
use crate::game::prefabs::enemy::Enemy;
use crate::game::prefabs::spawner::Spawner;
use crate::game::prefabs::tower::Tower;
use crate::game::prefabs::wizard::Wizard;
use crate::game::screens::Screen;
use avian3d::prelude::{
    AngularVelocity, CenterOfMass, Collider, ExternalAngularImpulse, ExternalImpulse, Friction,
    Mass, Restitution, RigidBody, Sensor,
};
use bevy::color::palettes::css::GREEN;
use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
pub struct LevelRoot;

#[auto_plugin(app=_app)]
pub fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Away,
    Toward,
}
pub fn generate_pin_layout(pin_width: f32, spacing: f32, rows: usize, facing: Facing) -> Vec<Vec2> {
    let mut positions = Vec::new();
    for r in 0..rows {
        let num_in_row = (rows - r) as f32;
        let y = (r as f32) * (pin_width + spacing);
        // total width occupied by this row: N * pin_width + (N - 1) * spacing
        let row_width = num_in_row * pin_width + (num_in_row - 1.0) * spacing;

        // The first pin’s center x should be at:
        //   -row_width/2 + pin_width/2
        // so that the row is centered around x = 0.0
        let start_x = -row_width / 2.0 + pin_width / 2.0;

        for i in 0..(num_in_row as usize) {
            let x = start_x + (i as f32) * (pin_width + spacing);
            let y = match facing {
                Facing::Away => -y,
                Facing::Toward => y,
            };
            positions.push(Vec2::new(x, y));
        }
    }

    positions
}

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const LANE_SIZE_X: f32 = 1.054;
    const LANE_SIZE_Y: f32 = 0.1;
    const LANE_SIZE_Z: f32 = 18.29;
    const PIN_SPACING: f32 = 0.1; // 2.82?
    const PIN_ROWS: usize = 4;
    let pins = generate_pin_layout(PIN_WIDTH, PIN_SPACING, PIN_ROWS, Facing::Away);

    let pin_bundles = pins.into_iter().map(|p| {
        println!("spawning pin {}", p.to_vec3());
        (
            // multi-line
            BowlingPin,
            Transform::from_translation(p.to_vec3() + Vec3::NEG_Z * LANE_SIZE_Z / 2.5),
        )
    });
    const END_SIZE_RATIO: f32 = 0.40;
    const START_SIZE_RATIO: f32 = 1.0 - END_SIZE_RATIO;
    let start_size = LANE_SIZE_Z * START_SIZE_RATIO;
    let end_size = LANE_SIZE_Z * END_SIZE_RATIO;
    let start_half = start_size / 2.0;
    let end_half = end_size / 2.0;
    let total_half = LANE_SIZE_Z / 2.0;
    commands.spawn((
        LevelRoot,
        StateScoped(Screen::Gameplay),
        Children::spawn((
            Spawn((
                Name::new("Light"),
                PointLight {
                    intensity: 99999.0,
                    range: 99999.0,
                    radius: 99999.0,
                    ..Default::default()
                },
                Transform::from_translation(Vec3::Y * 7.0),
            )),
            Spawn((
                Name::new("Lane"),
                Mesh3d(meshes.add(Cuboid::new(LANE_SIZE_X, LANE_SIZE_Y, LANE_SIZE_Z))),
                RigidBody::Static,
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.283, 0.198, 0.106),
                    perceptual_roughness: 0.0,
                    reflectance: 1.0,
                    ..Default::default()
                })),
                // Restitution::new(0.1),
                Collider::cuboid(LANE_SIZE_X, LANE_SIZE_Y, LANE_SIZE_Z),
                Friction::new(0.4),
                // children![
                //     (
                //         Name::new("LanePartWithOil"),
                //         Friction::new(0.1),
                //         Collider::cuboid(LANE_SIZE_X, LANE_SIZE_Y, start_size),
                //         Transform::from_xyz(0.0, 0.0, total_half - start_half),
                //     ),
                //     (
                //         Name::new("LanePartWithoutOil"),
                //         Friction::new(1.0),
                //         Collider::cuboid(LANE_SIZE_X, LANE_SIZE_Y, end_size),
                //         Transform::from_xyz(0.0, 0.0, -total_half + end_half),
                //     )
                // ],
            )),
            SpawnIter(pin_bundles),
            Spawn((
                BowlingBall,
                CameraTarget,
                Transform::from_xyz(0.0, BOWLING_BALL_RADIUS, LANE_SIZE_Z / 2.1),
                ExternalImpulse::new(Vec3::NEG_Z * 10.0),
                ExternalAngularImpulse::new(Vec3::NEG_X * 1.0),
            )),
        )),
    ));
}
