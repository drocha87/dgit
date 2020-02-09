use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::exit;

use super::config;
use super::index;
use super::util;

#[derive(Deserialize, Serialize)]
pub struct Commit {
    author: String,
    date: String,
    pub tree: Vec<String>,
    message: String,
}

impl Commit {
    pub fn new(message: String) -> Self {
        // Read the index and copy the tree structure
        let index = index::Index::new();
        let tree = index.entries.iter().map(|(_, val)| val.clone()).collect();
        Commit {
            author: "Diego Rocha".to_string(),
            date: Local::now().to_string(),
            tree,
            message,
        }
    }

    pub fn new_from(file: &str) -> Self {
        let mut path = util::root_pathbuf_from(config::COMMIT);
        path.push(&file);

        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(commit) => {
                    return commit;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    exit(1);
                }
            },
            Err(err) => {
                eprintln!(
                    "Can't read commit \"{:?}\"\nError message: {:?}",
                    path,
                    err.kind()
                );
                exit(1);
            }
        }
    }

    pub fn write(&self) {
        // TODO: handle error
        let content = serde_json::to_string(&self).unwrap();
        let hash = util::sha1_string(&content);
        let mut path = util::root_pathbuf_from(config::COMMIT);
        path.push(&hash);

        if path.exists() {
            println!("Commit: already exists {}", &hash[..8]);
            exit(1);
        }

        match File::create(&path) {
            Err(err) => {
                eprintln!("Error: {:?}", err);
                exit(1);
            }
            Ok(mut file) => match file.write_all(content.as_bytes()) {
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    exit(1);
                }
                Ok(_) => (),
            },
        }
        util::update_head(hash);
    }

    pub fn print(&self, short: bool) {
        if short {
            println!("{}", self.message);
        } else {
            println!("Author: {}", self.author);
            println!("Date: {}", self.date);
            println!("Files:");
            for entry in &self.tree {
                println!("\t{}", entry);
            }
            println!("\nMessage:\n\t{}\n", self.message);
        }
    }
}
