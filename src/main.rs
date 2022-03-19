use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    music_enabled: bool,
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Location, With<Player>>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if timer.0.tick(time.delta()).just_finished() {
        for location in query.iter() {
            //println!("{}", location.0.x);
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("dungeon-tileset/NPC/NPC1.png"),
                transform: Transform::from_translation(
                    Vec3::new(location.0.x, location.0.y, 15.0),
                ),
                ..Default::default()
            });
        }
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

fn setup(query: Query<&Location, With<Player>>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let floor_texture = asset_server.load("tiles/floor1.png");
    commands.spawn_bundle(SpriteBundle {
        texture: floor_texture.clone(),
        transform: Transform::from_translation(Vec3::new(-65.0, 0.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: floor_texture.clone(),
        transform: Transform::from_translation(Vec3::new(129.0, -61.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: floor_texture.clone(),
        transform: Transform::from_translation(Vec3::new(-194.0, -121.0, 0.0)),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: floor_texture.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, -181.0, 0.0)),
        ..Default::default()
    });
}

fn keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>, mut query: Query<(&Player, &mut Location)>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event.scan_code);

        for (player, mut location) in query.iter_mut() {
            
            if event.scan_code == 124 {
                location.0.x += 10.0;
            } else if event.scan_code == 123 {
                location.0.x -= 10.0;
            } else if event.scan_code == 126 {
                location.0.y += 10.0;
            } else if event.scan_code == 125 {
                location.0.y -= 10.0;
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

        app.add_startup_system(add_player)
            .add_startup_system(setup)
            .add_system(keyboard_event_system)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
