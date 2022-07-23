use bevy::utils::HashSet;

use crate::prelude::*;

pub fn movement(mut commands: Commands, mut movers: Query<(Entity, &mut Point, &WantsToMove)>, map: Res<Map>) {
    info!("moving");

    let mut taken_squares = HashSet::new();

    for (id, mut coords, movement) in movers.iter_mut() {
        if taken_squares.contains(&movement.destination) {
            commands.entity(id).remove::<WantsToMove>();
            continue;
        }

        if map.can_enter_tile(movement.destination) {
            taken_squares.insert(movement.destination);
            *coords = movement.destination
        }

        // info!("move ent to {coords:?}");
        commands.entity(id).remove::<WantsToMove>();
    }
}
