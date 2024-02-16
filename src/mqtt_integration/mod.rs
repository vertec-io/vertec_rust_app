use dotenv::dotenv;
use std::env;
use std::str;
use std::process::Command;
use bevy::prelude::*;
use rumqttc::{MqttOptions, Client as MqttClient, Transport};

#[derive(Component)]
pub struct MqttConfig {
    pub client: MqttClient,
    pub host: String,
    pub port: u16
}

pub struct MqttIntegrationPlugin;

impl Plugin for MqttIntegrationPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands) {
    
    dotenv().ok();

    let mqtt_broker_url = env::var("MQTT_BROKER_URL").expect("MQTT_BROKER_URL not set");
    let mqtt_host;
    let mqtt_port;

    if mqtt_broker_url == "local" {
        println!("Setting up local MQTT Broker");
        mqtt_host = env::var("LOCAL_MQTT_HOST").expect("LOCAL_MQTT_HOST not set");
        mqtt_port = env::var("LOCAL_MQTT_PORT").expect("LOCAL_MQTT_PORT not set").parse::<u16>().expect("Invalid LOCAL_MQTT_PORT");

        //Check if the EMQX broker Docker container is already running
        let output = Command::new("docker")
                    .args(&["ps", "--filter", "name=emqx", "--format", "{{.Names}}"])
                    .output()
                    .expect("Failed to execute docker command");

        let running_containers = str::from_utf8(&output.stdout).expect("Could not locate the docker running containers");
        
        if !running_containers.contains("emqx") {
            //Start the EMQX Docker container if it's not already running
            Command::new("docker")
                .args(&[
                    "run", "-d", "--name", "emqx",
                    "-p", "1883:1883",
                    "-p", "8083:8083",
                    "-p", "8084:8084",
                    "-p", "8883:8883",
                    "-p", "18083:18083",
                    "emqx/emqx:5.5.0"
                ])
                .spawn()
                .expect("Failed to start EMQX Docker instance");
        }else{
            // TODO: I'd like for this line to display the host and port that was found running
            println!("Found running EMQX: {}", &running_containers);
        }
        
    } else if mqtt_broker_url == "none" {
        println!("MQTT broker disabled");
        return
    } else {
        println!("Connecting to remote MQTT Broker");
        mqtt_host = env::var("REMOTE_MQTT_HOST").expect("REMOTE_MQTT_HOST not set");
        mqtt_port = env::var("REMOTE_MQTT_PORT").expect("REMOTE_MQTT_PORT not set").parse::<u16>().expect("Invalid REMOTE_MQTT_PORT");

    }

    //Initialize MQTT client options
    println!("Setting up MQTT Client");
    let mut mqtt_options = MqttOptions::new("rust_mqtt_client", &mqtt_host, mqtt_port.clone());
    mqtt_options.set_transport(Transport::Tcp);
    
    //Create the MQTT Client
    let (client, _) = MqttClient::new(mqtt_options, 10);
    // TODO: I need to add something here to check the connection to the broker and verify there is a connection
    commands.spawn(
        MqttConfig{
            client,
            host: mqtt_host,
            port: mqtt_port
        }
    );
}