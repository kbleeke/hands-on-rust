use rand::{thread_rng, Rng};

use crate::{dijkstra::dijkstra, prelude::*};

pub struct EmptyArchitect;

impl MapArchitect for EmptyArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        let dijkstra_map = dijkstra(&mb.map, mb.player_start);
        mb.amulet_start = dijkstra_map.find_furthest(&mb.map, &mb.player_start);

        let mut rng = thread_rng();
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.gen_range(1..SCREEN_WIDTH),
                rng.gen_range(1..SCREEN_HEIGHT),
            ));
        }

        mb
    }
}
