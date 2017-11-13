use std::path::Path;

mod fs;
use fs::walk_files;

fn main() {
	// Test the fs-walk on the current directory.
	let cur_files = walk_files(&Path::new(".")).expect("Error occured walking the current directory.");

	println!("Current directory contents: {:?}", cur_files);
}
