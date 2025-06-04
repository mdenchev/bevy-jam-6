use crate::game::behaviors::target_ent::TargetEnt;
use crate::game::camera::CameraTarget;
use crate::game::prefabs::bowling_ball::BowlingBall;
use crate::game::prefabs::enemy::Enemy;
use crate::game::prefabs::game_world::GameWorld;
use crate::game::prefabs::game_world_markers::{
    GameWorldMarkerSystemParam, auto_collider_mesh_obs,
};
use crate::game::prefabs::player::Player;
use crate::game::scenes::simple_bowling::{Facing, generate_pin_layout};
use crate::game::screens::Screen;
use crate::game::utils::extensions::vec2::Vec2Ext;
use avian3d::prelude::ExternalAngularImpulse;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_auto_plugin::auto_plugin::*;

pub fn spawn_level(mut commands: Commands) {
    info!("spawning world");
    commands
        .spawn((GameWorld, StateScoped(Screen::Gameplay)))
        .observe(auto_collider_mesh_obs)
        .observe(spawn_extras_on_instance_ready)
        .with_child((
            Name::new("Sun"),
            DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            CascadeShadowConfigBuilder {
                maximum_distance: 99999.9,
                ..Default::default()
            }
            .build(),
            Transform::from_translation(Vec3::Y * 100.0)
                .with_rotation(Quat::from_rotation_x(-45_f32.to_radians())),
        ));
}

fn spawn_extras_on_instance_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut game_world_marker: GameWorldMarkerSystemParam,
) {
    info!("Trigger<SceneInstanceReady>");
    game_world_marker
        .commands
        .entity(trigger.observer())
        .despawn();
    info!("spawning player");
    game_world_marker.spawn_in_player_spawn(
        (BowlingBall, ExternalAngularImpulse::new(Vec3::X * 5.0)),
        Some(Transform::from_scale(Vec3::splat(10.0))),
    );
    let player = game_world_marker.spawn_in_player_spawn((Player, CameraTarget), None);
    info!("spawning enemies");
    for pos in generate_pin_layout(20.0, 1.5, 3, Facing::Toward) {
        game_world_marker.spawn_in_enemy_spawn(
            (
                Enemy::BaseSkele,
                TargetEnt {
                    target_ent: player,
                    within_distance: 10.0,
                },
            ),
            Some(Transform::from_scale(Vec3::splat(10.0)).with_translation(pos.to_vec3())),
        );
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}
