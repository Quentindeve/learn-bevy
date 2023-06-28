use bevy::prelude::*;

fn hello_world() {
    println!("Test");
}

#[derive(Component)]
struct Person {
    pub id: usize,
}

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_startup_system(hello_world)
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person { id: 0 }, Name("Ceci est un test".to_string())));
    commands.spawn((Person { id: 1 }, Name("Bonjour enculé".to_string())));
}

fn greet_people(query: Query<&Person, With<Name>>) {
    for name in &query {
        println!("hello {}", name.id);
    }
}