use bevy::prelude::*;

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Welcome {}!", name.0);
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

fn add_player(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Name("Zork".to_string()));
}

fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("music/fantascape.ogg");
    audio.play(music);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup_music)
            .add_startup_system(add_player)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

