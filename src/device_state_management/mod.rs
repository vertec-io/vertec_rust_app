use bevy::prelude::*;
pub struct DeviceStateManagementPlugin;

impl Plugin for DeviceStateManagementPlugin {
     fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }

}

fn setup() {
    println!("Setting up Device State Management")
}