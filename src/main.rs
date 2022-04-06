use bevy::prelude::*;
use errio::{GameState, game::GamePlugin, menu::MenuPlugin, splash::SplashPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Game)
        .add_startup_system(setup_cameras)
        .add_plugin(MenuPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
