use packets::*;
use remote::Remote;
use super::netclient::NetClient;
use packethandler::PacketHandler;

use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use std::collections::HashMap;
use std::io::Error as IOError;

/// Look for any id that has not been given to a client. Optionally, a starting
/// id can be provided, where the caller expects room to be after.
/// Starts at 0 otherwise.
fn search_free_id<T>(map: &HashMap<ClientId, T>, start: ClientId) -> Option<ClientId> {
	// Search upwards, which is more probable to have room.
	for key in start..ClientIdMAX {
		if !map.contains_key(&key) {
			return Some(key);
		}
	}

	// Search low, since some old keys might be free again.
	for key in 1..start - 1 {
		if !map.contains_key(&key) {
			return Some(key);
		}
	}

	// No free key could be found.
	None
}

#[derive(Debug)]
pub enum Error {
	AlreadyListening,
	SockErr(IOError)
}

pub struct NetHandler {
	clients: Arc<RwLock<HashMap<ClientId, NetClient>>>,
	tx: Sender<(ClientId, Packet)>,
	rx: Receiver<(ClientId, Packet)>
}

impl NetHandler {
	pub fn new() -> NetHandler {
		let (tx, rx) = mpsc::channel();

		NetHandler {
			clients: Arc::new(RwLock::new(HashMap::new())),
			tx: tx,
			rx: rx
		}
	}

	pub fn start_listen(&self, port: u16) -> Result<(), Error> {
		let listener = match TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))) {
			Ok(l) => l,
			Err(err) => return Err(Error::SockErr(err))
		};

		let clients_cl = self.clients.clone();
		let tx_cl = self.tx.clone();
		thread::spawn(move || {
			let mut last_id: ClientId = 0;

			// Listen for and accept all clients that want to connect.
			for stream in listener.incoming() {
				// Check if the stream is valid and try to create a client for it.
				let stream = match stream {
					Ok(stream) => stream,
					Err(err) => {
						println!("Client tried to connect, but could not be accepted. {}", err);
						continue;
					}
				};

				last_id = match search_free_id(&clients_cl.read().unwrap(), last_id+1) {
					Some(id) => id,
					None => {
						println!("Could not find a free id. Denying client.");
						continue;
					}
				};

				// Wrap the TcpStream in a Remote
				let remote = match Remote::new(stream) {
					Ok(r) => r,
					Err(err) => {
						println!("Could not create remote for the client. Dropping. {:?}", err);
						continue;
					}
				};

				let client = NetClient::new(last_id, remote);
				client.start_rcv(tx_cl.clone());

				clients_cl.write().unwrap().insert(last_id, client);
			}
		});

		Ok(())
	}

	pub fn handle_packets<H>(&self, handler: &mut H) where H: PacketHandler {
		// Collect all packets into a Vec in case the handler needs a packet
		// that has come later first.
		let packets = self.rx.try_iter().collect();

		handler.handle(&packets);

		// Remove clients where the other end has hung up.
		for (c, p) in packets {
			if let Packet::Disconnect = p {
				self.clients.write().unwrap().remove(&c);
			}
		}
	}

	pub fn broadcast(&self, packet: &Packet) -> bool {
		let mut one_failed = false;
		let clients = self.clients.read().unwrap();
		for (ref id, ref client) in &*clients {
			if !client.send(&packet) {
				println!("Broadcasting {:?} failed for client [{}]", packet, id);
				one_failed = true;
			}
		}

		!one_failed
	}

	pub fn send(&self, to: ClientId, packet: &Packet) -> bool {
		match self.clients.read().unwrap().get(&to) {
			Some(ref client) => client.send(&packet),
			None => {
				println!("Could not send packet. Client [{}] does not exist.", to);
				false
			}
		}
	}

	pub fn clients(&self) -> RwLockReadGuard<HashMap<ClientId, NetClient>> {
		self.clients.read().unwrap()
	}

	pub fn clients_mut(&self) -> RwLockWriteGuard<HashMap<ClientId, NetClient>> {
		self.clients.write().unwrap()
	}
}
