use std::fs::{self, File};
use std::io::Write;

use super::config;
use super::util;

pub struct Blob {
    pub fname: String,
    pub hash: String,
    pub content: String,
}

impl Blob {
    pub fn new(file: &str) -> Self {
        let content = fs::read_to_string(file).map_err(util::exit_err).unwrap();

        Blob {
            fname: file.to_string(),
            hash: util::hasher(&content),
            content: content,
        }
    }

    fn exists(&self) -> bool {
        util::exists_file(config::BLOB, &self.hash)
    }

    pub fn write(&self) {
        if !self.exists() {
            let mut path = util::root_pathbuf_from(config::BLOB);
            path.push(&self.hash);
            let mut file = File::create(&path).map_err(util::exit_err).unwrap();
            let _ = file
                .write_all(self.content.as_bytes())
                .map_err(util::exit_err);
        }
    }
}
