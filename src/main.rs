mod utils;
mod server;
mod commands;

use warp::Filter;
use utils::config::CONFIG;
use commands::{command_handler};
use std::net::{Ipv4Addr, SocketAddr};
use server::{request::CommandRequest};

pub fn parse_address(ip: &str) -> Ipv4Addr {
    ip.parse().unwrap_or_else(|_| Ipv4Addr::new(127, 0, 0, 1))
}

#[tokio::main]
async fn main() {
    let root = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|request: CommandRequest| {
            let handler = command_handler(&request.command);
            let response = handler(request.parameters);
            warp::reply::json(&response)
        });

    let ip_address = parse_address(&CONFIG.address);
    let socket_addr = SocketAddr::new(ip_address.into(), CONFIG.port);

    println!("Server listening on {}", socket_addr);
    warp::serve(root).run(socket_addr).await;
}
