use packets::*;
use remote::Remote;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub struct NetClient {
	id: ClientId,
	remote: Arc<Remote>
}

impl NetClient {
	/// Create a new client.
	pub fn new(id: ClientId, remote: Remote) -> NetClient {
		NetClient {
			id: id,
			remote: Arc::new(remote)
		}
	}

	pub fn id(&self) -> ClientId {
		self.id
	}

	pub fn start_rcv(&self, tx: Sender<(ClientId, Packet)>) -> JoinHandle<()> {
		let remote_clone = self.remote.clone();
		let id = self.id;

		thread::spawn(move || {
			let mut running = true;
			while running {
				// Read the latest packet from the remote.
				let p = match remote_clone.read_packet() {
					Ok(p) => p,
					Err(PacketReadError::Closed) => {
						// Stop receiving after this.
						running = false;

						// Create a virtual Disconnection packet, in case the
						// packet handler needs to do something special when
						// the client disconnects.
						Packet::Disconnect
					},
					Err(err) => {
						// Ignore the packet.
						println!("Error reading packet from client [{}]. {:?}", id, err);
						continue;
					}
				};

				// Try sending the packet to the NetHandler. If the receiver
				// has hung up already, the packet receive thread can be
				// cancelled.
				if tx.send((id, p)).is_err() {
					running = false;
				}
			}
		})
	}

	pub fn send(&self, p: &Packet) -> bool {
		self.remote.write_packet(&p)
	}
}

