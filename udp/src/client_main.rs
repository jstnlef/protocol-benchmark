mod shared;

use get_if_addrs::get_if_addrs;
use serde_derive::{Serialize, Deserialize};
use shared::{get_eth_addr, start};
use std::{
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH}
};

fn main() -> Result<(), io::Error> {
    let client_addr = "127.0.0.1:9000".parse().unwrap();
    println!("Binding to address: {:?}", client_addr);
    let server_addr = "udp://server:9100".parse().unwrap();
    Ok(start(client_addr, server_addr))
}

