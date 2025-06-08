use crate::game::behaviors::despawn::Despawn;
use crate::game::behaviors::stopwatch::{Stopwatch, register_stopwatch};
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use std::cell::RefMut;

#[auto_register_type]
#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
#[component(immutable)]
#[require(DeadFor)]
pub struct Dead;

pub type DeadFor = Stopwatch<Dead>;

#[derive(QueryData)]
pub struct DeadQueryData {
    pub entity: Entity,
    pub dead: Ref<'static, Dead>,
    pub dead_for: Ref<'static, DeadFor>,
    pub despawn: Option<Ref<'static, Despawn>>,
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {
    register_stopwatch::<Dead>(app, PostUpdate, true);
}
