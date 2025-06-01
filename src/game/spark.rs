#![allow(unreachable_code)]

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use crate::game::{constants::UNITS_PER_METER, health::Dead};

use super::{
    health::{AdjustHp, Health, MaxHealth},
    pause_controller::PausableSystems,
    snapshot::Snapshot,
};

#[auto_register_type]
#[derive(Resource, Reflect)]
pub struct SparkConfig {
    pub max_charge: f32,
    pub start_charge: f32,
    pub decay_per_second: f32,
    pub cost_per_m: f32,
    pub damage_dealt_per_second: f32,
    pub max_distance_jump_m: f32,
}

impl Default for SparkConfig {
    fn default() -> Self {
        Self {
            max_charge: 100.0,
            start_charge: 50.0,
            decay_per_second: 5.0,
            damage_dealt_per_second: 20.0,
            cost_per_m: 10.0,
            max_distance_jump_m: 50.0,
        }
    }
}

#[auto_name]
#[auto_register_type]
#[derive(Component, Reflect)]
#[require(Transform,Snapshot<GlobalTransform>)]
pub struct Spark;

#[auto_register_type]
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct SparkTarget;

/// Spark -> Zapping -> SparkTarget
/// Inserts ChildOf
#[auto_register_type]
#[derive(Component, Reflect)]
#[require(Spark = enforce_exists!(Spark))]
#[relationship(relationship_target=ZappedBy)]
pub struct Zapping(pub Entity);

/// SparkTarget -> ZappedBy -> Spark
#[auto_register_type]
#[derive(Component, Reflect)]
#[require(SparkTarget = enforce_exists!(SparkTarget))]
#[relationship_target(relationship=Zapping)]
pub struct ZappedBy(Entity);

#[auto_plugin(app=app)]
pub fn plugin(app: &mut App) {
    app.init_resource::<SparkConfig>();

    app.add_observer(SparkTarget::handle_inserted)
        .add_observer(Zapping::handle_inserted)
        .add_observer(ZappedBy::handle_inserted)
        .add_observer(Spark::handle_inserted);

    app.add_systems(
        Update,
        (
            spark::decay_health,
            spark::deal_dot,
            spark::apply_distance_cost,
        )
            .in_set(PausableSystems),
    );
}

impl SparkTarget {
    fn handle_inserted(tr: Trigger<OnInsert, Self>, mut commands: Commands) {
        fn handle_clicked(
            tr: Trigger<Pointer<Click>>,
            mut commands: Commands,
            sparks: Query<(Entity, &GlobalTransform), With<Spark>>,
            targets: Query<&GlobalTransform, With<SparkTarget>>,
            cfg: Res<SparkConfig>,
        ) {
            let tl_target = targets
                .get(tr.target())
                .expect("cause of picking")
                .translation();

            for (spark, tf_spark) in sparks {
                let dist = (tf_spark.translation() - tl_target).length() * UNITS_PER_METER;

                if dist > cfg.max_distance_jump_m {
                    continue;
                }

                commands.entity(spark).insert(Zapping(tr.target()));
            }
        }

        commands.entity(tr.target()).observe(handle_clicked);
    }
}

impl Zapping {
    /// Updates parenting
    fn handle_inserted(
        tr: Trigger<OnInsert, Self>,
        comp: Query<&Self, Added<Self>>,
        mut commands: Commands,
    ) {
        let comp = comp.get(tr.target()).expect("OnInsert broken");
        commands
            .entity(tr.target())
            .insert((ChildOf(comp.0), Transform::default())); // TODO better relative positioning
    }
}

impl ZappedBy {
    fn handle_inserted(tr: Trigger<OnInsert, Self>, mut commands: Commands) {
        fn handle_death(tr: Trigger<OnInsert, Dead>, mut commands: Commands) {
            commands.entity(tr.target()).remove::<ZappedBy>();
        }

        commands.entity(tr.target()).observe(handle_death);
    }
}

impl Spark {
    fn handle_inserted(tr: Trigger<OnInsert, Self>, mut commands: Commands, cfg: Res<SparkConfig>) {
        fn handle_death(tr: Trigger<OnInsert, Dead>, mut commands: Commands) {
            commands.entity(tr.target()).despawn();
        }

        commands
            .entity(tr.target())
            .insert((Health(cfg.start_charge), MaxHealth(cfg.max_charge)))
            .observe(handle_death);
    }
}

#[allow(clippy::module_inception)]
mod spark {
    use super::*;

    pub fn decay_health(
        sparks: Query<Entity, With<Spark>>,
        time: Res<Time>,
        mut adjust_hp_event: EventWriter<AdjustHp>,
        cfg: Res<SparkConfig>,
    ) {
        let damage_amount = time.delta_secs() * cfg.decay_per_second;
        adjust_hp_event.write_batch(
            sparks
                .iter()
                .map(|spark| AdjustHp::new(spark, -damage_amount)),
        );
    }

    pub fn deal_dot(
        targets: Query<Entity, (With<ZappedBy>, With<Health>, Without<Dead>)>,
        time: Res<Time>,
        mut adjust_hp_event: EventWriter<AdjustHp>,
        cfg: Res<SparkConfig>,
    ) {
        let damage_amount = time.delta_secs() * cfg.damage_dealt_per_second;

        adjust_hp_event.write_batch(
            targets
                .iter()
                .map(|target| AdjustHp::new(target, -damage_amount)),
        );
    }

    pub fn apply_distance_cost(
        mut sparks: Query<
            (Entity, &GlobalTransform, &mut Snapshot<GlobalTransform>),
            (With<Spark>, Changed<GlobalTransform>),
        >,
        mut adjust_hp_event: EventWriter<AdjustHp>,
        cfg: Res<SparkConfig>,
    ) {
        for (spark, gt_new, mut gt_snap) in sparks.iter_mut() {
            let Some(gt_old) = gt_snap.replace(*gt_new) else {
                continue;
            };

            let dist = (gt_new.translation() - gt_old.translation()).length() * UNITS_PER_METER;

            adjust_hp_event.write(AdjustHp::new(spark, -dist * cfg.cost_per_m));
        }
    }
}
