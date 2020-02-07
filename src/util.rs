#![allow(warnings, allow_unused)]

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use super::config;

pub struct Index {
    entries: Vec<(String, String)>,
}

impl Index {
    pub fn new() -> Self {
        let mut index = Index {
            entries: Vec::new(),
        };
        match File::open(config::INDEX) {
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
        let mut index = OpenOptions::new().write(true).open(config::INDEX)?;
	for (f, h) in self.entries.iter() {
            let content = format!("{}{}\n", f, h);
            index.write(content.as_bytes())?;
        };
        Ok(())
    }

    // Add a Blob to index, if file exists check if it changed in disk
    // otherwise we have not to do
    // TODO: write better return type, we should return a Result with error implemented
    pub fn add(&mut self, blob: &Blob) {
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

pub struct Blob {
    pub fname: String,
    pub hash: String,
    pub content: String,
}

impl Blob {
    pub fn new(file: &str) -> Self {
        let blob: Blob;

        match fs::read_to_string(file) {
            Err(e) => panic!("{}", e),
            Ok(s) => {
                let mut hasher = Sha1::new();
                hasher.input_str(&s);

                blob = Blob {
                    fname: file.to_string(),
                    hash: hasher.result_str(),
                    content: s,
                }
            }
        }
        blob
    }

    fn check_object_exists(&self) -> bool {
        let full_path = format!(
            "{}/{}/{}",
            config::OBJECTS,
            &self.hash[..2],
            &self.hash[2..]
        );
        Path::new(&full_path).exists()
    }

    pub fn write(&self) -> io::Result<()> {
        if !self.check_object_exists() {
            let path = format!("{}/{}", config::OBJECTS, &self.hash[..2]);
            let mut path = PathBuf::from(&path);
            DirBuilder::new().create(&path)?;
            path.push(&self.hash[2..]);
            let mut file = File::create(&path)?;
            file.write_all(self.content.as_bytes())?;
        }
        Ok(())
    }
}
