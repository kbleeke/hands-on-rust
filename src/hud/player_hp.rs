use crate::{prelude::*, GameAssets};

#[derive(Component)]
pub struct HealthDisplay;

#[derive(Component)]
pub struct HealthBar;

pub fn create_player_hp(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(HEALTH_BAR_SIZE)),

                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::RED.into(),
            ..default()
        })
        .insert(HealthBar);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(HEALTH_BAR_SIZE)),
                position: Rect {
                    top: Val::Px(0.),
                    ..default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: HEALTH_BAR_SIZE,
                            color: Color::WHITE,
                        },
                        default(),
                    ),
                    ..default()
                })
                .insert(HealthDisplay);
        });
}

pub fn update_player_health(
    player: Query<&Health, With<Player>>,
    mut text: Query<&mut Text, With<HealthDisplay>>,
    mut bar: Query<&mut Style, With<HealthBar>>,
) {
    let player = match player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    let mut text = text.get_single_mut().unwrap();
    let mut bar = bar.get_single_mut().unwrap();

    text.sections[0].value = format!("{}/{}", player.current, player.max);
    bar.size.width = Val::Percent(player.current as f32 / player.max as f32 * 100.0);
}
