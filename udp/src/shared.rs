use get_if_addrs::get_if_addrs;
use serde_derive::{Serialize, Deserialize};
use std::{
    io,
    net::{IpAddr, SocketAddr, UdpSocket},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH}
};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u32,
    time: u128
}

pub fn get_eth_addr() -> Result<IpAddr, io::Error> {
    let addrs = get_if_addrs()?;
    let eth_addr = addrs
        .iter()
        .filter(|&addr| addr.name == "en0" || addr.name == "eth0")
        .map(|addr| addr.addr.ip())
        .last();

    match eth_addr {
        Some(a) => Ok(a),
        None => panic!("Can't run test! No address available.")
    }
}

pub fn start(client_addr: SocketAddr, server_addr: SocketAddr) {
    let socket = UdpSocket::bind(client_addr).unwrap();
    socket.set_nonblocking(true).unwrap();
    let mut id = 0;
    let mut now = SystemTime::now();
    loop {
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let now_ms = since_the_epoch.as_millis();
        let message = serde_json::to_vec(&Message {id, time: now_ms}).unwrap();
        socket.send_to(&message, server_addr).expect("Send should be successful");
        thread::sleep(Duration::from_secs(1) / 60);
        now = SystemTime::now();
        id += 1;
    }
}
