use remote::*;
use packets::*;
use packethandler::PacketHandler;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use std::io::Error as IOErr;
use std::io::ErrorKind as IOErrKind;

pub struct NetHandler {
	remote: Arc<Remote>,
	rx: Receiver<Packet>,
	running: Arc<AtomicBool>
}

impl NetHandler {
	pub fn new(remote: Remote) -> NetHandler {
		let (tx, rx) = mpsc::channel();
		let remote = Arc::new(remote);
		let running = Arc::new(AtomicBool::new(true));

		// Create a new thread for receiving packets.
		let remote_cl = remote.clone();
		let running_cl = running.clone();
		thread::spawn(move || {
			// Set the remote timeout, so that this thread can be closed gracefully.
			remote_cl.set_timeout(Some(Duration::from_millis(500)),
				DirSocket::Read).expect("Could net set Socket read timeout.");

			while running_cl.load(Ordering::Relaxed) {
				let packet = match remote_cl.read_packet() {
					Ok(p) => p,
					Err(PacketReadError::Closed) => {
						Packet::Disconnect
					},
					Err(PacketReadError::IOError(err)) => {
						if let IOErrKind::WouldBlock = err.kind() {
							// This error is thrown when the remote reaches the
							// timeout duration and can thusly be ignored.
							continue;
						}

						println!("IO Error reading packet: {:?}", err);
						continue;
					},
					Err(err) => {
						println!("Error reading packet. {:?}", err);
						continue;
					}
				};

				// Send the packet through the mspc channel.
				tx.send(packet).unwrap();
			}
		});

		NetHandler {
			remote: remote,
			rx: rx,
			running: running
		}
	}

	pub fn handle_packets<H>(&self, handler: &mut H) where H: PacketHandler {
		// Collect all packets into a Vec and send that to the PacketHandler.
		let packets = self.rx.try_iter().map(|x| { (0, x) }).collect();

		handler.handle(&packets);
	}

	pub fn send(&self, p: &Packet) -> bool {
		self.remote.write_packet(&p)
	}

	pub fn connected(&self) -> bool {
		self.running.load(Ordering::Relaxed)
	}

	pub fn disconnect(&self) {
		self.running.store(false, Ordering::Relaxed);
	}
}

impl Drop for NetHandler {
	fn drop(&mut self) {
		self.disconnect();
	}
}
