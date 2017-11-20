use packets::*;

pub trait PacketHandler {
	fn handle(&self, packets: &Vec<(ClientId, Packet)>);
}
