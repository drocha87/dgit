use std::fs::{self, DirBuilder, File};
use std::io::{self, Write};

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use super::config;
use super::util;

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

    fn exists(&self) -> bool {
        util::exists_file(config::BLOB, &self.hash)
    }

    pub fn write(&self) -> io::Result<()> {
        if !self.exists() {
            let mut path = util::root_pathbuf_from(config::BLOB);
	    path.push(&self.hash[..2]);
            DirBuilder::new().create(&path)?;
            path.push(&self.hash[2..]);
            let mut file = File::create(&path)?;
            file.write_all(self.content.as_bytes())?;
        }
        Ok(())
    }
}
