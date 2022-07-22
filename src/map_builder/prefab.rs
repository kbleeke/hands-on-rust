use rand::{thread_rng, Rng};

use crate::{
    dijkstra::{self, Map},
    prelude::*,
};

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#-M--#---
---#----#---
---######---
------------
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder) {
    let mut rng = thread_rng();
    let mut placement = None;

    let dijkstra = dijkstra::dijkstra(&mb.map, mb.player_start);

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let left = rng.gen_range(0..SCREEN_WIDTH - FORTRESS.1);
        let bottom = rng.gen_range(0..SCREEN_HEIGHT - FORTRESS.2);
        let dims = Rect {
            left,
            bottom,
            right: left + FORTRESS.1,
            top: bottom + FORTRESS.2,
        };

        let mut can_place = false;
        dims.for_each(|p| {
            let dist = dijkstra.map[mb.map.index(&p)];
            if dist < 2000.0 && dist > 20.0 && mb.amulet_start != p && mb.player_start != p {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dims.left, dims.bottom));
            let points = dims.point_set();
            mb.monster_spawns.retain(|p| !points.contains(p));
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        for (fy, line) in FORTRESS.0.lines().skip(1).enumerate() {
            for (fx, c) in line.chars().enumerate() {
                let x = placement.x + fx as i32;
                let y = placement.y + fy as i32;
                let i = map_idx(x, y);

                match c {
                    'M' => {
                        mb.map.tiles[i] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(x, y));
                    }
                    '-' => {
                        mb.map.tiles[i] = TileType::Floor;
                    }
                    '#' => {
                        mb.map.tiles[i] = TileType::Wall;
                    }
                    _ => {
                        warn!("No idea what to do with {c}");
                    }
                }
            }
        }
    }
}
