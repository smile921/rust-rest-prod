use clap::{ArgMatches, Command};
use crate::settings::{Settings, self};

use axum::{Router, ServiceExt};
use std::net::{IpAddr,Ipv4Addr, SocketAddr};

pub fn configure() -> Command {
    Command::new("serve").about("satrt the server!")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("serve") {
        println!("Server is up!");
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

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
            let addr =  SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
            let routes = Router::new();
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener,routes.into_make_service())
                .await?;
            Ok::<(),anyhow::Error>(())




            
        })?;
        std::process::exit(0);
}