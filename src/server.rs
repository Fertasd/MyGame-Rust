use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::thread;
use futures::{Async, BoxFuture, Canceled, Future, Poll, Sink, Stream};
use futures::sync::mpsc;
use futures::sync::oneshot;
use ws;

pub fn run(port: u16) {
	if let Err(error) = listen(port).wait() {
		println!("Error with websocket server: {}", error);
	}
}

fn listen(port: u16) -> Server {
	let (v4_rx, v4_broadcaster) = listen_on(SocketAddr::new([127, 0, 0, 1].into(), port));
	let (v6_rx, v6_broadcaster) = listen_on(SocketAddr::from_str(&format!("[::1]:{}", port)).unwrap());

	Server::new(v4_rx.join(v6_rx).boxed(), [v4_broadcaster, v6_broadcaster])
}

fn listen_on(address: SocketAddr) -> (oneshot::Receiver<ws::Result<ws::WebSocket<Listener>>>, ws::Sender) {
	let socket = ws::WebSocket::new(Listener).unwrap();
	let broadcaster = socket.broadcaster();
	let (tx, rx) = oneshot::channel::<ws::Result<ws::WebSocket<Listener>>>();
	println!("Listening on {}", address);
	thread::spawn(move || {
		let _ = tx.send(socket.listen(address));
	});
	(rx, broadcaster)
}

#[derive(Clone)]
struct Stopper
{
	tx: mpsc::Sender<()>
}

impl Stopper {
	fn stop(self) {
		let _ = self.tx.send(()).wait();
	}
}

struct Server {
	listener: BoxFuture<(ws::Result<ws::WebSocket<Listener>>, ws::Result<ws::WebSocket<Listener>>), Canceled>,
	shutdown_transmitter: mpsc::Sender<()>
}

impl Server {
	fn new(listener: BoxFuture<(ws::Result<ws::WebSocket<Listener>>, ws::Result<ws::WebSocket<Listener>>), Canceled>, broadcasters: [ws::Sender; 2]) -> Server
	{
		let (tx, rx) = mpsc::channel(1);
		thread::spawn(move || {
			let _ = rx.into_future().wait();
			for broadcaster in broadcasters.into_iter() {
				let _ = broadcaster.shutdown();
			}
		});
		Server { listener: listener, shutdown_transmitter: tx }
	}
	fn get_stopper(&self) -> Stopper {
		Stopper { tx: self.shutdown_transmitter.clone() }
	}
}

impl Future for Server {
	type Item = ();
	type Error = Canceled;
	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		match self.listener.poll() {
			Ok(Async::Ready(_)) => Ok(Async::Ready(())),
			Ok(Async::NotReady) => Ok(Async::NotReady),
			Err(err) => Err(err)
		}
	}
}

struct Listener;

impl ws::Factory for Listener {
	type Handler = ClientConnection;

	fn connection_made(&mut self, sender: ws::Sender) -> Self::Handler {
		ClientConnection::new(sender)
	}
}

struct ClientConnection {
	sender: ws::Sender
}

impl ClientConnection {
	fn new(sender: ws::Sender) -> ClientConnection {
		ClientConnection { sender: sender }
	}
}

impl ws::Handler for ClientConnection {}
