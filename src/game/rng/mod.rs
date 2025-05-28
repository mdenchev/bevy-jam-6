pub mod global;

use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

pub type PRNG = WyRand;
pub const SEED_LEN: usize = 8;
pub type Seed = [u8; SEED_LEN];
pub const ZERO_SEED: Seed = [0; SEED_LEN];

pub struct RngPlugin;

impl Plugin for RngPlugin {
    #[auto_plugin(app=app)]
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<PRNG>::with_seed(ZERO_SEED));
        app.add_plugins(global::plugin);
    }
}
