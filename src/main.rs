use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
	println!("Hello, world!");
	
	{
		// our UDP socket
		let sock = UdpSocket::bind("127.0.0.1:3400")?; 

		// lets have a buffer of size 10 for our sock to write to
		let mut buff = [67; 10];

		loop {
			println!("Waiting to receive information...");

			let (amnt, src) = sock.recv_from(&mut buff).expect("message too long");
			println!("got {amnt} bytes from {src}");
			
			let buff = &mut buff[..amnt]; 		// grabs bytes from buffer
			println!("{:?}", &buff);	// displays bytes, formatted

	//		println!("we will buff.reverse...");
			buff.reverse();
	//		println!("{:?}", &buff[..amnt]);	// displays bytes, formatted

			println!("now we will send to buffer...");
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
