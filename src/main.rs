use bevy::prelude::*;
mod app_ecs;
mod axum_server;
mod mqtt_integration;
mod device_interface;
mod device_state_management;
mod surrealdb_integration;

fn main() {
    println!("Starting the Application....");

    App::new()
        .add_plugins(
            (MinimalPlugins, 
             app_ecs::AppECSPlugin,
             mqtt_integration::MqttIntegrationPlugin, 
             axum_server::AxumServerPlugin,
             device_interface::DeviceInterfacePlugin,
             device_state_management::DeviceStateManagementPlugin,
             surrealdb_integration::SurrealDBPlugin
            )
        )
        .run();
}
