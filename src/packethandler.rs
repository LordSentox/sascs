use super::nethandler::NetHandler;
use packets::*;

pub trait PacketHandler {
	fn handle(&self, nh: &NetHandler, packets: &Vec<(ClientId, Packet)>);
}
