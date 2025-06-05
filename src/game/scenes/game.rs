use crate::game::behaviors::target_ent::TargetEnt;
use crate::game::camera::CameraTarget;
use crate::game::effects::lightning_ball::{LightningBall, LightningBallConduit};
use crate::game::prefabs::bowling_ball::BowlingBall;
use crate::game::prefabs::enemy::Enemy;
use crate::game::prefabs::game_world::GameWorld;
use crate::game::prefabs::game_world_markers::{
    GameWorldMarkerSystemParam, auto_collider_mesh_obs,
};
use crate::game::prefabs::player::{Player, PlayerSystemParam};
use crate::game::scenes::simple_bowling::{Facing, generate_pin_layout};
use crate::game::screens::Screen;
use crate::game::utils::extensions::vec2::Vec2Ext;
use avian3d::prelude::{ExternalAngularImpulse, ExternalImpulse, Friction, Mass};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_auto_plugin::auto_plugin::*;
use smart_default::SmartDefault;
use std::ops::{Deref, DerefMut};

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
    mut commands: Commands,
    mut game_world_marker: GameWorldMarkerSystemParam,
) {
    info!("Trigger<SceneInstanceReady>");
    commands.entity(trigger.observer()).despawn();
    info!("spawning player");
    let player = game_world_marker.spawn_in_player_spawn(Player, None);
    info!("spawning enemies");
    for pos in generate_pin_layout(5.0, 0.5, 3, Facing::Toward) {
        game_world_marker.spawn_in_enemy_spawn(
            (
                Enemy::BaseSkele,
                LightningBallConduit,
                Mass(1.0),
                Friction::new(0.4),
                TargetEnt {
                    // TODO: spawn point doesnt work?
                    target_ent: player, // game_world_marker.player_spawn.entity,
                    within_distance: 10.0,
                },
            ),
            Some(Transform::from_scale(Vec3::splat(4.0)).with_translation(pos.to_vec3())),
        );
    }
}

#[derive(Debug, SmartDefault)]
struct DemoCache {
    #[default = 1.0]
    power: f32,
    #[default = 0.0]
    accuracy: f32,
    #[default = 1.0]
    turn_rate: f32,
}
fn demo_input(
    time: Res<Time>,
    mut commands: Commands,
    mut local: Local<(bool, DemoCache)>,
    mut player_system_param: PlayerSystemParam,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    let mut apply_transform = |transform: Transform| {
        commands
            .entity(player_system_param.entity())
            .insert(transform);
    };
    let (changed, cache) = &mut *local;
    if button_input.pressed(KeyCode::ArrowLeft) {
        cache.accuracy += 1.0;
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowRight) {
        cache.accuracy -= 1.0;
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowUp) {
        cache.power += 0.1;
        *changed = true;
    }
    if button_input.pressed(KeyCode::ArrowDown) {
        cache.power -= 0.1;
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyW) {
        cache.turn_rate += 1.0;
        cache.turn_rate = cache.turn_rate.max(1.0);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyS) {
        cache.turn_rate -= 1.0;
        cache.turn_rate = cache.turn_rate.max(1.0);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyA) {
        let mut transform = player_system_param.player_transform.clone();
        transform.rotate(Quat::from_rotation_y(
            1_f32.to_radians() * cache.turn_rate * time.delta_secs(),
        ));
        apply_transform(transform);
        *changed = true;
    }
    if button_input.pressed(KeyCode::KeyD) {
        let mut transform = player_system_param.player_transform.clone();
        transform.rotate(Quat::from_rotation_y(
            -1_f32.to_radians() * cache.turn_rate * time.delta_secs(),
        ));
        apply_transform(transform);
        *changed = true;
    }
    if *changed {
        *changed = false;
        info!(
            "demo value updated: {cache:?} player_rotation: {}",
            player_system_param.player_transform.rotation
        );
    }
    if button_input.just_pressed(KeyCode::Space) {
        player_system_param.spawn_bowling_ball(cache.power, cache.accuracy);
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, demo_input);
}
