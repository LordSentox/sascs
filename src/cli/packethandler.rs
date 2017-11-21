use packets::*;
use packethandler::PacketHandler as PH;

pub struct PacketHandler {
}

impl PH for PacketHandler {
	fn handle(&mut self, packets: &Vec<(ClientId, Packet)>) {
		for p in packets {
			println!("Packet received: {:?}", p);
		}
	}
}
