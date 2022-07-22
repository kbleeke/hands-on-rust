use bevy::sprite::Anchor;

use crate::{
    constants::{HP_BAR_DEPTH, HP_TEXT_DEPTH},
    prelude::*,
    GameAssets,
};

#[derive(Component)]
pub struct EnemyHealthBar;

#[derive(Component)]
pub struct EnemyHealthText;

#[derive(Component)]
pub struct HasHealthBar {
    bar: Entity,
    text: Entity,
}

pub fn create_monster_hp_bars(
    mut commands: Commands,
    monster: Query<(Entity, &Health), Added<Enemy>>,
    assets: Res<GameAssets>,
) {
    for (monster, health) in monster.iter() {
        let bar = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., HP_BAR_DEPTH),
                    ..default()
                },
                ..default()
            })
            .insert(EnemyHealthBar)
            .id();

        let ratio = health.current as f32 / health.max as f32;
        let text = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    health.to_string(),
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 32.,
                        font: assets.font.clone(),
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(0., 0., HP_TEXT_DEPTH),
                    scale: Vec3::new(ratio, 1., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(EnemyHealthText)
            .id();

        let mut monster = commands.entity(monster);
        monster.insert(HasHealthBar { bar, text });
        monster.push_children(&[bar, text]);
    }
}

pub fn update_hp_bar_size(
    mut bars: Query<&mut Transform, With<EnemyHealthBar>>,
    enemies: Query<(&Health, &HasHealthBar), (With<Enemy>, Changed<Health>)>,
) {
    for (health, ents) in enemies.iter() {
        let mut tf = match bars.get_mut(ents.bar) {
            Ok(bar) => bar,
            Err(_) => {
                warn!("bar not found");
                continue;
            }
        };

        let ratio = health.current as f32 / health.max as f32;
        tf.scale.x = ratio;
    }
}

pub fn update_hp_bar_text(
    mut bars: Query<&mut Text, With<EnemyHealthText>>,
    enemies: Query<(&Health, &HasHealthBar), (With<Enemy>, Changed<Health>)>,
) {
    for (health, ents) in enemies.iter() {
        let mut text = match bars.get_mut(ents.text) {
            Ok(bar) => bar,
            Err(_) => {
                warn!("bar not found");
                continue;
            }
        };

        text.sections[0].value = health.to_string();
    }
}

pub fn position_hp_bars(
    mut bars: Query<(&mut Sprite, &mut Transform), With<EnemyHealthBar>>,
    tile_params: Res<TileParams>,
) {
    let size = tile_params.tile_size;
    for (mut sprite, mut tf) in bars.iter_mut() {
        sprite.custom_size = Some(Vec2::new(size * 1.5, size * 0.1));
        tf.translation.y = -size * (3. / 4.);
        tf.translation.x = -(size * 0.5 * 1.5);
    }
}

pub fn position_hp_text(
    mut text: Query<(&mut Text, &mut Transform), With<EnemyHealthText>>,
    tile_params: Res<TileParams>,
) {
    let size = tile_params.tile_size;
    for (mut text, mut tf) in text.iter_mut() {
        tf.translation.y = -size * (3. / 4.);
        text.sections[0].style.font_size = tile_params.tile_size * 0.4;
    }
}

pub fn enemy_child_visibility(
    mut child_ents: Query<&mut Visibility, (Without<Enemy>, Or<(With<EnemyHealthText>, With<EnemyHealthBar>)>)>,
    enemies: Query<(&Visibility, &Children), (With<Enemy>, Changed<Visibility>)>,
) {
    for (vis, children) in enemies.iter() {
        for child in children.iter() {
            if let Ok(mut child) = child_ents.get_mut(*child) {
                child.is_visible = vis.is_visible;
            }
        }
    }
}
