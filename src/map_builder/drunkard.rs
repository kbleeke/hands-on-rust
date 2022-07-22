use rand::{thread_rng, Rng};

use crate::{
    dijkstra::{dijkstra, Map as _},
    prelude::*,
};

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, map: &mut Map) {
        let mut drunkard_pos = *start;
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.index(&drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match thread_rng().gen_range(0..4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            };

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();
        let mut rng = thread_rng();

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, &mut mb.map);

        while mb.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR {
            self.drunkard(
                &Point::new(rng.gen_range(0..SCREEN_WIDTH), rng.gen_range(0..SCREEN_HEIGHT)),
                &mut mb.map,
            );

            let dijkstra_map = dijkstra(&mb.map, center);
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| **dist > 2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall)
        }

        mb.monster_spawns = mb.place_random_monsters(&center);
        mb.player_start = center;

        let dijkstra_map = dijkstra(&mb.map, center);
        mb.amulet_start = dijkstra_map.find_furthest(&mb.map, &center);

        mb
    }
}
