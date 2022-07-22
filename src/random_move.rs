use rand::{thread_rng, Rng};

use crate::prelude::*;

pub fn random_move(
    mut commands: Commands,
    movers: Query<(Entity, &Point), With<MovingRandomly>>,
    positions: Query<(Entity, &Point, Option<&Player>), With<Health>>,
    mut attacks: EventWriter<WantsToAttack>,
) {
    info!("moving monsters");
    let mut rng = thread_rng();

    for (id, entity) in movers.iter() {
        let walk = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };

        let destination = Point::new(entity.x + walk.x, entity.y + walk.y);

        let hit = positions
            .iter()
            .find_map(|(e, pos, player)| (pos == &destination).then(|| (e, player.is_some())));

        if let Some((hit, true)) = hit {
            attacks.send(WantsToAttack {
                attacker: id,
                victim: hit,
            })
        } else {
            commands.entity(id).insert(WantsToMove { destination });
        }
    }
}
