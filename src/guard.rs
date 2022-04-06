use bevy::{prelude::*, core::FixedTimestep};
use crate::{GameState, Cell, game::Game, consts::{fonts, assets, MAP_SIZE_I, MAP_SIZE_J}, despawn_entities, pathfinding};
use rand::Rng;

#[derive(Default)]
struct GuardRoster {
    guards: Vec<Guard>,
    handle: Handle<Scene>,
}

#[derive(Default)]
struct Guard {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    current_path: Option<Vec<Cell>>,
    pp: usize,
    patrol_points: Vec<(usize, usize)>,
}

pub struct GuardPlugin;
impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GuardRoster>()
            .add_startup_system(setup_guards)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(patrol)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_guard)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
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
) {
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
                    //println!("Moving Up");
                } else if guard.i > cell.i {
                    rotation = std::f32::consts::FRAC_PI_2;
                    //println!("Moving Down");
                } else if guard.j < cell.j {
                    rotation = std::f32::consts::PI;
                    //println!("Moving Left");
                } else if guard.j > cell.j {
                    rotation = 0.0;
                    //println!("Moving Right");
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
    if (guards.guards.len() * 4 <= game.score as usize) {
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
                            color: Color::rgb(1.0, 0.0, 0.0),
                            intensity: 100.0,
                            range: 10.0,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 2.0, 0.0),
                        ..Default::default()
                    });
                    cell.spawn_scene(guards.handle.clone());
                })
                .id(),
        );
        guards.guards.push(guard);
    }
}
