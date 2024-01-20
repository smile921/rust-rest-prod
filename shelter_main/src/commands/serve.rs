use clap::{ArgMatches, Command, value_parser, Arg};
use crate::settings::{Settings, self};

use axum::{Router, ServiceExt};
use std::net::{IpAddr,Ipv4Addr, SocketAddr};

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
        start_tokio(port,settings)?;
    }
    Ok(())
}

 fn start_tokio(port:u16, _settings:&Settings)-> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let addr =  SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            let routes = crate::api::configure();
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener,routes.into_make_service())
                .await?;
            Ok::<(),anyhow::Error>(())
            
        })?;
        std::process::exit(0);
}