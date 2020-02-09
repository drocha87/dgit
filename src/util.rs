use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::env;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use super::commit;
use super::config;

pub fn exit_err<E>(err: E)
where
    E: core::fmt::Display,
{
    eprintln!("Error: {}", err);
    exit(1);
}

fn mkdir_or_exit(parent: &mut PathBuf, dir: &str) {
    parent.push(dir);
    let _x = DirBuilder::new().create(&parent).map_err(exit_err);
    parent.pop();
}

fn mkfile_or_exit(parent: &mut PathBuf, file: &str) {
    parent.push(file);
    let _x = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&parent)
        .map_err(exit_err);
    parent.pop();
}

// Create the dgit directory tree
pub fn init_current_dir() {
    match env::current_dir() {
        Err(err) => exit_err(err),
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

pub fn hasher(content: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(content);
    return hasher.result_str();
}

pub fn sha1_from_file(file: &str) -> String {
    let content = fs::read_to_string(file).map_err(exit_err).unwrap();
    hasher(&content)
}

pub fn update_head(branch: String) {
    let path = root_pathbuf_from(config::HEAD);
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(exit_err)
        .unwrap();
    let _ = file.write_all(branch.as_bytes()).map_err(exit_err);
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
