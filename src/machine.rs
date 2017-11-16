use std::time::SystemTime;
use std::path::{Path, PathBuf};

/// Represents a physical machine with the head of the file system part in
/// question loaded into memory.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
	time: SystemTime,
	fs: Vec<(PathBuf, SystemTime)>
}

impl Machine {
	pub fn new(time: SystemTime) -> Machine {
		Machine {
			time: time,
			fs: Vec::new()
		}
	}

	pub fn new_now() -> Machine {
		Machine {
			time: SystemTime::now(),
			fs: Vec::new()
		}
	}

	pub fn new_with_fs(time: SystemTime, fs: Vec<(PathBuf, SystemTime)>) -> Machine {
		Machine {
			time: time,
			fs: fs
		}
	}

	/// Campare the file system state of this machine to the state of the other
	/// machine. Returns all the files that are newer on the other Machine.
	/// XXX: Find a better name for this function.
	pub fn compare_fs_state(other: &Machine) -> Vec<(PathBuf, SystemTime)> {
		
	}
}

