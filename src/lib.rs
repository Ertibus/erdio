#![allow(unused)]
use bevy::prelude::*;
pub mod game;
pub mod consts;
pub mod levelgen;
pub mod splash;
pub mod menu;
pub mod guard;
pub mod pathfinding;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cell {
    pub open_sides: [bool; 4],
    pub doors: [bool; 4],
    pub height: f32,
    pub i: usize,
    pub j: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Game,
    Menu,
    Splash,
    GameOver,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
