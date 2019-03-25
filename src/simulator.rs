use pnet::datalink::{
    self,
    Channel::Ethernet
};
use std::{
    net::SocketAddr,
    thread::{self, JoinHandle},
};

/// The NetworkSimulator acts as a proxy to simulate various network conditions
pub struct NetworkSimulator {
    packet_loss_percent: u32,
    target_latency_ms: u32,
    jitter_ms: u32,
}

impl NetworkSimulator {
    pub fn new(
        packet_loss_percent: u32,
        target_latency_ms: u32,
        jitter_ms: u32,
    ) -> Self {
        Self {
            packet_loss_percent,
            target_latency_ms,
            jitter_ms,
        }
    }

    /// Listens on the loopback interface and introduces all sorts of poor network conditions
    pub fn start(&self) -> JoinHandle<()> {
        let interfaces = datalink::interfaces();
        let loopback_interface = interfaces
            .into_iter()
            .find(|interface| interface.is_loopback())
            .expect("Should definitely have a loopback.");

        thread::spawn(move || {
            let (mut tx, mut rx) = match datalink::channel(&loopback_interface, Default::default()) {
                Ok(Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unhandled channel type"),
                Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
            };

            loop {
                match rx.next() {
                    Ok(packet) => {
                        tx.send_to(packet, None);
                    },
                    Err(e) => {
                        // If an error occurs, we can handle it here
                        panic!("An error occurred while reading: {}", e);
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::NetworkSimulator;
}
