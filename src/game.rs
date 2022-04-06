use bevy::{prelude::*, core::FixedTimestep};
use crate::{GameState, Cell, levelgen, guard::GuardPlugin, consts::{fonts, assets, MAP_SIZE_I, MAP_SIZE_J}, despawn_entities};
use rand::Rng;

const RESET_POS: [f32; 3] = [
    MAP_SIZE_I as f32 / 2.0,
    0.0,
    MAP_SIZE_J as f32 / 2.0,
];

const MOVE_DELAY: f32 = 0.3;
const CAMERA_OFFSET: [f32; 3] = [-5.0, 10.0, 1.0];

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_plugin(GuardPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                .with_system(setup_cameras)
                .with_system(setup)
                .with_system(setup_level)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                .with_system(move_player)
                .with_system(focus_camera)
                .with_system(rotate_bonus)
                .with_system(scoreboard_system)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                .with_system(despawn_entities::<LevelTag>)
            )
            .add_system_set(
                SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(spawn_bonus)
           )
            ;
    }
}

#[derive(Component)]
struct LevelTag;

#[derive(Default)]
pub struct Game {
    pub map: Vec<Cell>,
    pub player: Player,
    bonus: Bonus,
    pub score: i32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

#[derive(Default)]
struct Bonus {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    handle: Handle<Scene>,
}

#[derive(Default)]
pub struct Player {
    entity: Option<Entity>,
    pub i: usize,
    pub j: usize,
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
                       (MAP_SIZE_I as f32 / 2.0) + CAMERA_OFFSET[0],
                       CAMERA_OFFSET[1],
                       MAP_SIZE_J as f32 / 2.0 + CAMERA_OFFSET[2],
                       )
            .looking_at(game.camera_is_focus, Vec3::Y),
        ..Default::default()
    });
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    game.score = 0;
    game.player.i = MAP_SIZE_I / 2;
    game.player.j = MAP_SIZE_J / 2;
    game.player.move_cooldown = Timer::from_seconds(MOVE_DELAY, false);

    game.player.entity = Some(
        commands
            .spawn_bundle((
                    Transform {
                        translation: Vec3::new(game.player.i as f32, 0.0, game.player.j as f32),
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
    // Spawn lights
    let half_size: f32 = 4.0;
    commands.spawn_bundle(DirectionalLightBundle {
         transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 320.0,
            shadows_enabled: false,
            shadow_projection: OrthographicProjection {
                left: -half_size,
                right: half_size,
                bottom: -half_size,
                top: half_size,
                near: -10.0 * half_size,
                far: 10.0 * half_size,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // load the scene for the cake
    game.bonus.handle = asset_server.load(assets::BONUS);

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Score:",
            TextStyle {
                font: asset_server.load(fonts::MAIN_FONT),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
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
    let map: Vec<Cell> = levelgen::generate_level(MAP_SIZE_I, MAP_SIZE_J, 14, 3);
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

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if !game.player.move_cooldown.tick(time.delta()).finished() { return; }

    let mut moved = false;
    let mut rotation = 0.0;

    let player_pos = game.player.j * MAP_SIZE_I + game.player.i;

    if keyboard_input.pressed(KeyCode::Up) {
        if game.player.i < MAP_SIZE_I - 1 && game.map[player_pos].open_sides[1] {
            game.player.i += 1;
        }
        rotation = -std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        if game.player.i > 0 && game.map[player_pos].open_sides[3] {
            game.player.i -= 1;
        }
        rotation = std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        if game.player.j < MAP_SIZE_J - 1 && game.map[player_pos].open_sides[2] {
            game.player.j += 1;
        }
        rotation = std::f32::consts::PI;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        if game.player.j > 0 && game.map[player_pos].open_sides[0] {
            game.player.j -= 1;
        }
        rotation = 0.0;
        moved = true;
    }

    // move on the board
    if !moved { return; }

    println!("{}:{}", game.player.i, game.player.j);
    //println!("{:?}", game.map[game.player.j * MAP_SIZE_I + game.player.i]);
    game.player.move_cooldown.reset();
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new( game.player.i as f32, game.map[game.player.j * MAP_SIZE_I + game.player.i].height, game.player.j as f32),
        rotation: Quat::from_rotation_y(rotation),
        ..Default::default()
    };

    if let Some(entity) = game.bonus.entity {
        if game.player.i == game.bonus.i && game.player.j == game.bonus.j {
            game.score += 1;
            commands.entity(entity).despawn_recursive();
            game.bonus.entity = None;
        }
    }
}

// change the focus of the camera
fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: QuerySet<( QueryState<&mut Transform, With<Camera>>, QueryState<&Transform>,)>,
) {
    const SPEED: f32 = 2.0;
    // if there is both a player and a bonus, target the mid-point of them
    // otherwise, if there is only a player, target the player
    if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.q1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
    // otherwise, target the middle
    } else {
        game.camera_should_focus = Vec3::from(RESET_POS);
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for mut transform in transforms.q0().iter_mut() {
        transform.translation = game.camera_is_focus + Vec3::from_slice(&CAMERA_OFFSET);
    }
}

fn spawn_bonus(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
) {
    if *state.current() != GameState::Game || game.bonus.entity.is_some() {
        return;
    }
    loop {
        game.bonus.i = rand::thread_rng().gen_range(0..MAP_SIZE_I);
        game.bonus.j = rand::thread_rng().gen_range(0..MAP_SIZE_J);
        if game.bonus.i != game.player.i || game.bonus.j != game.player.j {
            break;
        }
    }
    game.bonus.entity = Some(
        commands
            .spawn_bundle((
                Transform {
                    translation: Vec3::new(
                        game.bonus.i as f32,
                        game.map[game.bonus.j * MAP_SIZE_I + game.bonus.i].height,
                        game.bonus.j as f32,
                    ),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|children| {
                children.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 100.0,
                        range: 10.0,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..Default::default()
                });
                children.spawn_scene(game.bonus.handle.clone());
            })
            .id(),
    );
}

// let the case turn on itself
fn rotate_bonus(game: Res<Game>, time: Res<Time>, mut transforms: Query<&mut Transform>) {
    if let Some(entity) = game.bonus.entity {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
            transform.scale = Vec3::splat( 1.0 + (game.score as f32 / 10.0 * time.seconds_since_startup().sin() as f32).abs(),);
        }
    }
}

// update the score displayed during the game
fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Intel collected: {}", game.score);
}

// restart the game when pressing spacebar
fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Game).unwrap();
    }
}
