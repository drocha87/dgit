use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::process::exit;

use super::config;
use super::index;
use super::util;

#[derive(Deserialize, Serialize)]
pub struct Commit {
    author: String,
    date: String,
    pub tree: BTreeMap<String, String>,
    message: String,
}

impl Commit {
    pub fn new(message: String) -> Self {
        // Read the index and copy the tree structure
        let index = index::Index::new();
        Commit {
            author: "Diego Rocha".to_string(),
            date: Local::now().to_string(),
            tree: index.entries.clone(),
            message,
        }
    }

    pub fn new_from(file: &str) -> Self {
        let mut path = util::root_pathbuf_from(config::COMMIT);
        path.push(&file);

        let content = fs::read_to_string(&path).map_err(util::exit_err).unwrap();
        serde_json::from_str(&content)
            .map_err(util::exit_err)
            .unwrap()
    }

    pub fn write(&self) {
        // TODO: handle error
        let content = serde_json::to_string(&self).unwrap();
        let hash = util::hasher(&content);
        let mut path = util::root_pathbuf_from(config::COMMIT);
        path.push(&hash);

        if path.exists() {
            println!("Commit: already exists {}", &hash[..8]);
            exit(1);
        }

        let mut file = File::create(&path).map_err(util::exit_err).unwrap();
        let _x = file.write_all(content.as_bytes()).map_err(util::exit_err);

        util::update_head(hash);
    }

    pub fn print(&self, short: bool) {
        if short {
            println!("{}", self.message);
        } else {
            println!("Author: {}", self.author);
            println!("Date: {}", self.date);
            println!("Files:");
            for (f, h) in &self.tree {
                println!("\t{} {}", f, h);
            }
            println!("\nMessage:\n\t{}\n", self.message);
        }
    }
}
