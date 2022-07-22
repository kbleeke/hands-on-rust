use crate::prelude::*;

pub struct MapTheme {
    pub floor: Handle<Image>,
    pub wall: Handle<Image>,
    pub exit: Handle<Image>,
}

pub fn dungeon_theme(server: &AssetServer) -> MapTheme {
    MapTheme {
        floor: server.load("tiles2/floor.png"),
        wall: server.load("tiles2/wall.png"),
        exit: server.load("tiles2/stairs.png"),
    }
}

pub fn forest_theme(server: &AssetServer) -> MapTheme {
    MapTheme {
        floor: server.load("tiles2/sand.png"),
        wall: server.load("tiles2/tree.png"),
        exit: server.load("tiles2/stairs.png"),
    }
}
