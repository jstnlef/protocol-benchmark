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
    let addr = get_eth_addr()?;
    println!("Binding to address: {:?}", addr);
    let client_addr: SocketAddr = (addr.clone(), 9000).into();
    let server_addr: SocketAddr = (addr, 9900).into();
    Ok(start(server_addr, client_addr))
}
