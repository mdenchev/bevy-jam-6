use crate::game::prefabs::game_world::GameWorld;
use avian3d::prelude::{Collider, ColliderConstructor, RigidBody, VhacdParameters};
use bevy::ecs::query::QueryData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_auto_plugin::auto_plugin::*;
use itertools::Itertools;
use smart_default::SmartDefault;
use std::collections::VecDeque;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct EnemySpawnMarker;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct PlayerSpawnMarker;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct OutOfBoundsMarker;

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Debug, Default, Copy, Clone, Reflect)]
pub struct LoadFinishedEvent;

#[auto_register_type]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct AutoColliderMesh;

#[auto_register_type]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(AutoColliderMesh)]
pub struct AutoColliderMeshNoApprox;

#[auto_register_type]
#[derive(Component, Debug, SmartDefault, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct DelayedRigidBody(#[default(RigidBody::Static)] RigidBody);

pub fn auto_collider_mesh_obs(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    auto_collider_mesh_q: Query<
        (Entity, Ref<AutoColliderMesh>, Has<AutoColliderMeshNoApprox>),
        Added<AutoColliderMesh>,
    >,
    delayed_rigid_body_q: Query<(Entity, Ref<DelayedRigidBody>), Added<DelayedRigidBody>>,
    added_mesh3d_q: Query<(Ref<Mesh3d>, Has<Collider>), Added<Mesh3d>>,
    children_q: Query<&Children>,
) {
    commands.entity(trigger.observer()).despawn();
    let entity = trigger.target();
    info!("Trigger<SceneInstanceReady> {entity}");
    for child in children_q.iter_descendants(entity) {
        if let Ok((entity, delayed_rigid_body)) = delayed_rigid_body_q.get(child) {
            if delayed_rigid_body.is_added() {
                info!("DelayedRigidBody {entity} {delayed_rigid_body:?}");
                commands
                    .entity(entity)
                    .remove::<DelayedRigidBody>()
                    .insert(delayed_rigid_body.0);
            }
        }
        let Ok((entity, auto_collider_mesh_ref, has_auto_collider_mesh_no_approx)) =
            auto_collider_mesh_q.get(child)
        else {
            continue;
        };
        if !auto_collider_mesh_ref.is_added() {
            continue;
        }
        let queue = vec![entity]
            .into_iter()
            .chain(children_q.iter_descendants(entity));
        for entity in queue {
            let Ok((mesh3d_ref, has_collider)) = added_mesh3d_q.get(entity) else {
                continue;
            };
            if !mesh3d_ref.is_added() || has_collider {
                continue;
            }
            info!("ConvexHullFromMesh {entity}");
            let mut entity_cmds = commands.entity(entity);
            if !has_auto_collider_mesh_no_approx {
                entity_cmds.insert(ColliderConstructor::ConvexHullFromMesh);
            } else {
                entity_cmds.insert(ColliderConstructor::ConvexDecompositionFromMeshWithConfig(
                    VhacdParameters {
                        convex_hull_approximation: false,
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

#[derive(QueryData)]
pub struct EntityWithGlobalTransform {
    pub entity: Entity,
    pub global_transform: Ref<'static, GlobalTransform>,
}

#[derive(SystemParam)]
pub struct GameWorldMarkerSystemParam<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub game_world: Single<'w, EntityWithGlobalTransform, With<GameWorld>>,
    pub player_spawn: Single<'w, EntityWithGlobalTransform, With<PlayerSpawnMarker>>,
}

impl GameWorldMarkerSystemParam<'_, '_> {
    pub fn spawn_in_player_spawn(&mut self, bundle: impl Bundle, transform: Option<Transform>) {
        let transform = transform.unwrap_or_default();
        let transform_target = self
            .player_spawn
            .global_transform
            .reparented_to(&self.game_world.global_transform);
        let final_transform = transform * transform_target;
        let child = self.commands.spawn(bundle).insert(final_transform).id();
        self.commands
            .entity(self.game_world.entity)
            .add_child(child);
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}
