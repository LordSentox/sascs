use packets::*;

pub trait PacketHandler {
	fn handle(&mut self, packets: &Vec<(ClientId, Packet)>);
}
