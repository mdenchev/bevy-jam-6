use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin_module::auto_register_type;

#[auto_register_type]
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health(pub f32);

#[auto_register_type]
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Health = panic!("MaxHealth requires Health") as Health)]
pub struct MaxHealth(pub f32);

#[derive(Event, Debug)]
pub struct AdjustHp {
    pub target: Entity,
    pub amount: f32,
}

#[auto_register_type]
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Dead;

// Plugin

#[derive(Default)]
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_adjust_hp);
        app.add_event::<AdjustHp>();
    }
}

// Internals

fn handle_adjust_hp(
    mut commands: Commands,
    mut damage_reader: EventReader<AdjustHp>,
    mut health_query: Query<&mut Health, Without<Dead>>,
) {
    for AdjustHp { target, amount } in damage_reader.read() {
        let Ok(mut health) = health_query.get_mut(*target) else {
            continue;
        };
        health.0 -= amount;

        if health.0 <= 0.0 {
            commands.entity(*target).insert(Dead);
        }
    }
}
