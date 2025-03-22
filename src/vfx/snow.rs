use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn setup_snow(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
) {
    let w = ExprWriter::new();

    let dir = w
        .rand(VectorType::VEC3F)
        .mul(w.lit(Vec3::ONE * 2.0))
        .sub(w.lit(Vec3::ONE));

    let dir_x = dir.clone().mul(w.lit(Vec3::new(1.0, 0.0, 0.0)));
    let dir_z = dir.mul(w.lit(Vec3::new(0.0, 0.0, 1.0)));

    let init_vel = SetAttributeModifier {
        attribute: Attribute::VELOCITY,
        value: w
            .lit(Vec3::new(0.0, -10.0, 0.0))
            .add(dir_z)
            .add(dir_x)
            .expr(),
    };

    let init_pos = SetPositionCircleModifier {
        center: w.lit(Vec3::ZERO).expr(),
        axis: w.lit(Vec3::Y).expr(),
        radius: w.lit(16.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_size = SetAttributeModifier {
        attribute: Attribute::SIZE,
        value: w.lit(0.5).expr(),
    };

    let init_age = SetAttributeModifier {
        attribute: Attribute::AGE,
        value: w.lit(0.0).expr(),
    };

    let init_lifetime = SetAttributeModifier {
        attribute: Attribute::LIFETIME,
        value: w.lit(2.0).expr(),
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

    let texture_slot = w.lit(0u32).expr();

    let mut module = w.finish();
    module.add_texture_slot("color");

    let effect = EffectAsset::new(32768, Spawner::rate(200.0.into()), module)
        .with_name("Snowfall")
        .init(init_pos)
        .init(init_vel)
        .init(init_size)
        .init(init_age)
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
