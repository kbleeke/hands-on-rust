use std::collections::VecDeque;

use arrayvec::ArrayVec;
use rand::{prelude::SliceRandom, thread_rng};

use crate::prelude::Point;

pub trait Map {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn index(&self, coords: &Point) -> usize;
    fn coords(&self, index: usize) -> Point;
    fn exits(&self, tile: &Point) -> ArrayVec<Point, 4>;

    fn in_bounds(&self, coords: &Point) -> bool;
    fn is_opaque(&self, coords: &Point) -> bool;
}

pub struct DistanceMap {
    pub map: Vec<f32>,
}

impl DistanceMap {
    pub fn find_lowest_exit(&self, position: &Point, map: &dyn Map) -> Option<Point> {
        let mut exits = map.exits(position);
        exits.shuffle(&mut thread_rng());

        exits.into_iter().min_by(|a, b| {
            let a = self.map[map.index(a)];
            let b = self.map[map.index(b)];

            a.total_cmp(&b)
        })
    }

    pub fn find_furthest(&self, map: &dyn Map, from: &Point) -> Point {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, dist)| **dist < f32::MAX)
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| map.coords(i))
            .unwrap_or(*from)
    }
}

pub fn dijkstra(map: &dyn Map, start: Point) -> DistanceMap {
    let size = map.width() * map.height();
    let mut dm = DistanceMap {
        map: vec![f32::MAX; size],
    };

    let mut open_list: VecDeque<(Point, f32)> = VecDeque::with_capacity(size);

    open_list.push_back((start, 0.0));

    while let Some((tile, depth)) = open_list.pop_front() {
        let exits = map.exits(&tile);
        for new_idx in exits {
            let add_depth = 1.0;
            let new_depth = depth + add_depth;
            let prev_depth = dm.map[map.index(&new_idx)];
            if new_depth >= prev_depth {
                continue;
            }
            if new_depth >= 1024. {
                continue;
            }
            dm.map[map.index(&new_idx)] = new_depth;
            open_list.push_back((new_idx, new_depth));
        }
    }

    dm
}
