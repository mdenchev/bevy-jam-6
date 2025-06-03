use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}

pub trait Vec3Ext {
    fn to_vec2(self) -> Vec2;
}

impl Vec3Ext for Vec3 {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, -self.z)
    }
}
