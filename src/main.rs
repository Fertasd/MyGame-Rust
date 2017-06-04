extern crate ws;

mod area;
//mod client;

use area::*;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
	//let c = client::NetworkClient::connect(IpAddr::from_str("192.168.10.1:1600").unwrap());

	let mut d = Dungeon::new();
	let l = Level::new();
	d.add_level(Rc::new(l));
}
