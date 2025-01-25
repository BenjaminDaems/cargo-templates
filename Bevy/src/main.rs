use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(Startup, (setup))
        .run();
}

fn setup(mut commands: Commands) {
    println!("Hello world!")
}
