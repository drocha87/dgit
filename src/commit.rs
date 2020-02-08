// Commit file model
//
// tree-hash
// author:
// date:
// Message:
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::exit;

use super::config;
use super::util;
use super::tree;

pub fn to_branch(branch: String) -> io::Result<()> {
    let path = util::root_pathbuf_from(config::HEAD);
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.write_all(branch.as_bytes())?;
    Ok(())
}

pub fn commit(msg: String) -> io::Result<()> {
    let tree = tree::write_tree();

    let commit_content = format!("{}\n{}\n{}\n{}\n", tree, "Diego Rocha", "...", msg);
    let hash = util::sha1_string(&commit_content);

    let mut path = util::root_pathbuf_from(config::COMMIT);
    path.push(&hash);

    if path.exists() {
	println!("Commit: already exists {}", &hash[..8]);
	exit(1);
    }

    match File::create(&path) {
	Err(err) => {
	    eprintln!("Error: {:?}", err);
	    exit(1);
	}
	Ok(mut file) => {
	    match file.write_all(commit_content.as_bytes()) {
		Err(err) => {
		    eprintln!("Error: {:?}", err);
		    exit(1);
		}
		Ok(_) => (),
	    }
	}
    }
    // match to_branch(result) {
    //     Err(e) => {
    // 	    eprintln!("Error: {:?}", e);
    // 	    exit(1);
    // 	}
    //     Ok(_) => (),
    // }
    Ok(())
}
