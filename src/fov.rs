use crate::prelude::*;

pub fn fov(mut views: Query<(&Point, &mut FieldOfView), Changed<Point>>, map: Res<Map>) {
    for (pos, mut fov) in views.iter_mut() {
        fov.visible_tiles = fov_set2::fov_set(*pos, fov.radius, &*map);
        // fov.is_dirty = false;
    }
}

pub fn reveal_map(
    mut tiles: Query<(&mut Visibility, &mut Sprite, &Point), With<TileType>>,
    player: Query<(&FieldOfView, ChangeTrackers<FieldOfView>), With<Player>>,
    map: Res<Map>,
) {
    if !DARKNESS {
        return;
    }

    let (player, changed) = match player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if !changed.is_changed() && !map.is_changed() {
        return;
    }

    for (mut vis, mut sprite, coords) in tiles.iter_mut() {
        if player.visible_tiles.contains(coords) {
            // sprite.color = Color::default();
            vis.is_visible = true;
            sprite.color = Color::default();
        } else if map.revealed[map_idx(coords.x, coords.y)] {
            vis.is_visible = true;
            sprite.color = Color::GRAY;
        } else {
            // sprite.color = Color::BLACK;
            vis.is_visible = false;
        }
    }
}

pub fn darken_objects(
    mut entities: Query<(&mut Visibility, &Point), Or<(With<Enemy>, With<Item>)>>,
    player: Query<&FieldOfView, (With<Player>, Changed<FieldOfView>)>,
) {
    let player = match player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    for (mut sprite, coords) in entities.iter_mut() {
        if player.visible_tiles.contains(coords) {
            // sprite.color = Color::default();
            sprite.is_visible = true;
        } else {
            // sprite.color = Color::BLACK;
            sprite.is_visible = false;
        }
    }
}

mod fov_set2 {

    struct BresenhamLine {
        end: Point,
        current: Point,
        dx: i32,
        dy: i32,
        ix: i32,
        iy: i32,
        error: i32,
    }

    impl BresenhamLine {
        fn new(start: Point, end: Point) -> Self {
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let ix = dx.signum();
            let iy = dy.signum();
            let dx = dx.abs();
            let dy = -dy.abs();

            let error = dx + dy;

            Self {
                end,
                current: start,
                dx,
                dy,
                ix,
                iy,
                error,
            }
        }
    }

    impl Iterator for BresenhamLine {
        type Item = Point;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current == self.end {
                return None;
            }

            if 2 * self.error - self.dy > self.dx - 2 * self.error {
                self.error += self.dy;
                self.current.x += self.ix;
            } else {
                self.error += self.dx;
                self.current.y += self.iy;
            }

            Some(self.current)
        }
    }

    use bevy::utils::HashSet;
    use bracket_geometry::prelude::*;

    use crate::dijkstra::Map;

    pub fn fov_set(start: crate::Point, range: i32, fov_check: &dyn Map) -> HashSet<crate::Point> {
        let mut visible_points: HashSet<crate::Point> = HashSet::with_capacity(((range * 2) * (range * 2)) as usize);
        visible_points.insert(start);

        let start = Point::new(start.x, start.y);
        BresenhamCircleNoDiag::new(start, range).for_each(|point| {
            scan_fov_line(start, point, fov_check, &mut visible_points);
        });

        visible_points
    }

    /// Helper method to scan along a line.
    fn scan_fov_line(start: Point, end: Point, fov_check: &dyn Map, visible_points: &mut HashSet<crate::Point>) {
        let line = BresenhamLine::new(start, end);

        for target in line {
            if !fov_check.in_bounds(&crate::Point::new(target.x, target.y)) {
                // We're outside of the map
                break;
            }
            visible_points.insert(crate::Point::new(target.x, target.y));
            if fov_check.is_opaque(&crate::Point::new(target.x, target.y)) {
                // FoV is blocked
                break;
            }
        }
    }
}
