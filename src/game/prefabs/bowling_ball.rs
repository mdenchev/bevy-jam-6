use crate::game::asset_tracking::LoadResource;
use avian3d::prelude::RigidBody;
use avian3d::prelude::{Collider, Friction, Mass, Restitution};
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[auto_name]
#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[require(Transform)]
#[require(Visibility)]
#[require(RigidBody::Dynamic)]
pub struct BowlingBall;

pub const BOWLING_BALL_RADIUS: f32 = 0.108;

#[auto_register_type]
#[derive(Resource, Asset, Debug, Clone, Reflect)]
pub struct BowlingBallAssets {
    #[dependency]
    pub bowling_ball: Handle<Scene>,
}

impl FromWorld for BowlingBallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bowling_ball: assets
                .load(GltfAssetLabel::Scene(0).from_asset("models/bowling/bowling_ball.glb")),
        }
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    app.load_resource::<BowlingBallAssets>();
    app.add_observer(on_added);
}

fn on_added(
    trigger: Trigger<OnAdd, BowlingBall>,
    assets: Res<BowlingBallAssets>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).insert((
        Collider::sphere(BOWLING_BALL_RADIUS),
        SceneRoot(assets.bowling_ball.clone()),
        Friction::new(0.1),
        Restitution::new(0.41),
        Mass(50.0),
    ));
}
