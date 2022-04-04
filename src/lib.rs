#![allow(unused)]
use bevy::prelude::*;
pub mod game;
pub mod consts;
pub mod levelgen;
pub mod splash;
pub mod menu;

#[derive(Default, Clone, Copy)]
pub struct Cell {
    open_sides: [bool; 4],
    doors: [bool; 4],
    height: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Game,
    Menu,
    Splash
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
