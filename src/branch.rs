use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use super::config;
use super::util;

#[derive(Deserialize, Serialize)]
pub struct Branch {
    pub head: String,
    pub branchs: BTreeMap<String, String>,
}

impl Branch {
    pub fn exists() -> bool {
        let path = util::root_pathbuf_from(config::BRANCH);
        path.exists()
    }

    pub fn init() -> Self {
        let content = fs::read_to_string(util::root_pathbuf_from(config::BRANCH))
            .map_err(util::exit_err)
            .unwrap();
        serde_json::from_str(&content)
            .map_err(util::exit_err)
            .unwrap()
    }

    pub fn head(&self) -> String {
	self.branchs.get(&self.head).unwrap().clone()
    }
    
    pub fn new(&mut self, name: String) {
        self.branchs.insert(name, self.head());
        self.write();
    }

    pub fn update(&mut self, branch: String) {
        self.branchs.insert(self.head.clone(), branch.clone());
        self.write();
    }

    pub fn write(&self) {
        let mut index = OpenOptions::new()
            .write(true)
            .open(util::root_pathbuf_from(config::BRANCH))
            .map_err(util::exit_err)
            .unwrap();
        let content = serde_json::to_string(&self).unwrap();
        let _ = index.write_all(content.as_bytes()).map_err(util::exit_err);
    }
}
