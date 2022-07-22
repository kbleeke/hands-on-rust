use rand::{thread_rng, Rng};

use crate::{
    dijkstra::{dijkstra, Map as _},
    prelude::*,
};

use super::distance2d_pythagoras;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, map: &mut Map) {
        for (idx, tile) in map.tiles.iter_mut().enumerate() {
            let coords = map_coords(idx);
            if coords.x == 0 || coords.x == SCREEN_WIDTH - 1 || coords.y == 0 || coords.y == SCREEN_HEIGHT - 1 {
                *tile = TileType::Wall;
                continue;
            }

            let roll = thread_rng().gen_range(0..100);
            if roll > 55 {
                *tile = TileType::Floor;
            } else {
                *tile = TileType::Wall;
            }
        }
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        ixy()
            .filter(|(ix, iy)| map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall)
            .count()
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);

                if neighbours > 4 || neighbours == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        let closest = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, distance2d_pythagoras(&center, &map.coords(idx))))
            .min_by(|(_, dist1), (_, dist2)| dist1.total_cmp(dist2))
            .map(|(idx, _)| idx)
            .unwrap();

        map.coords(closest)
    }
}

fn ixy() -> impl Iterator<Item = (i32, i32)> {
    (-1..=1)
        .flat_map(|iy| (-1..=1).map(move |ix| (ix, iy)))
        .filter(|(ix, iy)| *ix != 0 || *iy != 0)
}

impl MapArchitect for CellularAutomataArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();

        self.random_noise_map(&mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.place_random_monsters(&start);
        mb.player_start = start;

        let dijkstra_map = dijkstra(&mb.map, mb.player_start);
        mb.amulet_start = dijkstra_map.find_furthest(&mb.map, &mb.player_start);

        mb
    }
}
