extern crate ws;

mod area;
mod client;
mod server;

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

fn main() {
	match env::args().len() {
		1 => panic!("Server not implemented"),
		2 => {
			let address_str = &env::args().nth(1).unwrap();
			match SocketAddr::from_str(address_str) {
				Ok(address) => client::connect(address),
				Err(error) => println!("Cannot parse {} as server address.", address_str)
			}
		},
		_ => println!("Usage: rpg-game [server-address]")
	}
}
