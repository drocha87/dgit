#![allow(dead_code)]

use std::env;
use std::fs;
use std::fs::File;
use std::io;

mod config;
mod util;

fn init() {
    match fs::create_dir("./.dgit") {
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("Directory already initialized")
        }
        Err(e) => println!("{}", e),

        // Iff new initialization lets create all directories tree
        _ => {
            match fs::create_dir(config::OBJECTS) {
                Err(e) => println!("{}", e),
                _ => (),
            };
            match File::create(config::HEAD) {
                Err(e) => println!("{}", e),
                _ => (),
            };
	    match File::create(config::INDEX) {
                Err(e) => println!("{}", e),
                _ => (),
            };
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
	"init" => init(),

	"add" => {
	    let mut index = util::Index::new();
	    let blob = util::Blob::new(&args[2]);
	    index.add(&blob);
	}

	"ls-index" => {
	    let index = util::Index::new();
	    index.ls();
	}

	_ => println!("Invalid Option"),
    }
}
