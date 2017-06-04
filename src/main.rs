extern crate ws;

mod area;
mod client;
mod server;

use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::ops::Deref;

fn print_usage() {
	println!("Usage: rpg-game [server-address]")
}

fn main() {
	match env::args().len() {
		1 => panic!("Server not implemented"),
		2 => {
			match env::args().nth(1).unwrap().deref() {
				"-h" | "--help" => print_usage(),
				address_str => match SocketAddr::from_str(address_str) {
					Ok(address) => client::connect(address),
					Err(error) => println!("Cannot parse {} as server address.", address_str)
				}
			}
		},
		_ => print_usage()
	}
}
