use iyes_loopless::state::NextState;

use crate::{prelude::*, TurnState};

pub fn player_input(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    player: Query<(Entity, &Point), With<Player>>,
    enemies: Query<(Entity, &Point), With<Enemy>>,
    items: Query<(Entity, &Point, Option<&Weapon>), With<Item>>,
    carried: Query<Entity, (With<Carried>, With<Weapon>)>,
    inventory: Query<&Children, With<InventoryNode>>,
    mut attacks: EventWriter<WantsToAttack>,
    mut use_items: EventWriter<UseItem>,
) {
    if input.just_pressed(KeyCode::Space) {
        info!("player waiting");
        commands.insert_resource(NextState(TurnState::Turn));
        return;
    }

    if input.just_pressed(KeyCode::G) {
        let (id, p_pos) = player.single();

        let item = items.iter().find(|(_, i_pos, _)| *i_pos == p_pos);
        if let Some((item, _, is_weapon)) = item {
            info!("picking up item {item:?}");
            commands
                .entity(item)
                .remove_bundle::<SpriteSheetBundle>()
                .remove::<Point>()
                .insert(Carried(id))
                .despawn_descendants();

            if is_weapon.is_some() {
                for c in carried.iter() {
                    commands.entity(c).despawn_recursive();
                }
            }
        }

        return;
    }

    let number_keys = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
        KeyCode::Key0,
    ];
    for (i, key) in number_keys.iter().enumerate() {
        if input.just_pressed(*key) {
            let (player, _) = player.single();
            let inv = inventory.single();

            // 0 is heading
            if let Some(&item) = inv.get(i + 1) {
                info!("using item in slot {i}");
                use_items.send(UseItem { item, used_by: player })
            }

            commands.insert_resource(NextState(TurnState::Turn));
            return;
        }
    }

    let delta = if input.just_pressed(KeyCode::Left) {
        Point::new(-1, 0)
    } else if input.just_pressed(KeyCode::Right) {
        Point::new(1, 0)
    } else if input.just_pressed(KeyCode::Up) {
        Point::new(0, 1)
    } else if input.just_pressed(KeyCode::Down) {
        Point::new(0, -1)
    } else {
        return;
    };

    let (id, p) = player.get_single().unwrap();
    let destination = p + delta;
    info!("input player to {destination:?}");

    let hit_something = enemies
        .iter()
        .find_map(|(e, coords)| (coords == &destination).then(|| e));

    if let Some(hit_something) = hit_something {
        attacks.send(WantsToAttack {
            attacker: id,
            victim: hit_something,
        })
    } else {
        commands.entity(id).insert(WantsToMove { destination });
    }
    commands.insert_resource(NextState(TurnState::Turn));
}
