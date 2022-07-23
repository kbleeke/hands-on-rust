use bevy::utils::HashSet;
use rand::{thread_rng, Rng};

use crate::{dijkstra::Map as _, prelude::*, Point};

use self::{prefab::apply_prefab, themes::MapTheme};

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect<i32>>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn Fn() -> MapTheme>,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            map: Default::default(),
            rooms: Default::default(),
            monster_spawns: Default::default(),
            player_start: Default::default(),
            amulet_start: Default::default(),
            theme: Box::new(themes::dungeon_theme),
        }
    }
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile)
    }

    fn build_random_rooms(&mut self) {
        let mut rng = thread_rng();

        while self.rooms.len() < NUM_ROOMS {
            let left = rng.gen_range(1..SCREEN_WIDTH - 10);
            let bottom = rng.gen_range(1..SCREEN_HEIGHT - 10);

            let width = rng.gen_range(2..10);
            let height = rng.gen_range(2..10);

            let room = Rect {
                left,
                bottom,
                right: left + width,
                top: bottom + height,
            };

            let mut overlap = false;
            for r in self.rooms.iter() {
                if intersect(r, &room) {
                    overlap = true;
                }
            }

            if !overlap {
                for y in room.bottom..room.top {
                    for x in room.left..room.right {
                        if x > 0 && x < SCREEN_WIDTH && y > 0 && y < SCREEN_HEIGHT {
                            let idx = map_idx(x, y);
                            self.map.tiles[idx] = TileType::Floor
                        }
                    }
                }
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in i32::min(y1, y2)..=i32::max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in i32::min(x1, x2)..=i32::max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = thread_rng();

        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| center(a).x.cmp(&center(b).x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = center(&rooms[i - 1]);
            let new = center(room);

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn new() -> Self {
        // let mut architect = RoomsArchitect {};
        // let mut architect = automata::CellularAutomataArchitect {};
        // let mut architect = drunkard::DrunkardsWalkArchitect {};

        let mut architect: Box<dyn MapArchitect> = match thread_rng().gen_range(0..3) {
            0 => Box::new(drunkard::DrunkardsWalkArchitect {}),
            1 => Box::new(automata::CellularAutomataArchitect {}),
            _ => Box::new(rooms::RoomsArchitect {}),
        };

        let mut mb = architect.build();
        apply_prefab(&mut mb);

        mb.theme = match thread_rng().gen_range(0..2) {
            0 => Box::new(themes::dungeon_theme),
            _ => Box::new(themes::forest_theme),
        };

        mb
    }

    fn place_random_monsters(&self, start: &Point) -> Vec<Point> {
        let mut spawnable_tiles: Vec<_> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| **t == TileType::Floor && distance2d_pythagoras(start, &self.map.coords(*idx)) > 10.0)
            .map(|(idx, _)| self.map.coords(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_idx = thread_rng().gen_range(0..spawnable_tiles.len());
            spawns.push(spawnable_tiles[target_idx]);
            spawnable_tiles.remove(target_idx);
        }

        spawns
    }
}

fn intersect(a: &Rect<i32>, b: &Rect<i32>) -> bool {
    a.left < b.right && a.right > b.left && a.bottom < b.top && a.top > b.bottom
}

pub fn center(room: &Rect<i32>) -> Point {
    let x = (room.left + room.right) / 2;
    let y = (room.bottom + room.top) / 2;

    Point::new(x, y)
}

pub trait MapArchitect {
    fn build(&mut self) -> MapBuilder;
}

fn distance2d_pythagoras(start: &Point, end: &Point) -> f32 {
    let dx = (i32::max(start.x, end.x) - i32::min(start.x, end.x)) as f32;
    let dy = (i32::max(start.y, end.y) - i32::min(start.y, end.y)) as f32;
    let dsq = (dx * dx) + (dy * dy);

    f32::sqrt(dsq)
}

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
pub mod themes;

pub trait RectExt {
    fn size(&self) -> i32;
    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Point);
    fn point_set(&self) -> HashSet<Point>;
}

impl RectExt for Rect<i32> {
    fn size(&self) -> i32 {
        (self.right - self.left) * (self.top - self.bottom)
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.bottom..self.top {
            for x in self.left..self.right {
                f(Point::new(x, y))
            }
        }
    }

    fn point_set(&self) -> HashSet<Point> {
        let mut set = HashSet::with_capacity(self.size() as usize);
        self.for_each(|p| {
            set.insert(p);
        });
        set
    }
}
