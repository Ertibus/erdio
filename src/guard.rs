use bevy::{prelude::*, core::FixedTimestep};
use crate::{GameState, Cell, game::Game, consts::{fonts, assets, MAP_SIZE_I, MAP_SIZE_J}, despawn_entities, pathfinding};
use rand::Rng;

#[derive(Default)]
struct GuardRoster {
    guards: Vec<Guard>,
    handle: Handle<Scene>,
}

const VISION: usize = 3;

#[derive(Default)]
struct Guard {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    rotation: usize,
    current_path: Option<Vec<Cell>>,
    pp: usize,
    patrol_points: Vec<(usize, usize)>,
}

pub struct GuardPlugin;
impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GuardRoster>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(patrol)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_guards)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_guard)
                    .with_system(lookout)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(despawn_entities::<GuardTag>)
            )
        ;
    }
}

fn setup_guards(
    mut commands: Commands,
    mut guards: ResMut<GuardRoster>,
    asset_server: Res<AssetServer>,
) {
    guards.guards = Vec::new();
    guards.handle = asset_server.load(assets::ASTRONAUTS[0]);
}

#[derive(Component)]
struct GuardTag;

fn patrol(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut guards: ResMut<GuardRoster>,
    mut transforms: Query<&mut Transform>,
    mut state: ResMut<State<GameState>>,
) {
    if *state.current() != GameState::Game {
        return;
    }
    for guard in guards.guards.iter_mut() {
        match &mut guard.current_path {
            None => {
                guard.pp = if guard.pp as i32 >= guard.patrol_points.len() as i32 - 1 { 0 } else { guard.pp + 1 };
                guard.current_path = pathfinding::find_path(
                    &game,
                    &game.map[guard.j * MAP_SIZE_I + guard.i],
                    &game.map[guard.patrol_points[guard.pp].1 * MAP_SIZE_I + guard.patrol_points[guard.pp].0],
                );
            },
            Some(path) => {
                let cell = path.pop().unwrap();
                let mut rotation = 0.0;
                if guard.i < cell.i {
                    rotation = -std::f32::consts::FRAC_PI_2;
                    guard.rotation = 1;
                } else if guard.i > cell.i {
                    rotation = std::f32::consts::FRAC_PI_2;
                    guard.rotation = 3;
                } else if guard.j < cell.j {
                    rotation = std::f32::consts::PI;
                    guard.rotation = 2;
                } else if guard.j > cell.j {
                    rotation = 0.0;
                    guard.rotation = 0;
                }
                guard.i = cell.i;
                guard.j = cell.j;


                *transforms.get_mut(guard.entity.unwrap()).unwrap() = Transform {
                    translation: Vec3::new(guard.i as f32, game.map[guard.j * MAP_SIZE_I + guard.i].height, guard.j as f32),
                    rotation: Quat::from_rotation_y(rotation),
                    ..Default::default()
                };
                if path.is_empty() {
                    guard.current_path = None;
                    continue;
                }
            },
        };
    }
}

fn spawn_guard(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut guards: ResMut<GuardRoster>,
){
    if (guards.guards.len() * 3 <= game.score as usize) {
        for i in (0..2) {
            let mut guard: Guard = Guard::default();
            let mut patrol: Vec<(usize, usize)> = Vec::new();
            for _ in (0..rand::thread_rng().gen_range(2..=4)) {
                let i: usize = rand::thread_rng().gen_range(0..MAP_SIZE_I);
                let j: usize = rand::thread_rng().gen_range(0..MAP_SIZE_J);
                patrol.push((i, j));
            }
            guard.patrol_points = patrol;

            let i: usize = rand::thread_rng().gen_range(0..MAP_SIZE_I);
            let j: usize = rand::thread_rng().gen_range(0..MAP_SIZE_J);
            guard.i = i;
            guard.j = j;

            guard.entity = Some(
                commands
                    .spawn_bundle((
                            Transform {
                                translation: Vec3::new(guard.i as f32, 0.0, guard.j as f32),
                                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                                ..Default::default()
                            },
                            GlobalTransform::identity(),
                    ))
                    .insert(GuardTag)
                    .with_children(|cell| {
                        cell.spawn_bundle(PointLightBundle {
                            point_light: PointLight {
                                color: Color::rgb(0.5, 0.0, 0.0),
                                intensity: 5.0,
                                range: 3.0,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.2, 0.0),
                            ..Default::default()
                        });
                        cell.spawn_scene(guards.handle.clone());
                    })
                    .id(),
            );
            guards.guards.push(guard);
        }
    }
}

fn lookout (
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut guards: ResMut<GuardRoster>,
    mut state: ResMut<State<GameState>>,
) {
    for guard in guards.guards.iter_mut() {
        match guard.rotation {
            // 0 -> j-
            0 => {
                for i in (0..=VISION) {
                    if (guard.j as i32 - i as i32) < 0 || !game.map[(guard.j - i) * MAP_SIZE_I + guard.i].open_sides[0] {
                        break;
                    }
                    if game.player.j == guard.j - i && game.player.i == guard.i {
                        let _ = state.overwrite_set(GameState::GameOver);
                        return;
                    }
                }
            },
            1 => {
                for i in (0..=VISION) {
                    if guard.i + i >= MAP_SIZE_I || !game.map[guard.j * MAP_SIZE_I + guard.i + i].open_sides[1] {
                        break;
                    }
                    if game.player.i == guard.i + i && game.player.j == guard.j{
                        let _ = state.overwrite_set(GameState::GameOver);
                        return;
                    }
                }
            },
            2 => {
                for i in (0..=VISION) {
                    if guard.j + i >= MAP_SIZE_J || !game.map[(guard.j + i) * MAP_SIZE_I + guard.i].open_sides[2] {
                        break;
                    }
                    if game.player.j == guard.j + i && game.player.i == guard.i {
                        let _ = state.overwrite_set(GameState::GameOver);
                        return;
                    }
                }
            },
            3 => {
                for i in (0..=VISION) {
                    if (guard.i as i32 - i as i32) < 0 || !game.map[guard.j * MAP_SIZE_I + guard.i - i].open_sides[3] {
                        break;
                    }
                    if game.player.i == guard.i - i && game.player.j == guard.j {
                        let _ = state.overwrite_set(GameState::GameOver);
                        return;
                    }
                }
            },
            _ => (),
        }
    }
}
