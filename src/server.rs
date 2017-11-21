#![allow(dead_code)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate argparse;
extern crate chrono;

use std::path::Path;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::thread;
use std::time::Duration;

mod fs;
mod packets;
mod machine;
mod remote;
mod srv;

use fs::walk_files;
use srv::NetHandler;
use machine::Machine;

fn main() {
	let mut home_dir = Path::new(".");
	let mut port = 44942;
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Listen to incoming file system pushes.");
		ap.refer(&mut port).add_option(&["-p", "--port"], Store, "Listen to this port. Default is 44942");

		ap.parse_args_or_exit();
	}

	let nethandler = NetHandler::new();
	nethandler.start_listen(port).expect("Could not start listening");

	loop {
		let fs = fs::walk_files(home_dir).expect("Could not read home dir");
		let fs = fs::with_modified_time(fs).expect("Could not read time metadata");
		let machine = Machine::now_with_fs(fs);

		for (c, p) in nethandler.collect_packets() {
		}

		thread::sleep(Duration::from_millis(5000));
	}
}
