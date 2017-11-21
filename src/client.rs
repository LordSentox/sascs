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
mod machine;
mod fs;

use std::net::TcpStream;
use std::path::Path;
use argparse::{Store, StoreFalse, Print, ArgumentParser};

use remote::Remote;
use cli::nethandler::NetHandler;
use packets::*;
use machine::Machine;

pub fn main() {
	let mut directory: String = ".".into();
	let mut do_push = true;
	let mut remote: String = "localhost:44942".into();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Sync files of a specific directory to another computer.");
		ap.refer(&mut do_push)
			.add_option(&["-D", "--dry-run"], StoreFalse, "Do not push, just check which files would be replaced on the other end.");
		ap.refer(&mut directory)
			.add_option(&["-d", "--directory"], Store, "Specify the directory to run on");
		ap.refer(&mut remote)
			.add_option(&["-r", "--remote"], Store, "Specify the remote to push to");
		ap.add_option(&["-V", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");

		ap.parse_args_or_exit();
	}

	// Check that the directory entered is actually valid.
	match fs::metadata(&directory) {
		Ok(m) => {
			if !m.is_dir() {
				panic!("'{}' is not a directory", &directory);
			}
		},
		Err(e) => {
			panic!("'{}' is invalid: {}", &directory, e);
		}
	}

	// Read the basic directory data into memory.
	let fs = fs::walk_files(&directory).expect("Could not read directory");
	let fs = fs::with_modified_time(fs).expect("Could not read modification times");
	
	// Construct the struct corresponding to the local machine.
	let this = Machine::now_with_fs(fs);

	// Connect to the file server.
	let stream = TcpStream::connect(remote).expect("Could not connect to server.");
	let remote = Remote::new(stream).expect("Could not create remote from stream.");
	let nethandler = NetHandler::new(remote);

	// Send the request to the server.
	nethandler.send(&Packet::ReqPull(this.clone()));

	// Wait for the answer from the server.
	loop {
		// All packets received from the server waiting in the queue.
		for p in nethandler.collect_packets() {
			match p {
				Packet::ReqFile(path) => {
					if do_push { real_push(&path, &nethandler); }
					else {
						println!("Replacement candidate: {}", path.to_str().unwrap());
					}
				},
				Packet::Disconnect => {
					// The server aborts the transmission, or we are done.
					nethandler.disconnect();
					break;
				},
				p => {
					println!("This packet should not have been received: {:?}", p);
				}
			}
		}
	}
}

pub fn real_push<P: AsRef<Path>>(file: P, nethandler: &NetHandler) {
	println!("Pushing to server");
}
