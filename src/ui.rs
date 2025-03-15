use bevy::{color::palettes::basic::*, prelude::*};

const BASE_GAP: f32 = 16.0;

#[derive(Clone)]
pub enum TerrainButtonType {
    Generate,
    SetResolution,
}

#[derive(Clone, Component)]
pub struct TerrainButton<'a> {
    pub button_type: TerrainButtonType,
    pub text: &'a str,
}

impl<'a> TerrainButton<'a> {
    pub fn on_click(&self) {
        match self.button_type {
            TerrainButtonType::Generate => println!("Generate clicked!"),
            TerrainButtonType::SetResolution => println!("Set Resolution clicked!"),
        }
    }
}

const TERRAIN_BUTTONS: [TerrainButton; 2] = [
    TerrainButton {
        button_type: TerrainButtonType::Generate,
        text: "Generate Terrain",
    },
    TerrainButton {
        button_type: TerrainButtonType::Generate,
        text: "Set Resolution",
    },
];

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut ui_root = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Column,
        padding: UiRect {
            left: Val::Px(64.0),
            right: Val::Px(64.0),
            top: Val::Px(64.0),
            bottom: Val::Px(64.0),
        },
        ..default()
    });

    ui_root.with_children(|parent| {
        TERRAIN_BUTTONS.to_vec().iter().for_each(|tb| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(BASE_GAP),
                        },
                        ..default()
                    },
                    BorderRadius::all(Val::Px(2.0)),
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_child((
                    Text::new(tb.text),
                    TextFont {
                        font: asset_server.load("fonts/debug/Roboto/static/Roboto-Regular.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.85, 0.85)),
                ));
        })
    });
}
