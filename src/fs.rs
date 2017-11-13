use std::path::{Path, PathBuf};
use std::fs;
use std::io::Result as IOResult;

/// Walk through the directory and return all files with their respective
/// paths relative to the path provided.
pub fn walk_files(p: &Path) -> IOResult<Vec<PathBuf>> {
	let metadata = fs::symlink_metadata(p)?;

	if metadata.is_file() {
		// End of recursion. Just return the path the file was found at.
		return Ok(vec!(p.to_path_buf()));
	}

	let mut files = Vec::new();

	// Walk through the directory and recursively call this function on all elements.
	for e in fs::read_dir(p)? {
		let e = e?;

		files.append(&mut walk_files(&e.path())?);
	}

	Ok(files)
}
