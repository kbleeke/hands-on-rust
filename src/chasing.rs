use crate::{dijkstra::dijkstra, prelude::*};

pub fn chasing(
    mut commands: Commands,
    movers: Query<(Entity, &Point, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &Point, Option<&Player>), With<Health>>,
    player: Query<&Point, With<Player>>,
    map: Res<Map>,
    mut attacks: EventWriter<WantsToAttack>,
) {
    let player = player.get_single().unwrap();

    let dijkstra_map = dijkstra(&*map, *player);

    for (id, pos, fov) in movers.iter() {
        if !fov.visible_tiles.contains(player) {
            continue;
        }

        let dest = dijkstra_map.find_lowest_exit(pos, &*map);
        if let Some(dest) = dest {
            let distance = i32::abs(pos.x - player.x) + i32::abs(pos.y - player.y);
            let destination = if distance > 1 { dest } else { *player };

            let hit = positions
                .iter()
                .find_map(|(e, pos, player)| (pos == &destination).then(|| (e, player.is_some())));

            if let Some((hit, is_player)) = hit {
                if is_player {
                    info!(%distance, %destination, %player, %pos, "hit");
                    attacks.send(WantsToAttack {
                        attacker: id,
                        victim: hit,
                    })
                }
            } else {
                commands.entity(id).insert(WantsToMove { destination });
            }
        }
    }
}
