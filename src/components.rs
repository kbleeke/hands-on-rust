use bevy::utils::HashSet;

use crate::{prelude::*, Point};

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.current, self.max)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Component)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component, Debug)]
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Component)]
pub struct Player {
    pub level: i32,
}

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
        }
    }
}

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap {}

#[derive(Component)]
pub struct Carried(pub Entity);


#[derive(Component, Clone, Copy)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Weapon;