use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CofigRepos {
    pub repos: Option<Vec<CofigRepo>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CofigRepo {
    pub name: Option<String>,
    pub path: Option<String>
}

pub fn get_repos() -> CofigRepos {
    let content = match fs::read_to_string("rgit_repos.toml") {
        Ok(content) => content,
        Err(_) => panic!("File is missing!")
    };
    toml::from_str(content.as_str()).expect("Content is incorrect!")
}