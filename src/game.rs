use bevy::prelude::*;
use crate::{GameState, Cell, levelgen, consts::{assets, MAP_SIZE_I, MAP_SIZE_J}};

const RESET_POS: [f32; 3] = [
    MAP_SIZE_I as f32 / 2.0,
    0.0,
    MAP_SIZE_J as f32 / 2.0,
];

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                .with_system(setup_cameras)
                .with_system(setup)
                .with_system(setup_level)
                )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                )
            ;
    }
}

#[derive(Component)]
struct LevelTag;

#[derive(Default)]
struct Game {
    map: Vec<Cell>,
    player: Player,
    score: i32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    move_cooldown: Timer,
}

fn setup_cameras(
    mut commands: Commands,
    mut game: ResMut<Game>,
    ) {
    game.camera_should_focus = Vec3::from(RESET_POS);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(
                       -(MAP_SIZE_I as f32 / 2.0),
                       2.0 * MAP_SIZE_J as f32 / 3.0,
                       MAP_SIZE_J as f32 / 2.0 - 0.5,
                       )
            .looking_at(game.camera_is_focus, Vec3::Y),
            ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    ) {
    game.score = 0;
    game.player.i = MAP_SIZE_I / 2;
    game.player.j = MAP_SIZE_J / 2;
    game.player.move_cooldown = Timer::from_seconds(0.3, false);

    game.player.entity = Some(
        commands
        .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                                     game.player.i as f32,
                                     0.0,
                                     game.player.j as f32,
                                     ),
                                     rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                                     ..Default::default()
                },
                GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load(assets::ALIEN));
        })
        .id(),
        );
}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    ) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::Up) {
            if game.player.i < MAP_SIZE_I - 1 {
                game.player.i += 1;
            }
            rotation = -std::f32::consts::FRAC_PI_2;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.player.i > 0 {
                game.player.i -= 1;
            }
            rotation = std::f32::consts::FRAC_PI_2;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game.player.j < MAP_SIZE_J - 1 {
                game.player.j += 1;
            }
            rotation = std::f32::consts::PI;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.player.j > 0 {
                game.player.j -= 1;
            }
            rotation = 0.0;
            moved = true;
        }

        // move on the board
        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                                 game.player.i as f32,
                                 game.map[game.player.j * MAP_SIZE_I + game.player.i].height,
                                 game.player.j as f32,
                                 ),
                                 rotation: Quat::from_rotation_y(rotation),
                                 ..Default::default()
            };
        }
    }
}

fn setup_level(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    // Load assets
    let floor_scene: Handle<Scene> = asset_server.load(assets::FLOOR);
    let wall_scene: Handle<Scene> = asset_server.load(assets::WALL);
    let door_scene: Handle<Scene> = asset_server.load(assets::DOOR);
    //
    let map: Vec<Cell> = levelgen::generate_level(MAP_SIZE_I, MAP_SIZE_J, 10, 3);
    for j in 0..MAP_SIZE_J {
        for i in 0..MAP_SIZE_I {
            let cell: &Cell = &map[MAP_SIZE_I * j + i];
            // Spawn floor
            commands.spawn_bundle((
                    Transform {
                        translation: Vec3::new(i as f32, cell.height, j as f32),
                        rotation: Quat::from_rotation_y(0.0_f32.to_radians()),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                    },
                    GlobalTransform::identity(),
                    ))
                .insert(LevelTag)
                .with_children(|parent| {
                    parent.spawn_scene(floor_scene.clone());
                });
            // Spawn walls
            if i == 0 {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(270.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(wall_scene.clone());
                    });
            }
            if j == 0 {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(180.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(wall_scene.clone());
                    });
            }
            if !cell.open_sides[2] {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(0.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(wall_scene.clone());
                    });
            } else if cell.doors[2] {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(0.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(door_scene.clone());
                    });
            }
            if !cell.open_sides[1] {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(wall_scene.clone());
                    });
            } else if cell.doors[1] {
                commands.spawn_bundle((
                        Transform {
                            translation: Vec3::new(i as f32, 0.0, j as f32),
                            rotation: Quat::from_rotation_y(90.0_f32.to_radians()),
                            scale: Vec3::new(1.0, 1.0, 1.0),
                        },
                        GlobalTransform::identity(),
                        ))
                    .insert(LevelTag)
                    .with_children(|parent| {
                        parent.spawn_scene(door_scene.clone());
                    });
            }
        }
    }
    game.map = map;
}
