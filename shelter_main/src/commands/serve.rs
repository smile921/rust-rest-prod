use crate::{settings::Settings, state::ApplicationState};
use clap::{value_parser, Arg, ArgMatches, Command};
use opentelemetry::{global, logs::LogError, trace::TraceError, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::logs::Config;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::{
    metrics::MeterProvider, propagation::TraceContextPropagator, runtime, Resource,
};
use sea_orm::Database;
use tower_http::trace::TraceLayer;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::ServiceExt;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

pub fn configure() -> Command {
    Command::new("serve").about("satrt the server!").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        println!("Server is up!");
        let port = *matches.get_one("port").unwrap_or(&8080);
        start_tokio(port, settings)?;
    }
    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            global::set_text_map_propagator(TraceContextPropagator::new());

            // let otlp_endpoint = settings.tracing.otlp_endpoint.clone().unwrap_or("http://127.0.0.1:4317".to_string());

            // let tracer = init_tracer(&otlp_endpoint)?;

            // let subscriber = Registry::default()
            // .with(LevelFilter::from_level(Level::DEBUG));

            let subscriber =
                tracing_subscriber::registry().with(LevelFilter::from_level(Level::DEBUG));

            let telemetry_layer =
                if let Some(otlp_endpoint) = settings.tracing.otlp_endpoint.clone() {
                    let tracer = init_tracer(&otlp_endpoint)?;

                    let _meter_privider = init_metrics(&otlp_endpoint);
                    let _log_provider = init_logs(&otlp_endpoint);
                    Some(tracing_opentelemetry::layer().with_tracer(tracer))
                } else {
                    None
                };

            // let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            subscriber
                .with(telemetry_layer)
                .with(fmt::Layer::default())
                .init();

            let db_url = settings.database.url.clone().unwrap_or("".to_string());
            let db_conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            let state = Arc::new(ApplicationState::new(settings, db_conn)?);

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            let routes = crate::api::configure(state).layer(TraceLayer::new_for_http());
            tracing::info!("starting axum on port {}", port);
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, routes.into_make_service()).await?;
            Ok::<(), anyhow::Error>(())
        })?;
    std::process::exit(0);
}

fn init_tracer(otlp_endpoint: &str) -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "shelter-project",
            )])),
        )
        .install_batch(runtime::Tokio)
}

fn init_metrics(otlp_endpoint: &str) -> opentelemetry::metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: otlp_endpoint.to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "shelter-project",
        )]))
        .build()
}

fn init_logs(otlp_endpoint: &str) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "shelter-project",
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint.to_string()),
        )
        .install_batch(runtime::Tokio)
}
