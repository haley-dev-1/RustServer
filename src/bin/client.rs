use std::net::UdpSocket;

fn main() -> std::io::Result<()> {

	println!("Hello client\n");

	// our client socket
	let sock = UdpSocket::bind("127.0.0.1:0")?;
	let server = "127.0.0.1:3400";

	loop {
		
		// build an empty message to send, e.g. "quit"
		let msg: &[u8] = &[0; 10]; // array of 10 bytes	
			
		// alter message based on input from cli

		// send message to server, server is bound to 34000
		sock.send_to(msg, server)?;
		println!("sent {:?}", msg);
		
		// exit condition: 
		if msg ==b"quit" {
			break;
		}
	}


	Ok(())
}

