use bincode::{serialize, deserialize, Bounded, Error};

use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;

use super::machine::Machine;

pub type ClientId = u32;
pub use std::u32::MAX as ClientIdMAX;

pub const MAX_PACKET_SIZE: u64 = 512;

#[derive(Debug)]
pub enum PacketReadError {
	/// The packet could not be properly deserialised.
	DeserializeError(Error),
	/// The packet could not be read properly from the stream.
	IOError(io::Error),
	/// The connection has been closed by the peer socket.
	Closed
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Packet {
	/// Request the current system time of the target machine.
	ReqTime,
	/// Sends basic information about the machine and its file system state so
	/// that it can be compared to others.
	MachineState(Machine),
	/// An internal Packet to show that the connection has been closed by the client.
	Disconnect
}

impl Packet {
	pub fn write_to_stream(&self, stream: &mut TcpStream) -> bool {
		let size = Bounded(MAX_PACKET_SIZE);

		let data: Vec<u8> = match serialize(&self, size) {
			Ok(data) => data,
			Err(err) => { println!("Error serialising packet: {}", err); return false; }
		};

		match stream.write(&data) {
			Ok(len) => {
				assert!(len <= MAX_PACKET_SIZE as usize);
				true
			},
			Err(err) => {
				println!("Failed writing packet to stream: {}", err);
				false
			}
		}
	}

	// TODO: Later, the buffer could be borrowed, which would increase performance.
	// This has to be crosschecked with the inner workings of bytecode, however.
	/// Read a packet from the stream. This returns a packet, in case one could be read
	/// in conjunction with a bool stating false, in case the stream has been closed.
	pub fn read_from_stream(stream: &mut TcpStream) -> Result<Packet, PacketReadError> {
		let mut data: Vec<u8> = vec![0; MAX_PACKET_SIZE as usize];

		match stream.read(&mut data) {
			Ok(len) => {
				if len == 0 {
					Err(PacketReadError::Closed)
				}
				else {
					match deserialize(&data) {
						Ok(p) => Ok(p),
						Err(err) => {
							Err(PacketReadError::DeserializeError(err))
						}
					}
				}
			}
			// XXX: There might be some cleanup to do in case of the following
			// error, which still needs to be tested.
			Err (err) => Err(PacketReadError::IOError(err))
		}
	}
}
