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
	pub fn newer_files_from(&self, other: &Machine) -> Vec<(PathBuf, SystemTime)> {
		let mut own = self.fs.clone();
		let mut oth = other.fs.clone();
		own.sort_by(|a, b| { a.0.cmp(&b.0) });
		oth.sort_by(|a, b| { a.0.cmp(&b.0) });
		let own = own;
		let oth = oth;

		let mut res: Vec<(PathBuf, SystemTime)> = Vec::new();
		let mut i = 0; let mut j = 0;
		while i < own.len() && j < oth.len() {
			if own[i].0 == oth[j].0 {
				// The file exists on both systems, so check which one is newer and
				// keep that one.
				// XXX: This does not take the different system times into account.
				if own[i].1 < oth[j].1 {
					res.push(oth[j].clone());
				}

				i += 1;
				j += 1;
			}
			if own[i].0 < oth[j].0 {
				// The own machine has a file that is not on the other machine.
				// Advance own fs by one next loop, the other will check the
				// current file again.
				j += 1;
			}
			else {
				// The other machine has a file thit is not on the own machine.
				// Advance other fs by one next loop, own stays the same.
				// Also add the new file lacking in the current fs.
				res.push(oth[j].clone());
				i += 1;
			}
		}

		// If there are files at the end of the new machine left over, add all of them.
		while j < oth.len() {
			res.push(oth[j].clone());
		}

		res
	}
}
