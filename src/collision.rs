use crate::{monster::Enemy, prelude::*, Coords};

pub fn collisions(
    mut commands: Commands,
    player: Query<&Coords, With<Player>>,
    enemies: Query<(Entity, &Coords), With<Enemy>>,
) {
    info!("colliding");
    let player = player.get_single().unwrap();
    info!("collide player {player:?}");

    enemies.for_each(|(e, coords)| {
        if coords == player {
            commands.entity(e).despawn_recursive();
        }
    });
}
