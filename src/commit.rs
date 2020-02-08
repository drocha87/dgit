// Commit file model
//
// tree-hash
// author:
// date:
// Message:
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use super::config;
use super::blob::Blob;


pub fn to_branch(branch: String) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(config::HEAD)?;
    file.write_all(branch.as_bytes())?;
    Ok(())
}

pub fn commit(msg: String) -> io::Result<()> {
    // A tree is just a blob of index. then we save its hash
    let tree = Blob::new(config::INDEX);
    tree.write()?;

    let commit_content = format!("{}\n{}\n{}\n{}\n", tree.hash, "Diego Rocha", "...", msg);
    let mut hasher = Sha1::new();
    hasher.input_str(&commit_content);
    let result = hasher.result_str();

    let path = format!("{}/{}", config::COMMIT, &result[..2]);
    let mut path = PathBuf::from(&path);
    DirBuilder::new().create(&path)?;
    path.push(&result[2..]);
    let mut file = File::create(&path)?;
    file.write_all(commit_content.as_bytes())?;

    match to_branch(result) {
	Err(e) => panic!("{}", e),
	Ok(_) => (),
    }
    Ok(())
}
