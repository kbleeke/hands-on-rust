use bevy::ecs::schedule::StateData;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{prelude::*, GameAssets};

pub enum GameOverReason {
    Lose,
    Win,
}

#[derive(Component)]
struct GameOverScreen;

fn create_gameover(
    mut commands: Commands,
    assets: Res<GameAssets>,
    tile_params: Res<TileParams>,
    reason: Res<GameOverReason>,
) {
    info!("game over");

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .insert(GameOverScreen)
        .with_children(|commands| match *reason {
            GameOverReason::Lose => losing_screen(commands, assets, tile_params),
            GameOverReason::Win => winning_screen(commands, assets, tile_params),
        });
}

fn losing_screen(commands: &mut ChildBuilder, assets: Res<GameAssets>, tile_params: Res<TileParams>) {
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Game Over",
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: 2.0 * tile_params.tile_size,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..default()
    });
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "You were slain by a monster",
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: 1.5 * tile_params.tile_size,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..default()
    });
}

fn winning_screen(commands: &mut ChildBuilder, assets: Res<GameAssets>, tile_params: Res<TileParams>) {
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Congratulations",
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: 2.0 * tile_params.tile_size,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..default()
    });
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "You collected the amulet",
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: 1.5 * tile_params.tile_size,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..default()
    });
}

pub struct GameOverPlugin<T> {
    state: T,
}

impl<T> GameOverPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for GameOverPlugin<T>
where
    T: StateData + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state, create_gameover);
    }
}
