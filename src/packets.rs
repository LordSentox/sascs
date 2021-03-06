use bincode::{serialize, deserialize, Bounded, Error};

use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;

use super::machine::Machine;
use std::path::PathBuf;

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
	/// Make a pull-request to the file server. The server will then in turn
	/// request files that are newer on the machine that made the request.
	ReqPull(Machine),
	/// Request a file to be sent.
	ReqFile(PathBuf),
	/// Indicates that a file is about to be sent. Sends basic information
	/// about the file that might be useful.
	/// The files path, the files size (in bytes)
	FileStart(PathBuf, u64),
	/// Send parts of a file. It is important that these are used in the
	/// strict order they are sent in.
	FileChunk(Vec<u8>),
	/// Marks the end of a file transmission.
	FileEnd,
	/// An internal Packet to show that the connection has been closed by the client.
	Disconnect
}

pub trait PacketWrite: io::Write {
	fn write_packet(&mut self, p: &Packet) -> bool {
		let size = Bounded(MAX_PACKET_SIZE);

		let data: Vec<u8> = match serialize(&p, size) {
			Ok(data) => data,
			Err(err) => { println!("Error serialising packet: {}", err); return false; }
		};

		match self.write(&data) {
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
}

pub trait PacketRead: io::Read {
	fn read_packet(&mut self) -> Result<Packet, PacketReadError> {
		let mut data: Vec<u8> = vec![0; MAX_PACKET_SIZE as usize];

		match self.read(&mut data) {
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

impl PacketWrite for TcpStream {}
impl PacketRead for TcpStream {}
