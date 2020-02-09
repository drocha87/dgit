#![allow(warnings, allow_unused)]

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::env;
use std::fs::{self, ReadDir, DirBuilder, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use super::blob;
use super::config;
use super::commit;

fn mkdir_or_exit(parent: &mut PathBuf, dir: &str) {
    parent.push(dir);
    match DirBuilder::new().create(&parent) {
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
        Ok(_) => (),
    };
    parent.pop();
}

fn mkfile_or_exit(parent: &mut PathBuf, file: &str) {
    parent.push(file);
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&parent)
    {
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
        Ok(_) => (),
    };
    parent.pop();
}

// Create the dgit directory tree
pub fn init_current_dir() {
    match env::current_dir() {
        Err(e) => panic!("{}", e),
        Ok(ref mut dir) => {
            mkdir_or_exit(dir, config::ROOT);
            dir.push(config::ROOT);
            mkdir_or_exit(dir, config::BLOB);
            mkdir_or_exit(dir, config::COMMIT);

            mkfile_or_exit(dir, config::HEAD);
            mkfile_or_exit(dir, config::INDEX);
            mkfile_or_exit(dir, config::IGNORE);
        }
    }
}

pub fn exists_file(dir: &str, sha: &str) -> bool {
    let full_path = format!("{}/{}/{}", dir, &sha[..2], &sha[2..]);
    Path::new(&full_path).exists()
}

pub fn root_pathbuf_from(f: &str) -> PathBuf {
    let mut path = PathBuf::from(config::ROOT);
    path.push(f);
    path
}

pub fn sha1_string(s: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&s);
    hasher.result_str()
}

pub fn sha1_from_file(file: &str) -> String {
    match fs::read_to_string(file) {
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1);
        }
        Ok(s) => {
            let mut hasher = Sha1::new();
            hasher.input_str(&s);
            return hasher.result_str();
        }
    }
}

pub fn hasher(content: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&content);
    return hasher.result_str();
}

pub fn update_head(branch: String) {
    let path = root_pathbuf_from(config::HEAD);
    match OpenOptions::new().write(true).open(path) {
        Err(err) => {
            eprintln!("Error head: {:?}", err);
            exit(1);
        }
        Ok(mut file) => {
            file.write_all(branch.as_bytes())
                .expect("Head: Cannot write in head");
        }
    }
}

pub fn commit_path() -> PathBuf {
    root_pathbuf_from(config::COMMIT)
}

pub fn log(short: bool) {
    let path = commit_path();

    match fs::read_dir(path) {
	Ok(entries) => {
	    for entry in entries {
		if let Ok(entry) = entry {
		    let path = entry.path();
		    let path = path.file_name().unwrap();
		    let path = path.to_str().unwrap();
		    let commit = commit::Commit::new_from(&path);

		    if short {
			print!("{}: ", &path[..8]);
		    } else {
			println!("Commit: {}", path);
		    }
		    commit.print(short);
		}
	    }
	}
	Err(err) => {
	    eprintln!("Error log: {:?}", err);
	    exit(1);
	}
    }
}
