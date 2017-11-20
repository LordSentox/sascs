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

pub fn main() {
}
