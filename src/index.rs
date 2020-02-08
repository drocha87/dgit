use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use super::blob;
use super::config;
use super::util;

pub struct Index {
    entries: Vec<(String, String)>,
}

impl Index {
    pub fn new() -> Self {
        let mut index = Index {
            entries: Vec::new(),
        };
        match File::open(util::root_pathbuf_from(config::INDEX)) {
            Err(e) => panic!("index corrupted: {}", e),
            Ok(file) => {
                let reader = BufReader::new(file);
                for mut line in reader.lines() {
                    match line {
                        Err(e) => panic!("Cannot read index: {}", e),
                        Ok(ref l) => {
                            let mut fname = l.to_string();
                            let sha = fname.split_off(fname.len() - 40);
                            index.entries.push((fname, sha));
                        }
                    }
                }
            }
        }
        index
    }

    pub fn ls(&self) {
        self.entries
            .iter()
            .for_each(|(fname, _)| println!("{}", fname));
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = OpenOptions::new()
            .write(true)
            .open(util::root_pathbuf_from(config::INDEX))?;
        for (f, h) in self.entries.iter() {
            let content = format!("{}{}\n", f, h);
            index.write(content.as_bytes())?;
        }
        Ok(())
    }

    // Add a Blob to index, if file exists check if it changed in disk
    // otherwise we have not to do
    // TODO: write better return type, we should return a Result with error implemented
    pub fn add(&mut self, blob: &blob::Blob) {
        for (fname, hash) in self.entries.iter_mut() {
            // Blob already add to index, check if it changed on disk
            if blob.fname == *fname {
                if blob.hash == *hash {
                    println!("File {} is up to date", fname);
                    return;
                }
                println!("Added modifications to file {}", fname);

                // File changed on disk, update content and hash
                *hash = blob.hash.clone();
                blob.write();
                self.write();
                return;
            }
        }
        self.entries.push((blob.fname.clone(), blob.hash.clone()));
        blob.write();
        self.write();
    }
}
