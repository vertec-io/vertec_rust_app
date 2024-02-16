use bevy::prelude::*;

pub struct SurrealDBPlugin;

impl Plugin for SurrealDBPlugin {
     fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }
}

fn setup() {
    println!("Setting up the SurrealDB database")
}