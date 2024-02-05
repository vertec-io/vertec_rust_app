use bevy::prelude::*;
pub struct DeviceInterfacePlugin;

impl Plugin for DeviceInterfacePlugin {
     fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }

}

fn setup() {
    println!("Setting up Device Interfaces")
}
