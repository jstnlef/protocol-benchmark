# Layer 4 Protocol Benchmark

This is designed to be a generic network protocol testing benchmark.

The key benefit of this library is to generate some metrics around the performance
of various network protocols under a number of simulated scenarios.

Metrics to track:

- Latency: 1-way, RTT minimum/maximum/mean
- Throughput: minimum/maximum/mean
- Jitter: mean
- Total number of sent/received packets: For reliable protocols, we expect these to be equivalent

Configurable settings:

- Packet loss - % packets dropped
- Latency - target latency +- thread sleep resolution
- Jitter - delay amount to +- to the target latency

This library has been designed to explicitly test situations
more similar to what a realtime multiplayer game would see.

Specifically:
* 60 hertz send/receive rate
* Payloads of various sizes: less than or greater than MTU (~1500 bytes)

## Protocols to test
* TCP
* UDP
* [QUIC as implemented by Quinn](https://github.com/djc/quinn)
* [laminar](https://github.com/amethyst/laminar)
* [RakNet](https://github.com/facebookarchive/RakNet)
* [Valve GameNetworkingSockets](https://github.com/ValveSoftware/GameNetworkingSockets)
