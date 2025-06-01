use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use smart_default::SmartDefault;

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Reflect, SmartDefault)]
pub struct SparkConfig {
    #[default(100.0)]
    pub max_charge: f32,
    #[default(50.0)]
    pub start_charge: f32,
    #[default(5.0)]
    pub decay_per_second: f32,
    #[default(1.0)]
    pub cost_per_m: f32,
    #[default(20.0)]
    pub damage_dealt_per_second: f32,
    #[default(50.0)]
    pub max_distance_jump_m: f32,
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {}
