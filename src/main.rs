#![allow(dead_code)]

use std::env;

mod blob;
mod commit;
mod config;
mod index;
mod tree;
mod util;

// fn init() {
//     match fs::create_dir("./.dgit") {
//         Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
//             println!("Directory already initialized")
//         }
//         Err(e) => println!("{}", e),

//         // Iff new initialization lets create all directories tree
//         _ => {
//             match fs::create_dir(config::OBJECTS) {
//                 Err(e) => println!("{}", e),
//                 _ => (),
//             };
//             match File::create(config::HEAD) {
//                 Err(e) => println!("{}", e),
//                 _ => (),
//             };
// 	    match File::create(config::INDEX) {
//                 Err(e) => println!("{}", e),
//                 _ => (),
//             };
//         }
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => util::init_current_dir(),

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
            commit::commit(msg.to_string());
        }

        "write-tree" => {
            tree::write_tree();
        }

        _ => println!("Invalid Option"),
    }
}
