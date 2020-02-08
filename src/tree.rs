use std::fs;
use std::io;
use std::process::exit;

use super::config;
use super::index;
use super::util;

pub fn write_tree() -> String {
    let mut tree = util::root_pathbuf_from(config::TREE);
    let index = util::root_pathbuf_from(config::INDEX);
    let hash = index::Index::hash_index();

    tree.push(&hash);

    if tree.exists() {
        println!("Tree {} already exists", &hash[..8]);
	exit(1);
    } else {
        match fs::copy(index, tree) {
            Err(err) => {
                eprintln!("Tree: {:?}", err);
                exit(1);
            }
            Ok(_) => (),
        }
    }
    hash
}
