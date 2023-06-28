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

#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_startup_system(hello_world)
            .insert_resource(GreetTimer(Timer::from_seconds(2f32, TimerMode::Repeating)))
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person { id: 0 }, Name("Ceci est un test".to_string())));
    commands.spawn((Person { id: 1 }, Name("Bonjour enculé".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Person, With<Name>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}", name.id);
        }
    }
}
