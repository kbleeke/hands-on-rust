pub struct MapTheme {
    pub floor: usize,
    pub wall: usize,
    pub exit: usize,
}

pub fn dungeon_theme() -> MapTheme {
    MapTheme {
        floor: '.' as usize,
        wall: '#' as usize,
        exit: '>' as usize,
    }
}

pub fn forest_theme() -> MapTheme {
    MapTheme {
        floor:  ';' as usize,
        wall: '"' as usize,
        exit: '>' as usize,
    }
}
