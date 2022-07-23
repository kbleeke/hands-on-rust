use std::fs::File;

use bevy::utils::HashSet;
use rand::{prelude::SliceRandom, thread_rng};
use ron::de::from_reader;
use serde::Deserialize;

use crate::{
    constants::{ITEM_DEPTH, MONSTER_DEPTH},
    prelude::*, GameAssets,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub entity_type: EntityType,
    pub level: HashSet<usize>,
    pub frequency: usize,
    pub name: String,
    pub glyph: char,
    #[serde(default)]
    pub provides: Vec<(String, i32)>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum EntityType {
    Enemy,
    Item,
}

impl EntityType {
    fn depth(&self) -> f32 {
        match self {
            EntityType::Enemy => MONSTER_DEPTH,
            EntityType::Item => ITEM_DEPTH,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("template file");
        from_reader(file).expect("load templates")
    }

    pub fn spawn_entities(&self, commands: &mut Commands, assets: &GameAssets, level: i32, spawn_points: &[Point]) {
        let available_entities: Vec<_> = self
            .entities
            .iter()
            .filter(|e| e.level.contains(&(level as usize)))
            .collect();

        for pt in spawn_points {
            if let Ok(entity) = available_entities.choose_weighted(&mut thread_rng(), |item| item.frequency) {
                self.spawn_entity(pt, entity, commands, assets);
            }
        }
    }

    fn spawn_entity(&self, pt: &Point, template: &Template, commands: &mut Commands, assets: &GameAssets) {
        let mut entity = commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., template.entity_type.depth()),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: template.glyph as usize,
                ..Default::default()
            },
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        });
        entity
            .insert(*pt)
            .insert(TileSized)
            .insert(Name::new(template.name.clone()));

        match template.entity_type {
            EntityType::Item => {
                entity.insert(Item);
            }
            EntityType::Enemy => {
                entity
                    .insert(Enemy)
                    .insert(FieldOfView::new(6))
                    .insert(ChasingPlayer)
                    .insert(Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    });
            }
        }

        for (provides, n) in &template.provides {
            match provides.as_str() {
                "Healing" => {
                    entity.insert(ProvidesHealing { amount: *n });
                }
                "MagicMap" => {
                    entity.insert(ProvidesDungeonMap {});
                }
                _ => warn!("We don't know how to provide {provides}"),
            }
        }

        if let Some(damage) = template.base_damage {
            entity.insert(Damage(damage));
            if template.entity_type == EntityType::Item {
                entity.insert(Weapon);
            }
        }
    }
}
