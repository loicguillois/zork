use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    music_enabled: bool,
}

struct GreetTimer(Timer);

#[derive(Default)]
struct GameState {
    player_added: bool,
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Location, With<Player>>, mut commands: Commands, asset_server: Res<AssetServer>, mut game_state: ResMut<GameState>) {
    if timer.0.tick(time.delta()).just_finished() {
        if !game_state.player_added {
            for location in query.iter() {
                //println!("{}", location.0.x);
                commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("dungeon-tileset/NPC/NPC1.png"),
                    transform: Transform::from_translation(
                        Vec3::new(location.0.x, location.0.y, 100.0),
                    ),
                    ..Default::default()
                }).insert(Player);
            }
            game_state.player_added = true;
        }
    }
}

fn player_movement(
    mut player: Query<(&Player, &mut Transform)>,
    query: Query<&Location, With<Player>>
) {
    let start_x = 50.0;
    let start_y = 50.0;

    for (_player, mut transform) in player.iter_mut() {
        for location in query.iter() {
            transform.translation.y =  start_y + location.0.y;
            transform.translation.x = start_x + location.0.x;
        }
    }
}

fn camera_movement(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    query: Query<&Location, With<Player>>
) {
        for location in query.iter() {
            let mut camera_transform = camera_query.single_mut();
            camera_transform.translation.y = location.0.y;
            camera_transform.translation.x = location.0.x;
        }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Location(Vec2);

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name("Zork".to_string()))
        .insert(Location(Vec2::default()));
}

fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("music/fantascape.ogg");
    audio.play(music);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
    .insert_bundle(OrthographicCameraBundle::new_2d())
    .insert(Transform::from_xyz(0.0, 0.0, 1000.0));

    let width: usize = 10;
    let height: usize = 10;

    let step_x = 191;
    let step_y = 120;

    let mut map = vec![vec![0; width]; height];

    let floor_texture = asset_server.load("tiles/floor1.png");
    let floor_texture2 = asset_server.load("tiles/TileStairB.png");
    let wall_texture_v = asset_server.load("tiles/WallV_Tile31d.png");
    let wall_texture_h = asset_server.load("tiles/WallH_Tile22b.png");

    // Door
    map[4][7] = 1;

    // Floor
    for x in 0..width {
        for y in 0..height {
            let calculated_x = x as i32 * step_x + 130 * y as i32;
            let calculated_y = y as i32 * step_y - 60 * x as i32;
            commands.spawn_bundle(SpriteBundle {
                texture: if map[y][x] == 0 { floor_texture.clone() } else { floor_texture2.clone() },
                transform: Transform::from_translation(Vec3::new(calculated_x as f32, calculated_y as f32, 0.0)),
                ..Default::default()
            });
        }
    }

    // Wall top left
    for y in (0..height+1).rev() {
        let x = -1;
        let calculated_x = x as i32 * step_x + 130 * y as i32;
        let calculated_y = y as i32 * step_y - 58 * x as i32;
        commands.spawn_bundle(SpriteBundle {
            texture: wall_texture_v.clone(),
            transform: Transform::from_translation(Vec3::new(calculated_x as f32, calculated_y as f32, (height - y) as f32)),
            ..Default::default()
        });
    }

    let step_y_h = 60;

    // Wall top right
    for y in (height-1..height+width-1).rev() {
        let x = -1;
        let calculated_x = -x as i32 * step_x + 191 * y as i32 - 640;
        let calculated_y = ((height+width) as i32 * step_y_h as i32) - (y as i32 * step_y_h + 300 * x as i32) + 290;
        commands.spawn_bundle(SpriteBundle {
            texture: wall_texture_h.clone(),
            transform: Transform::from_translation(Vec3::new(calculated_x as f32, calculated_y as f32, y as f32)),
            ..Default::default()
        });
    }

    // Wall bottom left
    for y in (0..width).rev() {
        let x = -1;
        let calculated_x = -x as i32 * step_x + 191 * y as i32 - 290;
        let calculated_y = - (y as i32 * step_y_h + 300 * x as i32) - (height as i32 * 60) + 290;
        println!("{:?} {:?}",y, calculated_y);
        commands.spawn_bundle(SpriteBundle {
            texture: wall_texture_h.clone(),
            transform: Transform::from_translation(Vec3::new(calculated_x as f32, calculated_y as f32, (y+1 * 200) as f32)),
            ..Default::default()
        });
    }

    // Wall bottom right
    for y in (0..width).rev() {
        let x = -1;
        let calculated_x = x as i32 * step_x + 130 * y as i32 + 1980;
        let calculated_y = y as i32 * step_y - 60 * x as i32 - 540;
        commands.spawn_bundle(SpriteBundle {
            texture: wall_texture_v.clone(),
            transform: Transform::from_translation(Vec3::new(calculated_x as f32, calculated_y as f32, ((height+width)*10-y) as f32)),
            ..Default::default()
        });
    }
}

fn keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>, mut query: Query<(&Player, &mut Location)>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event.scan_code);

        for (_player, mut location) in query.iter_mut() {
            
            let speed = 30.0;

            if event.scan_code == 124 {
                location.0.x += speed;
            } else if event.scan_code == 123 {
                location.0.x -= speed;
            } else if event.scan_code == 126 {
                location.0.y += speed;
            } else if event.scan_code == 125 {
                location.0.y -= speed;
            }
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let music_enabled = &settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()["music_enabled"];

        app.insert_resource(GreetTimer(Timer::from_seconds(0.5, true)));

        if music_enabled == "true" {
            app.add_startup_system(setup_music);
        }

        app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .add_startup_system(add_player)
            .add_startup_system(setup)
            .init_resource::<GameState>()
            .add_system(keyboard_event_system)
            .add_system(greet_people)
            .add_system(player_movement)
            .add_system(camera_movement);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
