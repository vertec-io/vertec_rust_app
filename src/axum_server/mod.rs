use bevy::prelude::*;

pub struct AxumServerPlugin;

impl Plugin for AxumServerPlugin {
     fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }

}

fn setup() {
    println!("Setting up Axum Server")
}