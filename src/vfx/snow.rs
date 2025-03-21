use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn setup_snow(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
) {
    let w = ExprWriter::new();

    let prop_dir = w.add_property("dir", Vec3::new(0.0, -1.0, 0.0).into());
    let prop_vel = w.add_property("vel", Vec3::new(3.0, 3.0, 3.0).into());

    let init_vel = SetAttributeModifier {
        attribute: Attribute::VELOCITY,
        value: w.prop(prop_dir).mul(w.prop(prop_vel)).expr(),
    };

    let init_pos = SetPositionCircleModifier {
        center: w.lit(Vec3::ZERO).expr(),
        axis: w.lit(Vec3::Y).expr(),
        radius: w.lit(16.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_lifetime = SetAttributeModifier {
        attribute: Attribute::LIFETIME,
        value: w.lit(5.0).expr(),
    };

    let init_color = SetAttributeModifier {
        attribute: Attribute::COLOR,
        value: w
            .lit(LinearRgba::new(0.95, 0.95, 0.95, 1.0).as_u32())
            .expr(),
    };

    let init_sprite_index = SetAttributeModifier {
        attribute: Attribute::SPRITE_INDEX,
        value: w
            .rand(ScalarType::Float)
            .mul(w.lit(16.0))
            .floor()
            .cast(ScalarType::Int)
            .expr(),
    };

    // Use the F32_0 attribute as a per-particle rotation value, initialized on
    // spawn and constant after. The rotation angle is in radians, here randomly
    // selected in [0:2*PI].
    let rotation = (w.rand(ScalarType::Float) * w.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);

    let texture_slot = w.lit(0u32).expr();
    let rotation_attr = w.attr(Attribute::F32_0).expr();

    let mut module = w.finish();
    module.add_texture_slot("color");

    let effect = EffectAsset::new(32768, Spawner::rate(50.0.into()), module)
        .with_name("Snowfall")
        .init(init_pos)
        .init(init_rotation)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_color)
        .init(init_sprite_index)
        .render(ParticleTextureModifier {
            texture_slot,
            sample_mapping: ImageSampleMapping::Modulate,
        })
        .render(FlipbookModifier {
            sprite_grid_size: UVec2::new(4, 4),
        });

    let effect_handle = effects.add(effect);
    let texture_handle: Handle<Image> = asset_server.load("textures/snow_sheet_01.png");

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
            ..default()
        })
        .insert(EffectMaterial {
            images: vec![texture_handle],
        });
}
