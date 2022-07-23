use arrayvec::ArrayVec;
use bevy::window::WindowResized;
use iyes_loopless::state::NextState;

use crate::{
    dijkstra::{self, Map as _},
    prelude::{themes::MapTheme, *},
    GameAssets, GameMode, Point,
};

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed: Vec<bool>,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed: vec![false; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && {
            let tt = self.tiles[map_idx(point.x, point.y)];
            tt == TileType::Floor || tt == TileType::Exit
        }
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

impl dijkstra::Map for Map {
    fn width(&self) -> usize {
        SCREEN_WIDTH as usize
    }

    fn height(&self) -> usize {
        SCREEN_HEIGHT as usize
    }

    fn index(&self, coords: &Point) -> usize {
        map_idx(coords.x, coords.y)
    }

    fn coords(&self, index: usize) -> Point {
        let x = index % SCREEN_WIDTH as usize;
        let y = index / SCREEN_WIDTH as usize;
        Point::new(x as i32, y as i32)
    }

    fn exits(&self, tile: &Point) -> arrayvec::ArrayVec<Point, 4> {
        let mut exits = ArrayVec::new();

        let deltas = [Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

        for delta in deltas {
            let location = tile + delta;
            if self.in_bounds(location) && self.can_enter_tile(location) {
                exits.push(location)
            }
        }

        exits
    }

    fn in_bounds(&self, coords: &Point) -> bool {
        self.in_bounds(*coords)
    }

    fn is_opaque(&self, coords: &Point) -> bool {
        self.tiles[map_idx(coords.x, coords.y)] == TileType::Wall
    }
}

#[derive(Component)]
pub struct TileSized;

#[derive(Default, Debug)]
pub struct TileParams {
    pub left: f32,
    pub bottom: f32,
    pub tile_size: f32,
    pub scale: f32,
}

pub fn update_tile_params(mut params: ResMut<TileParams>, mut resize: EventReader<WindowResized>) {
    for resize in resize.iter() {
        let mut width = resize.width;
        let mut height = resize.height - HEALTH_BAR_SIZE;

        let window_ratio = width / height;
        if window_ratio > SCREEN_RATIO {
            width = height * SCREEN_RATIO;
        } else {
            height = width / SCREEN_RATIO;
        }

        let x_step = width / SCREEN_WIDTH as f32;
        let y_step = height / SCREEN_HEIGHT as f32;

        let step = f32::min(x_step, y_step);

        let left = (width - step) / -2.0 * CAMERA_SCALE;
        let bottom = (height - step + HEALTH_BAR_SIZE) / -2.0 * CAMERA_SCALE;

        let tile_size = f32::min(x_step, y_step) * CAMERA_SCALE;

        *params = TileParams {
            left,
            bottom,
            tile_size,
            scale: CAMERA_SCALE,
        };
        info!(width, height, ?params, "resize");
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn map_coords(index: usize) -> Point {
    let x = index % SCREEN_WIDTH as usize;
    let y = index / SCREEN_WIDTH as usize;
    Point::new(x as i32, y as i32)
}

pub fn spawn_map(commands: &mut Commands, game_assets: &GameAssets, assets: &MapTheme, map: &Map) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            let ttype = map.tiles[idx];

            let texture = match ttype {
                TileType::Wall => assets.wall.clone(),
                TileType::Floor => assets.floor.clone(),
                TileType::Exit => assets.exit.clone(),
            };

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: game_assets.atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: texture,
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(ttype)
                .insert(Point::new(x, y))
                .insert(TileSized);
        }
    }
}

pub fn update_revealed(player: Query<&FieldOfView, (With<Player>, Changed<FieldOfView>)>, mut map: ResMut<Map>) {
    let player = match player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    for tile in player.visible_tiles.iter() {
        map.revealed[map_idx(tile.x, tile.y)] |= true;
    }
}

pub fn generate_map(mut commands: Commands, mut map: ResMut<Map>, assets: Res<GameAssets>) {
    let player_start = random_map(&mut commands, &mut *map, &assets, 0);

    spawn_player(&mut commands, &assets, &player_start);
    // spawn_amulet_of_yala(&mut commands, &assets, &mb.amulet_start);

    info!("map generated");
    commands.insert_resource(NextState(GameMode::Playing));
}

/// returns player start
fn random_map(commands: &mut Commands, map: &mut Map, assets: &GameAssets, level: i32) -> Point {
    let mb = MapBuilder::new();
    *map = mb.map;
    let theme = (mb.theme)();

    if level == MAX_LEVEL {
        spawn_amulet_of_yala(commands, assets, &mb.amulet_start);
    } else {
        let exit_idx = map.index(&mb.amulet_start);
        map.tiles[exit_idx] = TileType::Exit;
    }

    spawn_map(commands, &assets, &theme, &map);

    spawn_level(commands, assets, level, &mb.monster_spawns);
    mb.player_start
}

pub fn regenerate_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    assets: Res<GameAssets>,
    entities: Query<Entity, (With<Point>, Without<Player>)>,
    mut player: Query<(&mut Player, &mut Point, &mut FieldOfView)>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let (mut player, mut coords, mut fov) = player.single_mut();
    player.level += 1;
    fov.visible_tiles.clear();

    let player_pos = random_map(&mut commands, &mut map, &assets, player.level);
    *coords = player_pos;

    info!("map regenerated");
    commands.insert_resource(NextState(GameMode::Playing));
}
