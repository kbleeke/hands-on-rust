// mod collision;
mod chasing;
mod combat;
mod components;
mod constants;
mod coords;
mod dijkstra;
mod fov;
mod gameover;
mod hud;
mod items;
mod map;
mod map_builder;
mod movement;
mod player;
mod random_move;
mod spawner;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub const SCREEN_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
    pub const NUM_ROOMS: usize = 20;
    pub const NUM_MONSTERS: usize = 20;
    pub const CAMERA_SCALE: f32 = 2.0;
    pub const CAM_ON_PLAYER: bool = true;
    pub const HEALTH_BAR_SIZE: f32 = 16.;
    pub const DARKNESS: bool = true;
    pub const MAX_LEVEL: i32 = 2;

    // pub use crate::collision::*;
    pub use crate::chasing::*;
    pub use crate::combat::*;
    pub use crate::components::*;
    pub use crate::coords::*;
    pub use crate::hud::*;
    pub use crate::items::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::movement::*;
    pub use crate::player::*;
    pub use crate::random_move::*;
    pub use crate::spawner::*;
}

use bevy::{
    app::AppExit,
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    render::camera::Camera2d,
};
use gameover::{GameOverPlugin, GameOverReason};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem},
    state::{CurrentState, NextState},
};
use prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameMode {
    Generating,
    Playing,
    Regenerating,
    GameOver,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    AwaitingInput,
    Turn,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
enum GameStage {
    MovePlayer,
    MonsterAI,
    MoveMonster,
    MoveTiles,
}

impl GameStage {
    pub fn add(app: &mut App, entry: impl StageLabel) {
        use GameStage::*;
        let stages = [MovePlayer, MonsterAI, MoveMonster, MoveTiles];
        app.add_stage_after(entry, stages[0], SystemStage::parallel());
        for ele in stages.windows(2) {
            app.add_stage_after(ele[0], ele[1], SystemStage::parallel());
        }
    }
}

fn playing_and_input(game: Res<CurrentState<GameMode>>, turn: Res<CurrentState<TurnState>>) -> bool {
    game.0 == GameMode::Playing && turn.0 == TurnState::AwaitingInput
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(HudPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<GameAssets>()
        .init_resource::<TileParams>()
        .add_event::<WantsToAttack>()
        .add_event::<UseItem>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Map::default())
        .add_loopless_state(GameMode::Generating)
        .add_loopless_state(TurnState::AwaitingInput)
        .add_plugin(GameOverPlugin::new(GameMode::GameOver));
    GameStage::add(&mut app, CoreStage::Update);
    app.add_startup_system(create_camera)
        .add_system_to_stage(CoreStage::PreUpdate, update_tile_params)
        .add_system(generate_map.run_in_state(GameMode::Generating))
        .add_system(regenerate_map.run_in_state(GameMode::Regenerating))
        .add_system(player_input.run_if(playing_and_input))
        .add_system(update_revealed.run_in_state(GameMode::Playing))
        .add_system(exit)
        .add_system_set_to_stage(
            GameStage::MovePlayer,
            ConditionSet::new()
                .run_in_state(TurnState::Turn)
                .with_system(combat)
                .with_system(movement)
                .with_system(use_healing)
                .with_system(use_map)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::MonsterAI,
            ConditionSet::new()
                .run_in_state(TurnState::Turn)
                .with_system(random_move)
                .with_system(chasing)
                // .with_system(clear_events::<WantsToAttack>)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::MoveMonster,
            ConditionSet::new()
                .run_in_state(TurnState::Turn)
                .with_system(combat)
                .with_system(movement)
                .with_system(end_turn)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::MoveTiles,
            ConditionSet::new()
                .run_in_state(GameMode::Playing)
                .with_system(game_over)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::MoveTiles,
            ConditionSet::new()
                .run_in_state(GameMode::Playing)
                .label("move-tiles")
                .with_system(transform_entities)
                .with_system(resize_entities)
                .with_system(fov::fov)
                // .with_system(update_events::<WantsToAttack>)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::MoveTiles,
            ConditionSet::new()
                .run_in_state(GameMode::Playing)
                .after("move-tiles")
                .with_system(fov::reveal_map)
                .with_system(fov::darken_objects)
                // .with_system(update_events::<WantsToAttack>)
                .into(),
        )
        .run();
}

fn exit(input: Res<Input<KeyCode>>, mut event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        event.send(AppExit);
    }
}

fn game_over(
    mut commands: Commands,
    player: Query<(Entity, &Health, &Point), With<Player>>,
    maps: Query<Entity, (With<Carried>, With<ProvidesDungeonMap>)>,
    amulet: Query<&Point, With<AmuletOfYala>>,
    map: Res<Map>,
) {
    let (_id, health, pos) = player.get_single().unwrap();

    if map.tiles[map_idx(pos.x, pos.y)] == TileType::Exit {
        for m in maps.iter() {
            commands.entity(m).despawn_recursive();
        }

        commands.insert_resource(NextState(GameMode::Regenerating));
        return;
    }

    let amulet = amulet.get_single();
    if health.current <= 0 {
        commands.insert_resource(NextState(GameMode::GameOver));
        commands.insert_resource(GameOverReason::Lose);
    } else if amulet.map(|amulet| pos == amulet).unwrap_or_default() {
        commands.insert_resource(NextState(GameMode::GameOver));
        commands.insert_resource(GameOverReason::Win);
    }
}

fn end_turn(mut commands: Commands, turn: Res<CurrentState<TurnState>>) {
    let new_state = match turn.0 {
        TurnState::AwaitingInput => return,
        TurnState::Turn => TurnState::AwaitingInput,
    };
    info!("end turn to: {new_state:?}");

    // *turn = new_state;
    commands.insert_resource(NextState(new_state));
}

fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct GameAssets {
    font: Handle<Font>,
    #[allow(unused)]
    spritesheet: Handle<Image>,
    atlas: Handle<TextureAtlas>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();

        let font = assets.load("fonts/FiraSans-Bold.ttf");
        let spritesheet = assets.load("dungeonfont.png");
        let atlas = TextureAtlas::from_grid(spritesheet.clone(), Vec2::splat(32.), 16, 16);

        let mut atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let atlas = atlases.add(atlas);

        Self {
            font,
            spritesheet,
            atlas,
        }
    }
}

fn transform_entities(
    mut tiles: Query<(&mut Transform, &Point, Option<&Player>, ChangeTrackers<Point>), Without<Camera2d>>,
    tile_params: Res<TileParams>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let all_change = tile_params.is_changed();

    for (mut tf, coords, player, changed) in tiles.iter_mut() {
        if !(all_change || changed.is_changed()) {
            continue;
        }

        let x = tile_params.left + coords.x as f32 * tile_params.tile_size;
        let y = tile_params.bottom + coords.y as f32 * tile_params.tile_size;
        tf.translation.x = x;
        tf.translation.y = y;

        if player.is_some() && CAM_ON_PLAYER {
            let mut camera = camera.get_single_mut().unwrap();
            camera.translation.x = x;
            camera.translation.y = y;
        }
    }
}

fn resize_entities(mut entities: Query<&mut TextureAtlasSprite, With<TileSized>>, tile_params: Res<TileParams>) {
    for mut sprite in entities.iter_mut() {
        sprite.custom_size = Some(Vec2::splat(tile_params.tile_size));
    }
}
