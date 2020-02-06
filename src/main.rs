use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

mod util;

fn check_init() -> bool {
    Path::new("./.dgit").exists()
}

fn init() {
    match fs::create_dir("./.dgit") {
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("Directory already initialized")
        }
        Err(e) => println!("{}", e),

        // Iff new initialization lets create all directories tree
        _ => {
            match fs::create_dir("./.dgit/objects") {
                Err(e) => println!("{}", e),
                _ => (),
            };
            match File::create("./.dgit/HEAD") {
                Err(e) => println!("{}", e),
                _ => (),
            };
        }
    }
}

fn main() {
    //    let args: Vec<String> = env::args().collect();
    //    let hex = hash_sha1(&args[1]);
    init();
    //add(&"Cargo.toml");
    //assert!(check_init());
    let blob = util::Blob::new(&"Cargo.toml");
    blob.write();
    //println!("{} -- {}\n{}", blob.dir, blob.hash, blob.content);
}
