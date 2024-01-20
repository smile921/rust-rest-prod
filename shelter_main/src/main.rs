use clap::{Arg, Command};
use dotenv::dotenv;

use shelter_main::{commands, settings};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{Registry, layer::SubscriberExt};

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let key = "SHELTER__LOGGING__LOG_LEVEL";
    let val = dotenv::var(key).unwrap();
    println!("Hello, world! {}", val);
    let mut command = Command::new("Dog Shelter sanple application")
        .version("1.0")
        .author("Frere Jac")
        .about("A sample application to experiment with Rust-based microservice")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("configuration file location")
                .default_value("config.json"),
        );
    command = commands::configure(command);
    let mathces = command.get_matches();
    let config_location = mathces
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");
    let settings = settings::Settings::new(config_location, "SHELTER")?;

    // let subscriber = Registry::default()
    //         .with(LevelFilter::from_level(Level::DEBUG))
    //         .with(tracing_subscriber::fmt::Layer::default()
    //             .with_writer(std::io::stdout)
    //         );
    //     tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let _ = commands::handle(&mathces, &settings);
    println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing database url".to_string())
    );

    println!(
        "logging level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );
    Ok(())
}
