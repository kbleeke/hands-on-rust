use crate::prelude::*;

mod enemy_hp;
mod inventory;
mod level;
mod names;
mod player_hp;

pub use inventory::InventoryNode;
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_hp::create_player_hp)
            .add_startup_system(inventory::create_inventory)
            .add_startup_system(level::create_level_text)
            .add_system(enemy_hp::create_monster_hp_bars)
            .add_system(names::create_entity_name)
            .add_system_to_stage(CoreStage::PostUpdate, names::mouse_hover)
            .add_system_to_stage(CoreStage::PostUpdate, names::scale_entity_name)
            .add_system_to_stage(CoreStage::PostUpdate, inventory::add_items)
            .add_system_to_stage(CoreStage::PostUpdate, inventory::item_numbers)
            .add_system_to_stage(CoreStage::PostUpdate, player_hp::update_player_health)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_hp::position_hp_bars)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_hp::position_hp_text)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_hp::update_hp_bar_size)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_hp::update_hp_bar_text)
            .add_system_to_stage(CoreStage::PostUpdate, enemy_hp::enemy_child_visibility)
            .add_system_to_stage(CoreStage::PostUpdate, level::update_level_text);
    }
}
