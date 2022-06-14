#[macro_use]
extern crate tracing;

use clap::{App as ClapApp, Arg};
use mqtt2influx_core::{Executor, InfluxDbSink, MqttConnectionParameters, MqttEventSource, SinkTee};
use std::sync::Arc;

mod api;
mod conf;
mod utils;

const VERSION: &str = git_version::git_version!(args = ["--tags", "--always", "--abbrev=1", "--dirty=-modified"]);
const CONFIG_PATH_ARG: &str = "config";

#[actix_web::main]
async fn main() {
    let matches = ClapApp::new("MQTT2Influx")
        .version(VERSION)
        .about("Send MQTT events to Influx database")
        .author("Carlos Quintana <carlos@cquintana.dev>")
        .arg(
            Arg::with_name(CONFIG_PATH_ARG)
                .required(false)
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file"),
        )
        .get_matches();

    let config_path = matches.value_of(CONFIG_PATH_ARG);
    let configuration = conf::load(config_path).expect("Could not load the configuration");
    utils::setup_logging(&configuration.log_level);

    let source = MqttEventSource::new(
        MqttConnectionParameters {
            client_id: &configuration.client_id,
            host: &configuration.mqtt.host,
            port: configuration.mqtt.port,
        },
        configuration.subscriptions(),
    );

    let influx_sink = InfluxDbSink::new(configuration.influx.as_connection_parameters())
        .await
        .expect("Error creating InfluxDbSink");
    let api_sink = Arc::new(api::ApiState::default());
    let tee = SinkTee::new(Arc::new(influx_sink), api_sink.clone());

    tokio::spawn(async move {
        info!("Executor started");
        if let Err(e) = Executor::run(source, &tee).await {
            error!("[Executor] Fatal error: {}", e);
            std::process::exit(1);
        }
    });

    info!("Application started: [{}]", VERSION);
    if let Err(e) = api::run(configuration.port, api_sink).await {
        error!("[API] Fatal error: {}", e);
        std::process::exit(1);
    }
}
