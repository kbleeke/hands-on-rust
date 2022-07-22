use bevy::ecs::event::Events;

use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut events: ResMut<Events<WantsToAttack>>,
    attackers: Query<&Damage>,
    items: Query<(&Damage, &Carried)>,
    mut targets: Query<(&mut Health, Option<&Player>)>,
) {
    for attack in events.drain() {
        info!("attack");

        let mut damage = attackers.get(attack.attacker).copied().unwrap_or(Damage(1)).0;

        items.for_each(|(dmg, carried)| {
            if carried.0 == attack.attacker {
                damage += dmg.0;
            }
        });

        if let Ok((mut target, is_player)) = targets.get_mut(attack.victim) {
            target.current -= damage;
            if target.current < 1 && is_player.is_none() {
                commands.entity(attack.victim).despawn_recursive();
            }
        }
    }
}
