use super::error::GitError;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn init() -> Result<(), GitError> {
    let dir = Path::new(".agit");

    //create an empty directory at the provided path
    fs::create_dir(dir)?;
    //create an empty directory /dir/objects
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    //create a file "HEAD" at dir directory
    let mut head = File::create(dir.join("HEAD"))?;
    head.write_all("refs: refs/heads/master".as_bytes())?;

    Ok(())
}
