# Rust Server

## Goal

This project is meant to build a Rust server using TCP and UDP.  
This project is meant to build a custom protocol for robotics/ML inference. 
This project is meant to be off LAN, thus off internet, running completely secure and local! 

## Direcory 
#### Print with tree tool
```bash
sudo apt install tree
```
- bin 		-> 	starting program e.g. main  
- server/client -> 	how program works  
- protocol 	-> 	message structs & encoding  

From source: 
.
├── bin
│   ├── client.rs
│   └── server.rs
├── client
│   ├── mod.rs
│   └── tcp.rs
├── main.rs
├── protocol
│   ├── control.rs
│   └── telemetry.rs
├── protocol.rs
└── server
    ├── tcp_server.rs
    └── udp_server.rs
