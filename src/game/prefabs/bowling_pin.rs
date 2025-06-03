use crate::game::asset_tracking::LoadResource;
use avian3d::prelude::{
    CenterOfMass, Collider, ColliderConstructor, ColliderConstructorHierarchy, ColliderTransform,
    Friction, Mass, Restitution, Sleeping, VhacdParameters,
};
use avian3d::prelude::{ComputedCenterOfMass, RigidBody};
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use std::process::Child;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Dynamic)]
pub struct BowlingPin;

pub const PIN_WIDTH: f32 = 0.12;
pub const PIN_HEIGHT: f32 = 0.38;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
pub struct BowlingPinAssets {
    #[dependency]
    pub bowling_pin: Handle<Scene>,
    #[dependency]
    pub bowling_pin_mesh: Handle<GltfMesh>,
}

impl FromWorld for BowlingPinAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        const PATH: &str = "models/bowling/bowling_pin.glb";
        Self {
            bowling_pin: assets.load(GltfAssetLabel::Scene(0).from_asset(PATH)),
            bowling_pin_mesh: assets.load(GltfAssetLabel::Mesh(0).from_asset(PATH)),
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<BowlingPinAssets>();
    app.add_observer(on_added);
}

fn on_added(
    trigger: Trigger<OnAdd, BowlingPin>,
    mut local: Local<Option<Collider>>,
    assets: Res<BowlingPinAssets>,
    transform_q: Query<&Transform, With<BowlingPin>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    let entity = trigger.target();

    let collider = if let Some(collider) = local.as_ref() {
        collider.clone()
    } else {
        let Some(gltf) = gltf_meshes.get(&assets.bowling_pin_mesh) else {
            panic!("failed to get gltf mesh for {entity}");
        };
        let Some(mesh_handle) = gltf.primitives.first() else {
            panic!("failed to get gltf mesh primitive for {entity}");
        };
        let Some(mesh) = meshes.get(mesh_handle.mesh.id()) else {
            panic!("failed to get mesh for {entity}");
        };
        let Some(collider) = Collider::convex_decomposition_from_mesh_with_config(
            mesh,
            &VhacdParameters {
                convex_hull_approximation: false,
                ..Default::default()
            },
        ) else {
            panic!("failed to create collider for BowlingPin");
        };
        assert!(
            local.replace(collider.clone()).is_none(),
            "collider already created - impossible"
        );
        collider
    };

    let Ok(transform) = transform_q.get(entity) else {
        panic!("failed to get transform for bowling pin {entity}");
    };
    commands.entity(entity).insert((
        SceneRoot(assets.bowling_pin.clone()),
        RigidBody::Dynamic,
        Restitution::new(0.1),
        Friction::new(0.5),
        Mass(1.5),
        CenterOfMass::new(0.0, -0.05, 0.0),
        // Collider::cylinder(PIN_WIDTH / 2.0, PIN_HEIGHT),
        collider,
        transform.with_translation(transform.translation + Vec3::Y * (PIN_HEIGHT / 2.0 + 0.06)),
    ));
}
