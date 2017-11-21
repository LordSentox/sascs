use std::path::{Path, PathBuf};
use std::fs;
use std::io::Result as IOResult;
use std::time::SystemTime;
use std::process::Command;
use chrono::DateTime;
use chrono::prelude::*;

// Reexport from the std-module fs.
pub use std::fs::*;

/// Walk through the directory and return all files with their respective
/// paths relative to the path provided.
pub fn walk_files<P: AsRef<Path>>(p: P) -> IOResult<Vec<PathBuf>> {
	let metadata = fs::symlink_metadata(&p)?;

	if metadata.is_file() {
		// End of recursion. Just return the path the file was found at.
		return Ok(vec!(p.as_ref().to_path_buf()));
	}

	let mut files = Vec::new();

	// Walk through the directory and recursively call this function on all elements.
	for e in fs::read_dir(p)? {
		let e = e?;

		files.append(&mut walk_files(&e.path())?);
	}

	Ok(files)
}

pub fn with_modified_time(fs: Vec<PathBuf>) -> IOResult<Vec<(PathBuf, SystemTime)>> {
	let mut res = Vec::with_capacity(fs.len());
	for f in fs {
		let meta = f.metadata()?.modified()?;
		res.push((f, meta.into()));
	}

	Ok(res)
}

#[cfg(unix)]
pub fn set_modified<P: AsRef<Path>>(f: P, time: &SystemTime) -> bool {
	let time: DateTime<Local> = time.clone().into();
	// touch -m -t 198801010000.00 hello
	Command::new("touch")
		.arg("-m")
		.arg("-t").arg(time.format("%Y%m%d%H%M.%S").to_string())
		.arg(f.as_ref())
		.status().unwrap().success()
}

#[cfg(windows)]
pub fn set_modified<Tz: TimeZone>(f: &Path, time: &SystemTime) -> bool where Tz::Offset: Display {
	// TODO
	panic!("Windows ist doof.");
}

#[cfg(test)]
mod test {
	use super::*;
	use std::fs::File;
	use chrono::NaiveDateTime;
	use chrono::prelude::*;

	/// Create and walk an empty directory.
	#[test]
	fn walk_empty() {
		// Create a test directory.
		fs::create_dir_all("./.test_dir").unwrap();

		// Traversing the directory should yield an empty vec.
		let files = walk_files("./.test_dir").unwrap();
		assert!(files.is_empty());

		// Remove the test directory.
		fs::remove_dir("./.test_dir").unwrap();
	}

	/// Create and walk directory with empty sub dirs only.
	#[test]
	fn walk_empty_sub_dirs() {
		// Create a test directory.
		fs::create_dir_all("./.test_dir2").unwrap();
		fs::create_dir("./.test_dir2/one").unwrap();
		fs::create_dir("./.test_dir2/two").unwrap();
		fs::create_dir("./.test_dir2/three").unwrap();
		fs::create_dir("./.test_dir2/four").unwrap();

		// Traversing the directory should yield an empty vec.
		let files = walk_files("./.test_dir2").unwrap();
		assert!(files.is_empty());

		// Remove the test directory.
		fs::remove_dir_all("./.test_dir2").unwrap();
	}

	/// Test changing the modification time of a file
	#[test]
	fn test_set_modified() {
		let file = File::create(".test_file").expect("Could not create file");

		let time: DateTime<Local> = Local.ymd(2042, 4, 2).and_hms(0, 42, 42);
		assert!(set_modified(".test_file", &time.into()));

		assert_eq!(fs::metadata(".test_file").unwrap().modified().unwrap(), time.into());

		fs::remove_file(".test_file").unwrap();
	}

	#[test]
	fn test_with_modified_time() {
		fs::create_dir_all("./.test_dir3").expect("Could not create directory");

		let mut files: Vec<PathBuf> = vec!["./.test_dir3/one".into(), "./.test_dir3/two".into(), "./.test_dir3/three".into()];
		let mut times: Vec<SystemTime> = vec![Local.timestamp(0, 0).into(), Local.timestamp(963409, 0).into(), Local::now().into()];

		for (file, time) in files.iter().zip(times.clone()) {
			File::create(&file).expect("Could not create file");
			assert!(set_modified(&file, &time));
		}

		let with_mod = with_modified_time(files.clone()).unwrap();

		for ((lp, lt), (rp, rt)) in files.into_iter().zip(times).zip(with_mod) {
			let lt: DateTime<Local> = lt.into();
			let rt: DateTime<Local> = rt.into();

			// Using timestamp to ignore nanoseconds, which can't be set by touch.
			assert_eq!((lp, lt.timestamp()), (rp, rt.timestamp()));
		}

		fs::remove_dir_all("./.test_dir3").unwrap();
	}

}
