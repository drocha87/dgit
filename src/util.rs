#![allow(warnings, allow_unused)]

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::env;
use std::fs;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use super::blob;
use super::config;

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
            mkdir_or_exit(dir, config::TREE);
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
