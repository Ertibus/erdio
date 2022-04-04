use bevy::prelude::*;

use crate::{despawn_entities, GameState};

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
            .add_system_set(SystemSet::on_exit(GameState::Splash).with_system(despawn_entities::<OnSplashScreen>));
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon: Handle<Image> = asset_server.load("textures/splash.png");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size { width: Val::Percent(100.0), height: Val::Percent(100.0) },
                ..Default::default()
            },
            color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        .insert(OnSplashScreen)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                    ..Default::default()
                },
                image: UiImage(icon),
                ..Default::default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, false)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}
