use get_if_addrs::get_if_addrs;
use serde_derive::{Serialize, Deserialize};
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

    let client_handle = start_client(client_addr, server_addr);
    let server_handle = start_server(client_addr, server_addr);
    client_handle.join().unwrap();
    server_handle.join().unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u32,
    time: u128
}

fn start_client(client_addr: SocketAddr, server_addr: SocketAddr) -> JoinHandle<()> {
    thread::spawn(move || {
        let socket = UdpSocket::bind(client_addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        let mut id = 0;
        let mut now = SystemTime::now();
        loop {
            let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let now_ms = since_the_epoch.as_millis();
            let message = serde_json::to_vec(&Message {id, time: now_ms}).unwrap();
            socket.send_to(&message, server_addr);
            thread::sleep(Duration::from_secs(1) / 60);
            now = SystemTime::now();
            id += 1;
        }
    })
}

fn start_server(client_addr: SocketAddr, server_addr: SocketAddr) -> JoinHandle<()> {
    let mut buffer: Vec<u8> = vec![0; 1500];
    thread::spawn(move || {
        let socket = UdpSocket::bind(server_addr).unwrap();
        loop {
            match socket.recv(&mut buffer) {
                Ok(len) => {
                    let message = serde_json::from_slice::<Message>(&buffer[..len]).unwrap();
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                    println!("{:?}", now - Duration::from_millis(message.time as u64));
                },
                Err(e) => {}
            }
        }
    })
}

fn get_eth_addr() -> Result<IpAddr, io::Error> {
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
