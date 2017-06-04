use atomic::{AtomicFlag, ATOMIC_FLAG_INIT};
use ctrlc;
use std::ops::Deref;
use std::net::SocketAddr;
use std::str::FromStr;
use std::thread;
use futures::{Async, BoxFuture, Canceled, Future, Poll, Sink, Stream};
use futures::sync::mpsc;
use futures::sync::oneshot;
use ws;

fn setup_signal_handler<F: Fn() -> () + 'static + Send>(handler: F) {
	static SIGNAL_HANDLER_REGISTERED: AtomicFlag = ATOMIC_FLAG_INIT;

	if !SIGNAL_HANDLER_REGISTERED.test_and_set() {
		if let Err(error) = ctrlc::set_handler(handler) {
			println!("Error registering signal handler: {}", match error {
				ctrlc::Error::Init(str) => str,
				ctrlc::Error::MultipleHandlers(str) => str,
				ctrlc::Error::SetHandler => "failed to set handler".into()
			});
		}
	}
}

pub fn run(port: u16) {
	let server = listen(port);
	let stopper = server.get_stopper();
	setup_signal_handler(move || stopper.clone().stop());
	if let Err(error) = server.wait() {
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
	thread::spawn(move || {
		println!("Listening on {}", address);
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
		Server { listener, shutdown_transmitter: tx }
	}
	fn get_stopper(&self) -> Stopper {
		Stopper { tx: self.shutdown_transmitter.clone() }
	}
}

composite_error!(
	ServerError,
	WebSocket(ws::Error),
	Future(Canceled)
);

impl Future for Server {
	type Item = ();
	type Error = ServerError;
	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		match self.listener.poll() {
			Ok(Async::Ready(result)) => match result {
				(Ok(_), Ok(_)) => Ok(Async::Ready(())),
				(Err(error), Ok(_)) => Err(error.into()),
				(Ok(_), Err(error)) => Err(error.into()),
				(Err(error1), Err(error2)) => {
					println!("Unhandled error: {}", error2);
					Err(error1.into())
				},
			},
			Ok(Async::NotReady) => Ok(Async::NotReady),
			Err(error) => Err(error.into())
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
