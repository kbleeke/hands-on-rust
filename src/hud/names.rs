use bevy::{math::Vec3Swizzles, render::camera::Camera2d};

use crate::{constants::NAME_DEPTH, prelude::*, GameAssets};

#[derive(Component)]
pub struct HasNameText {
    text: Entity,
}

#[derive(Component)]
pub struct NameText;

pub fn create_entity_name(
    mut commands: Commands,
    monster: Query<(Entity, &Name), (Added<Name>, With<Point>)>,
    assets: Res<GameAssets>,
) {
    for (monster, name) in monster.iter() {
        let text = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    name,
                    TextStyle {
                        font: assets.font.clone(),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(0., 0., NAME_DEPTH),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(NameText)
            .id();

        commands.entity(monster).insert(HasNameText { text }).add_child(text);
    }
}

pub fn scale_entity_name(mut text: Query<&mut Text, With<NameText>>, tile_params: Res<TileParams>) {
    // let size = tile_params.tile_size;
    for mut text in text.iter_mut() {
        text.sections[0].style.font_size = tile_params.tile_size * 0.4;
    }
}
pub fn mouse_hover(
    mut mouse: EventReader<CursorMoved>,
    windows: Res<Windows>,
    entity: Query<(&Sprite, &Transform, &HasNameText, &Visibility), With<Name>>,
    camera: Query<(&Transform, ChangeTrackers<Transform>), With<Camera2d>>,
    mut name_node: Query<&mut Visibility, (With<NameText>, Without<Name>)>,
) {
    let (camera, changed) = camera.single();
    if changed.is_changed() {
        // reset
        name_node.for_each_mut(|mut vis| vis.is_visible = false);
    }

    for e in mouse.iter() {
        entity.for_each(|(sprite, tf, name_text, vs)| {
            let size = match sprite.custom_size {
                Some(s) => s,
                None => return,
            };
            let window = windows.get(e.id).unwrap();
            let pos = e.position - Vec2::new(window.width(), window.height()) / 2.0;

            let lb = tf.translation.xy() - size / 2.0 - camera.translation.xy();
            let rb = tf.translation.xy() + size / 2.0 - camera.translation.xy();

            // if vs.is_visible {
            //     info!(?lb, ?rb, ?pos, "hit?");
            // }

            let hit_x = lb.x <= pos.x && pos.x <= rb.x;
            let hit_y = lb.y <= pos.y && pos.y <= rb.y;

            let mut text_node = match name_node.get_mut(name_text.text) {
                Ok(n) => n,
                Err(_) => return,
            };
            if hit_x && hit_y && vs.is_visible {
                text_node.is_visible = true;
            } else {
                text_node.is_visible = false;
            }
        });
    }
}
