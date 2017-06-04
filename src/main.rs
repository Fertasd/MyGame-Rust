extern crate futures;
extern crate ws;

mod area;
mod characters;
mod client;
mod items;
mod server;

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::ops::Deref;

fn print_usage() {
	println!("Usage:");
	println!("rpg-game");
	println!("rpg-game server-address");
	println!("rpg-game client-port");
}

fn main() {
	match env::args().len() {
		1 => panic!("Client controller not implemented"),
		2 => match env::args().nth(1).unwrap().deref() {
			"-h" | "--help" => print_usage(),
			address_str => {
				if let Ok(address) = SocketAddr::from_str(address_str) {
					client::connect(address)
				} else if let Ok(port) = address_str.parse::<u16>() {
					server::run(port)
				} else {
					println!("Cannot parse {} as server address or client port.", address_str)
				}
			}
		},
		_ => print_usage()
	}
}
