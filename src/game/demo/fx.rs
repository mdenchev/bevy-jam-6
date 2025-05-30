use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use bevy_hanabi::prelude::*;

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct FxAssets {
    #[dependency]
    pub light: Handle<EffectAsset>,
}

impl FromWorld for FxAssets {
    fn from_world(world: &mut World) -> Self {
        // Define a color gradient from red to transparent black
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.));
        gradient.add_key(1.0, Vec4::splat(0.));

        // Create a new expression module
        let writer = ExprWriter::new();

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(2.).expr(),
            dimension: ShapeDimension::Surface,
        };

        let init_vel = SetAttributeModifier::new(
            Attribute::VELOCITY,
            // Mix random spherical direction with strong upward bias for cone effect
            ((writer.rand(VectorType::VEC3F).normalized() * writer.lit(0.8)
                + writer.lit(Vec3::Y) * writer.lit(1.5))
            .normalized()
                * (writer.rand(ScalarType::Float) * writer.lit(50.0) + writer.lit(10.0)))
            .expr(),
        );

        let lifetime = writer.lit(2.).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let accel = writer.lit(Vec3::new(0., 5., 0.)).expr();
        let update_accel = AccelModifier::new(accel);

        // Create the effect asset
        let effect = EffectAsset::new(
            // Maximum number of particles alive at a time
            32768,
            SpawnerSettings::rate(500.0.into()),
            // Move the expression module into the asset
            writer.finish(),
        )
        .with_name("Light")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier::new(gradient));

        // Insert into the asset system
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        let effect_handle = effects.add(effect);
        Self {
            light: effect_handle,
        }
    }
}

#[auto_plugin(app=app)]
pub(super) fn plugin(app: &mut App) {}
