# Transport Protocol (Layer 4) Benchmark

This is designed to be a generic network protocol benchmark tailored around specific use cases. It will 
generate some metrics around the performance of various network protocols under a number of simulated scenarios. As
with all benchmarks, take all results with a grain of salt.

Metrics to track:
- Latency: 1-way, RTT minimum/maximum/mean
- Throughput: minimum/maximum/mean
- Jitter: mean
- Total number of sent/received packets: For reliable protocols, we expect these to be equivalent

Configurable settings:
- Packet loss - % packets dropped
- Latency - target latency +- thread sleep resolution
- Jitter - delay amount to +- to the target latency

Initially, the tests will place some focus on a fast-paced multiplayer game scenario. e.g. FPS games, realtime 
simulations, etc.

Specifically:
* 60 hertz send/receive rate
* Payloads of various sizes: less than or greater than MTU (~1500 bytes)

Last thing to note, I'm going to use Rust's default UDP/TCP implementation for those benchmarks. 

## Protocols to test
* [UDP](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
* [TCP](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
* [RakNet](https://github.com/facebookarchive/RakNet)
* [Valve GameNetworkingSockets](https://github.com/ValveSoftware/GameNetworkingSockets)
* [Unity Transport Package](https://github.com/Unity-Technologies/multiplayer)
* [QUIC as implemented by Quinn](https://github.com/djc/quinn)
* [Laminar](https://github.com/amethyst/laminar)

## Methodology
Each protocol implementation is going to run in a docker container configured using linux's built in
`Traffic Control` (tc) command to simulate unfavorable network conditions.
