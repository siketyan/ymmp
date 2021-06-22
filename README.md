# 🌏 YMMP: Yumetter Messaging Protocol

## Prerequisites
- Rust Toolchain 1.53.0+

## Summary
YMMP is a protocol for broadcast a notification to all nodes within the network or subnet, made for Yumetter project.
The lower layer protocol is UDP (User Diagram Protocol), and uses port 17339 by default.

## Usage
```rust
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 17339);
let target = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, 17339));
let broadcaster = ymmp::Broadcaster::bind(addr, target);

let message = vec![b'h', b'e', b'l', b'l', b'o'];
let packet = ymmp::Packet::new(message);

broadcaster
    .broadcast(packet)
    .expect("Failed to broadcast.")
;
```

## Diagram
<table>
  <thead>
    <tr>
      <th></th>
      <th>0</th>
      <th>1</th>
      <th>2</th>
      <th>3</th>
      <th>4</th>
      <th>5</th>
      <th>6</th>
      <th>7</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">0</th>
      <td colspan="4">Magic Bytes (`YMMP`)</td>
      <td colspan="4">Version (`v1.0`)</td>
    </tr>
    <tr>
      <th scope="row">8</th>
      <td colspan="8">Length of Message (Little Endian)</td>
    </tr>
    <tr>
      <th scope="row">16</th>
      <td colspan="8" rowspan="2">Raw Message (0..N octets)</td>
    </tr>
    <tr>
      <th scope="row">32</th>
    </tr>
  </tbody>
</table>
