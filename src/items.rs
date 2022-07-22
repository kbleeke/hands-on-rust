use crate::prelude::*;

pub struct UseItem {
    pub used_by: Entity,
    pub item: Entity,
}

pub fn use_healing(
    mut commands: Commands,
    mut events: EventReader<UseItem>,
    items: Query<&ProvidesHealing, With<Item>>,
    mut health_ents: Query<&mut Health>,
) {
    info!("checking items");
    for used_item in events.iter() {
        info!("use healing potion");
        let healing = items.get(used_item.item);
        if let Ok(healing) = healing {
            let mut e = health_ents.get_mut(used_item.used_by).unwrap();

            e.current += healing.amount;
            e.current = i32::min(e.current, e.max);
            commands.entity(used_item.item).despawn_recursive();
        }
    }
}

pub fn use_map(
    mut commands: Commands,
    mut events: EventReader<UseItem>,
    items: Query<&ProvidesDungeonMap, With<Item>>,
    mut map: ResMut<Map>,
) {
    for used_item in events.iter() {
        info!("use map");
        let reveal = items.get(used_item.item);
        if reveal.is_ok() {
            map.revealed.fill(true);
            commands.entity(used_item.item).despawn_recursive();
        }
    }
}
