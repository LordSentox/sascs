extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::path::Path;

mod fs;
mod packets;
mod packethandler;
mod machine;
mod nethandler;
mod netclient;
mod remote;

use fs::walk_files;

fn main() {
	// Test the fs-walk on the current directory.
	let cur_files = walk_files(&Path::new(".")).expect("Error occured walking the current directory.");

	println!("Current directory contents: {:?}", cur_files);
}
