use packethandler::PacketHandler;
use super::nethandler::NetHandler;
use machine::Machine;
use packets::*;

pub struct FileServer {
	/// The machine this fileserver is running on.
	machine: Machine
}

impl FileServer {
	pub fn new(machine: Machine) -> FileServer {
		FileServer {
			machine: machine
		}
	}
}

impl PacketHandler for FileServer {
	fn handle(&self, packets: &Vec<(ClientId, Packet)>) {
		for p in packets {
			println!("Received packet: {:?}", p);
		}
	}
}
