use crate::game::prefabs::game_world::GameWorld;
use avian3d::parry::mass_properties::MassProperties;
use avian3d::prelude::{
    AngularInertia, CenterOfMass, Collider, ColliderConstructor, Mass, NoAutoAngularInertia,
    NoAutoCenterOfMass, NoAutoMass, RigidBody, VhacdParameters,
};
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
#[derive(Debug, Default, Copy, Clone, Reflect)]
pub enum Method {
    #[default]
    ConvexHull,
    ConvexDecomposition,
    ConvexDecompositionNoApprox,
    TriMesh,
}

#[auto_register_type]
#[derive(Component, Debug, SmartDefault, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct AutoColliderMesh {
    method: Method,
}

pub fn auto_collider_mesh_obs(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    auto_collider_mesh_q: Query<
        (Entity, Ref<AutoColliderMesh>, Option<&RigidBody>),
        Added<AutoColliderMesh>,
    >,
    added_mesh3d_q: Query<(Ref<Mesh3d>, Has<Collider>), Added<Mesh3d>>,
    children_q: Query<&Children>,
) {
    commands.entity(trigger.observer()).despawn();
    let entity = trigger.target();
    info!("Trigger<SceneInstanceReady> {entity}");
    for child in children_q.iter_descendants(entity) {
        let Ok((entity, auto_collider_mesh_ref, rigid_body_opt)) = auto_collider_mesh_q.get(child)
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
            let has_rigid_body = rigid_body_opt.is_some();
            let rigid_body = rigid_body_opt.copied().unwrap_or(RigidBody::Static);
            if matches!(rigid_body, RigidBody::Static) {
                // required for large meshes to prevent: assertion failed: self.is_normalized()
                //  avian3d::dynamics::rigid_body::mass_properties::update_mass_properties
                entity_cmds.insert((
                    NoAutoMass,
                    NoAutoAngularInertia,
                    NoAutoCenterOfMass,
                    Mass::ZERO,
                    AngularInertia::ZERO,
                    CenterOfMass::ZERO,
                ));
            }
            if !has_rigid_body {
                entity_cmds.insert(rigid_body);
            }
            match auto_collider_mesh_ref.method {
                Method::ConvexHull => {
                    entity_cmds.insert(ColliderConstructor::ConvexHullFromMesh);
                }
                Method::ConvexDecomposition => {
                    entity_cmds.insert(ColliderConstructor::ConvexDecompositionFromMesh);
                }
                Method::ConvexDecompositionNoApprox => {
                    entity_cmds.insert(ColliderConstructor::ConvexDecompositionFromMeshWithConfig(
                        VhacdParameters {
                            convex_hull_approximation: false,
                            ..Default::default()
                        },
                    ));
                }
                Method::TriMesh => {
                    entity_cmds.insert(ColliderConstructor::TrimeshFromMesh);
                }
            }
        }
    }
}

#[derive(QueryData)]
pub struct EntityWithGlobalTransformQueryData {
    pub entity: Entity,
    pub global_transform: Ref<'static, GlobalTransform>,
}

#[derive(SystemParam)]
pub struct GameWorldMarkerSystemParam<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub game_world: Single<'w, EntityWithGlobalTransformQueryData, With<GameWorld>>,
    pub player_spawn: Single<'w, EntityWithGlobalTransformQueryData, With<PlayerSpawnMarker>>,
    pub transform_helper: TransformHelper<'w, 's>,
}

impl GameWorldMarkerSystemParam<'_, '_> {
    fn get_or_compute_global_transform(
        &self,
        target: &EntityWithGlobalTransformQueryDataItem,
        error_msg: &str,
    ) -> GlobalTransform {
        let gt_res = if *target.global_transform == GlobalTransform::default() {
            self.transform_helper
                .compute_global_transform(target.entity)
        } else {
            Ok(*target.global_transform)
        };
        gt_res.expect(error_msg)
    }
    pub fn spawn_in_player_spawn(
        &mut self,
        bundle: impl Bundle,
        transform: Option<Transform>,
    ) -> Entity {
        let transform = transform.unwrap_or_default();
        let player_spawn_global_transform =
            self.get_or_compute_global_transform(&self.player_spawn, "PlayerSpawnMarker");

        let game_world_global_transform =
            self.get_or_compute_global_transform(&self.game_world, "GameWorld");

        let transform_target =
            player_spawn_global_transform.reparented_to(&game_world_global_transform);
        // remove scale before applying transform and re-add it back
        let final_transform =
            (transform.with_scale(Vec3::splat(1.0)) * transform_target).with_scale(transform.scale);
        let child = self.commands.spawn(bundle).insert(final_transform).id();
        self.commands
            .entity(self.game_world.entity)
            .add_child(child);
        child
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}
