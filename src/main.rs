#![allow(dead_code)]

use std::env;

mod blob;
mod commit;
mod config;
mod index;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: dgit options");
    } else {
        match args[1].as_str() {
            "init" => {
		println!("Initializing in current directory");
		util::init_current_dir();
	    }

            "add" => {
                let mut index = index::Index::new();
                let blob = blob::Blob::new(&args[2]);
                index.add(&blob);
            }

            "ls-index" => {
                let index = index::Index::new();
                index.ls();
            }

            "commit" => {
                let msg = &args[2];
                let commit = commit::Commit::new(msg.to_string());
                commit.write();
            }

            "commit-tree" => {
                let commit = commit::Commit::new_from(&args[2]);
                commit.tree.iter().for_each(|(f, h)| println!("{} {}", f, h));
            }

            "status" => {
                let index = index::Index::new();
                index.status();
            }

	    "log-short" => util::log(true),
	    "log" => util::log(false),
	    
            _ => println!("Invalid Option"),
        }
    }
}
