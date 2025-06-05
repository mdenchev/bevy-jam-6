use crate::game::asset_tracking::LoadResource;
use crate::game::prefabs::game_world::GameWorld;
use crate::game::prefabs::game_world_markers::{
    BowlingBallSpawnMarker, EntityWithGlobalTransformQueryData, SpawnHelper,
};
use avian3d::prelude::RigidBody;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Kinematic)]
pub struct Player;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
pub struct PlayerAssets {
    #[dependency]
    pub scene: Handle<Scene>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            scene: assets.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("models/zeus/zeus_rigged_manual_bowling_ball.glb"),
            ),
        }
    }
}

#[derive(SystemParam)]
pub struct PlayerSystemParam<'w, 's> {
    pub bowling_ball_spawn: SpawnHelper<'w, 's, GameWorld, BowlingBallSpawnMarker>,
}

impl PlayerSystemParam<'_, '_> {
    pub fn spawn_bowling_ball_spawn(
        &mut self,
        bundle: impl Bundle,
        transform: Option<Transform>,
    ) -> Entity {
        self.bowling_ball_spawn.spawn_in(bundle, transform)
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.add_observer(on_added);
}

fn on_added(trigger: Trigger<OnAdd, Player>, assets: Res<PlayerAssets>, mut commands: Commands) {
    let entity = trigger.target();
    commands
        .entity(entity)
        .insert(SceneRoot(assets.scene.clone()));
}
