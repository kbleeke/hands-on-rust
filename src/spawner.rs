mod template;
use crate::{
    constants::{ITEM_DEPTH, PLAYER_DEPTH},
    prelude::*,
    GameAssets,
};

use self::template::Templates;

pub fn spawn_player(commands: &mut Commands, assets: &GameAssets, start: &Point) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., PLAYER_DEPTH),
                ..default()
            },
            ..default()
        })
        .insert(*start)
        .insert(Player { level: 0 })
        .insert(Health { current: 10, max: 10 })
        .insert(TileSized)
        .insert(FieldOfView::new(8))
        .insert(Damage(1));
}

pub fn spawn_amulet_of_yala(commands: &mut Commands, assets: &GameAssets, pos: &Point) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.amulet.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., ITEM_DEPTH),
                ..Default::default()
            },
            ..default()
        })
        .insert(*pos)
        .insert(TileSized)
        .insert(Name::new("Amulet of Yala"))
        .insert(Item)
        .insert(AmuletOfYala);
}

pub fn spawn_level(commands: &mut Commands, assets: &AssetServer, level: i32, spawn_points: &[Point]) {
    let template = Templates::load();
    template.spawn_entities(commands, assets, level, spawn_points)
}
