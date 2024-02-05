use bevy::prelude::*;

pub struct AppECSPlugin;

impl Plugin for AppECSPlugin {
     fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }
}

fn setup() {
    println!("Setting up Application Entity Component System")
}