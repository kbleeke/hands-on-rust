use crate::{dijkstra::dijkstra, prelude::*};

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();

        mb.fill(TileType::Wall);
        mb.build_random_rooms();
        mb.build_corridors();
        mb.player_start = center(&mb.rooms[0]);

        let dijkstra_map = dijkstra(&mb.map, mb.player_start);
        mb.amulet_start = dijkstra_map.find_furthest(&mb.map, &mb.player_start);

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(center(room));
        }

        mb
    }
}
