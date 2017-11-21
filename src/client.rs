#![allow(dead_code)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate argparse;
extern crate chrono;

mod packets;
mod cli;
mod remote;
mod packethandler;
mod machine;
mod fs;

use std::net::TcpStream;
use remote::Remote;
use cli::nethandler::NetHandler;
use packets::*;
use machine::Machine;

pub fn main() {
	// Test connecting to a server.
	let stream = TcpStream::connect("localhost:44942").expect("Could not connect to server.");
	let remote = Remote::new(stream).expect("Could not create remote from stream.");
	let nethandler = NetHandler::new(remote);

	// Read current home directory state.
	let fs = fs::walk_files(".").expect("Could not read home directory");
	let fs = fs::with_modified_time(fs).expect("Could not read modification times");

	let machine = Machine::now_with_fs(fs);
	nethandler.send(&Packet::MachineState(machine));
}
