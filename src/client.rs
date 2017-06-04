use std::net;
use std::net::IpAddr;
use std::ops::Deref;
use ws;

pub fn connect(address: net::SocketAddr) {
	let address_str = String::from("ws://") + &address.to_string();
	if let Err(error) = ws::connect(address_str.deref(), |sender| Client::new(sender)) {
		println!("Error with client connection {}\n{}", address_str, error)
	}
}

struct Client {
	sender: ws::Sender
}

impl Client {
	fn new(sender: ws::Sender) -> Client {
		Client { sender }
	}
}

impl ws::Handler for Client {}
