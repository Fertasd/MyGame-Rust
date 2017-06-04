use std::net::IpAddr;
use ws;

pub struct NetworkClient {
	sender: ws::Sender
}

impl NetworkClient {
	pub fn connect(address: IpAddr, function: FnOnce(NetworkClient)) {
		let mut socket = ws::WebSocket::new(client).unwrap();
		socket.connect(address.into()).unwrap();
		function(socket.r)
	}
}

impl ws::Factory for NetworkClient {
	type Handler = Self;

	fn connection_made(&mut self, sender: ws::Sender) -> Self::Handler {
		NetworkClient { sender: sender }
	}
}