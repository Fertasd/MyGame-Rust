use std::net;
use std::net::IpAddr;
use std::ops::Deref;
use ws;

pub fn connect(address: net::SocketAddr) {
	let address_str = String::from("ws://") + &address.to_string();
	if let Err(error) = ws::connect(address_str.deref(), |_| Client::new()) {
		println!("Error connecting to {}\n{}", address_str, error)
	}
}

struct Client;

impl Client {
	fn new() -> Client {
		Client {}
	}
}

impl ws::Handler for Client {}
