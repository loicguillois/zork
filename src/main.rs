use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Player>>) {
    for name in query.iter() {
        println!("Welcome {}!", name.0);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

fn add_player(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Name("Zork".to_string()));
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

