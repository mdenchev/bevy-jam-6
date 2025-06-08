use crate::game::pause_controller::Pause;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use std::marker::PhantomData;
use std::time::Duration;

#[derive(Component, Debug, Default, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct Stopwatch<T>
where
    T: TypePath + Send + Sync + 'static,
{
    duration: Duration,
    #[reflect(ignore)]
    _marker: PhantomData<T>,
}

impl<T> Stopwatch<T>
where
    T: TypePath + Send + Sync + 'static,
{
    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn secs(&self) -> f32 {
        self.duration.as_secs_f32()
    }
}

fn update<T>(time: Res<Time>, mut stopwatches: Query<(Entity, &mut Stopwatch<T>)>)
where
    T: TypePath + Send + Sync + 'static,
{
    for (entity, mut stopwatch) in stopwatches.iter_mut() {
        let delta = time.delta();
        let previous = stopwatch.duration;
        stopwatch.duration = stopwatch.duration.saturating_add(delta);
        if stopwatch.duration == previous {
            warn_once!("Stopwatch {entity} maxed");
        }
    }
}

pub fn register_stopwatch<T>(app: &mut App, schedule_label: impl ScheduleLabel, pausable: bool)
where
    T: TypePath + Send + Sync + 'static,
{
    app.register_type::<Stopwatch<T>>();
    if pausable {
        app.add_systems(
            schedule_label,
            update::<T>.run_if(not(in_state(Pause(true)))),
        );
    } else {
        app.add_systems(schedule_label, update::<T>);
    }
}

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}
