use packethandler::PacketHandler as PH;
use super::nethandler::NetHandler;
use machine::Machine;
use packets::*;

pub struct PacketHandler {
	/// The machine this fileserver is running on.
	machine: Machine
}

impl PacketHandler {
	pub fn new(machine: Machine) -> PacketHandler {
		PacketHandler {
			machine: machine
		}
	}
}

impl PH for PacketHandler {
	fn handle(&mut self, packets: &Vec<(ClientId, Packet)>) {
		for p in packets {
			println!("Received packet: {:?}", p);
		}
	}
}
