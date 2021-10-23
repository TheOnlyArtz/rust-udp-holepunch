# rust-udp-holepunch
A small holepunching implementation written in Rust (UDP)

#### Prerequisites
Your rendezvous server must lay in a network which **doesn't** have a NAT!
The peers may or may not lay in the same network

### Getting Started
`server`
(The server runs on port 3000)
Running the rendezvous server should be as simple as running
```
cargo r
```
or (assuming you've compiled)
```
./server.exe
```

`client`
Running a client (or a peer) should be as simple as running
```
cargo r <local_ip>:<desired_port> <rendzvous_ip>
```
or 
```
./client.exe <local_ip>:<desired_port> <rendzvous_ip>
```

### Example
running the server on a VPS **(without a NAT!!!!)**
```
./server.exe
```

`client A on network A`
```
./client.exe 10.0.0.29:44445 216.58.212.110
```

`client B on network B`
```
./client.exe 192.168.10.5:31112 216.58.212.110
```

The peers may now communicate with each other and send `Handshake!` strings.
you may close the rendezvous server and watch as they speak without the need of port-forwarding!
