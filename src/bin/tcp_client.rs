
/*
 * UDP client
 *  Haley Lind
 *   Rust Journey 2025
 */


use std::net::UdpSocket;
use std::time;
use std::io;

fn main() -> std::io::Result<()> {

	// client and server sockets here
	let sock = UdpSocket::bind("127.0.0.1:0")?;
	let server = "127.0.0.1:3400";

	loop {
		
		// build an empty message to send, e.g. "quit"
		//let mut msg: &[u8] = b"hello"; 		// byte literal 	
			
		// get user input
		let mut msg_line = String::new();
		io::stdin().read_line(&mut msg_line)?;

		// converts input string to bytes
		let msg = &msg_line.trim_end().as_bytes();
		
		// send byte array message to server, server is bound to 34000
		sock.send_to(&msg, server)?;
		println!("sent {:?}", &msg);
		
		// exit condition: 
		if msg == b"quit" {
			break;
			println!("quit was sent, exitting now!");
		}
		
		std::thread::sleep(time::Duration::from_millis(50));
	}
	Ok(())
}

