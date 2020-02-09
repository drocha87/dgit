use serde::{Deserialize, Serialize};
use std::collections::btree_map::{BTreeMap, Entry::Occupied, Entry::Vacant};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process::exit;

use super::blob;
use super::config;
use super::util;

#[derive(Deserialize, Serialize)]
pub struct Index {
    pub entries: BTreeMap<String, String>,
}

impl Index {
    pub fn hash_index() -> String {
        let path = util::root_pathbuf_from(config::INDEX);
        match fs::read_to_string(&path) {
            Err(err) => panic!("{:?}", err),
            Ok(s) => util::sha1_string(&s),
        }
    }

    pub fn new() -> Self {
        let entries: BTreeMap<String, String>;
        match fs::read_to_string(util::root_pathbuf_from(config::INDEX)) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(map) => entries = map,
                Err(_) => entries = BTreeMap::new(),
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                exit(1);
            }
        }
        Self { entries }
    }

    pub fn update(&mut self, file: &str, hash: String) {
        let _x = self.entries.entry(file.to_string()).or_insert(hash);
    }

    pub fn ls(&self) {
        self.entries.keys().for_each(|fname| println!("{}", fname));
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = OpenOptions::new()
            .write(true)
            .open(util::root_pathbuf_from(config::INDEX))?;
        let content = serde_json::to_string(&self.entries).unwrap();
        index.write_all(content.as_bytes())?;
        Ok(())
    }

    // Add a Blob to index, if file exists check if it changed in disk
    // otherwise we have not to do
    // TODO: write better return type, we should return a Result with error implemented
    pub fn add(&mut self, blob: &blob::Blob) {
        match self.entries.entry(blob.fname.clone()) {
            Occupied(ref mut entry) => {
                let val = entry.get_mut();
                if *val != blob.hash {
                    println!("Updating {} in index", blob.fname);
                    *val = blob.hash.clone();
                } else {
                    // Ok, file is the same on disk and index, so just return
                    println!("{} is up to date", blob.fname);
                    return;
                }
            }
            Vacant(entry) => {
                entry.insert(blob.hash.clone());
            }
        }
        let _e = self.write();
        let _e = blob.write();
    }
}
