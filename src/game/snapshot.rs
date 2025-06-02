use bevy::prelude::*;

#[derive(Component)]
pub struct Snapshot<T>(Option<T>);

impl<T> Default for Snapshot<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Snapshot<T> {
    pub fn replace(&mut self, new: T) -> Option<T> {
        self.0.replace(new)
    }
}
