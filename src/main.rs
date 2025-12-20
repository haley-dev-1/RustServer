

use std::net::UdpSocket;
use std::collections::HashMap;

/* * UDP Server
 *   Haley Lind
 *   Rust/Cyber/Networks Journey
*/

const MAX_TRANSMISSION_UNIT: usize = 1200; // in bytes

fn main() -> std::io::Result<()> {

let mut client_table: HashMap<std::net::SocketAddr, usize> = HashMap::new(); // keep track of the clients connected.

	
	{
		// our UDP socket
		let sock = UdpSocket::bind("127.0.0.1:3400")?; 

		// lets have a buffer of size 10 for our sock to write to
		let mut buff = [67; MAX_TRANSMISSION_UNIT];

		println!("\nWaiting to receive information...");

		loop {

			let (amnt, src) = sock.recv_from(&mut buff)?;

			//let _: () = src;
			let counter = client_table.entry(src).or_insert(0);
			*counter += 1;

			// TODO: Size handling on amnt e.g. if it exceeds MTU, don't receive. 

			let buff = &mut buff[..amnt]; 		// grabs bytes from buffer
			println!("CLIENT: [{src}] {:?}, size {}, count: {}", &buff, &amnt, counter);

			// we can do manipulation to the UDP message here e.g. reverse
			// buff.reverse();

			sock.send_to(buff, &src)?;
	
			let msg = &buff;


			// TODO: Handle "quit" because only 1 client says "quit" for it to quit.	
			if msg == b"quit" {
				println!("Shutting down UDP server.");
				break;

		     	}
		}
	
		Ok(())

	}
}
