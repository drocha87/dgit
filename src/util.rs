use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{Utc, DateTime};

const OBJECTS_ROOT: &str = "./.dgit/objects";
const HEAD: &str = "./.dgit/HEAD";

pub struct Blob {
    pub dir: String,
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
                let result = hasher.result_str();

                blob = Blob {
                    dir: result[..2].to_string(),
                    hash: result[2..].to_string(),
                    content: s,
                }
            }
        }
        blob
    }

    fn check_object_exists(&self) -> bool {
        let full_path = format!("{}/{}/{}", OBJECTS_ROOT, self.dir, self.hash);
        Path::new(&full_path).exists()
    }

    pub fn write(&self) -> io::Result<()> {
        if !self.check_object_exists() {
            let mut path = PathBuf::new();
            path.push(OBJECTS_ROOT);
            path.push(self.dir.clone());

            DirBuilder::new().create(&path)?;
            path.push(self.hash.clone());
            let mut file = File::create(&path)?;
            file.write_all(self.content.as_bytes())?;

            // Add the object to HEAD
            let mut head = OpenOptions::new().append(true).open(HEAD)?;
            let content = format!("{} {}\n", self.dir, self.hash);
            head.write(content.as_bytes());
        } else {
            println!("Object up to date!");
        }
        Ok(())
    }
}
