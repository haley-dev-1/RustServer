
/* 
 * UDP Server
 *  Haley Lind
 *   Rust/Cyber/Networks Journey
 */


use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
	println!("Hello, world!\n");
	
	{
		// our UDP socket
		let sock = UdpSocket::bind("127.0.0.1:3400")?; 

		// lets have a buffer of size 10 for our sock to write to
		let mut buff = [67; 10];

		println!("\nWaiting to receive information...");


		loop {

			let (amnt, src) = sock.recv_from(&mut buff).expect("message too long");
			
			let buff = &mut buff[..amnt]; 		// grabs bytes from buffer
			println!("CLIENT: [{src}] {:?}", &buff);

			// we can do manipulation to the UDP message here e.g. reverse
			// buff.reverse();

			sock.send_to(buff, &src)?;
	
			let msg = &buff;
	
			if msg == b"quit" {
				println!("Shutting down UDP server.");
				break;

		     	}
		}
	
		Ok(())

	}
}
