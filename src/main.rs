use bevy::prelude::*;
use erdio::{GameState, game::GamePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_plugin(GamePlugin)
        .run();
}
