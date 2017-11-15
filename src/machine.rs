use std::time::SystemTime;
use std::path::{Path, PathBuf};

/// Represents a physical machine with the head of the file system part in
/// question loaded into memory.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
	time: SystemTime,
	fs: Vec<(PathBuf, SystemTime)>
}


