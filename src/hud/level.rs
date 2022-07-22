pub use crate::prelude::*;
use crate::GameAssets;

#[derive(Component)]
pub struct LevelText;

pub fn create_level_text(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(HEALTH_BAR_SIZE),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|commands| {
            let style = TextStyle {
                font: assets.font.clone(),
                font_size: HEALTH_BAR_SIZE,
                color: Color::WHITE,
            };

            commands
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                style: style.clone(),
                                value: "Dungeon Level: ".into(),
                            },
                            TextSection {
                                style: style.clone(),
                                ..Default::default()
                            },
                        ],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Left,
                        },
                    },
                    ..Default::default()
                })
                .insert(LevelText);
        });
}

pub fn update_level_text(mut level_text: Query<&mut Text, With<LevelText>>, player: Query<&Player, Changed<Player>>) {
    let player = match player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    let mut level_text = level_text.single_mut();

    level_text.sections[1].value = player.level.to_string();
}
