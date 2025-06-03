use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_plugin(app=app)]
pub(crate) fn plugin(app: &mut App) {}

pub trait Vec2Ext {
    fn to_vec3(self) -> Vec3;
}

impl Vec2Ext for Vec2 {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, 0.0, -self.y)
    }
}
