/* 
 *   TCP Server
 *   by Haley Lind
 *   Rust/Cyber/Networks Journey
*/

use std::net::{TcpStream, TcpListener};
use std::collections::HashMap;
use std::time::Instant;

const MAX_TRANSMISSION_UNIT: usize = 1200; // in bytes

struct ClientStatistics {
	client_name: String, // name
	packets_sent: usize,
	packets_received: usize, 
	bytes_sent: usize,
	bytes_received: usize,
	last_seen: Instant, 
	malformed_packets: usize,
	oversize_packets: usize,
	unknown_msg_types: usize
}

fn handle_client(mut stream: TcpStream){

	let mut buffer = [0 as u8; MAX_TRANSMISSION_UNIT];
	
	// reads stream, stores read data into the buffer.
	stream.read(&mut buffer).expect("Failed to read from client");
	
	// handles converting data in buffer to utf8 string :) 
	let request = String::from_utf8_lossy(&buffer[..]);
	println!("Received request: {}", request);
	let response = String::from("Hello, Client").as_bytes();
	stream.write(response).expect("Cannot write server response to stream");

}

fn main() -> std::io::Result<()> {

	let mut client_table: HashMap<std::net::SocketAddr, ClientStatistics> = HashMap::new(); // keep track of the clients connected.
	
	{ 
		// our UDP socket
		let sock = TcpListener::bind("127.0.0.1:3400")?; 

		// lets have a buffer of size 10 for our sock to write to
		let mut buff = [0 as u8; MAX_TRANSMISSION_UNIT];

		println!("\nWaiting to receive information...");

		loop {

			let (amnt, src) = sock.recv_from(&mut buff)?;

			//let _: () = src;
			let stats = client_table.entry(src).or_insert_with(
				|| ClientStatistics {
					client_name: src.to_string(),
					packets_sent: 0,
					packets_received: 0,
					bytes_sent: 0,
					bytes_received: 0,
					last_seen: Instant::now(),
					malformed_packets: 0,
					oversize_packets: 0,
					unknown_msg_types: 0,
				}
			);

			stats.client_name = "".to_string();			
			stats.packets_received += 1;
			stats.bytes_received += amnt;
			stats.last_seen = Instant::now();
			stats.packets_sent += 1;
			
			// TODO: Size handling on amnt e.g. if it exceeds MTU, don't receive. 

			let buff = &mut buff[..amnt]; 		// grabs bytes from buffer

			println!("CLIENT: [{src}] {:?}, size {}, count: {}", &buff, &amnt, stats.packets_sent);

			// we can do manipulation to the UDP message here e.g. reverse
			// buff.reverse();

			sock.send_to(buff, &src)?;
	
			let msg = &buff;


			// TODO: Handle "quit" because only 1 client says "quit" for it to quit.	
			if msg == b"quit" {
				println!("Shutting down TCP server.");
				break;

		     	}
		}
	
		Ok(())

	}
}
