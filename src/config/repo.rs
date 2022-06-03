use serde::{Deserialize, Serialize};

use std::fs;

use crate::repo::GitRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRepos {
    pub repos: Vec<ConfigRepo>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRepo {
    pub info: ConfigRepoInfo,
    pub last_update: i64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigRepoInfo {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigRepoInfos {
    pub repos: Vec<ConfigRepoInfo>
}

pub fn get_repos() -> ConfigRepos {
    let content = match fs::read_to_string("rgit_repos.toml") {
        Ok(content) => content,
        Err(_) => panic!("File is missing!")
    };
    
    let infos: ConfigRepoInfos = toml::from_str(content.as_str()).expect("Content is incorrect!");
    
    let mut repos = Vec::new();
    
    infos.repos.iter().for_each(|info| {
        let repo = GitRepo::new(info.path.as_str());
        repos.push(ConfigRepo{info: info.clone(), last_update: repo.unwrap().last_update()});
    });
    
    ConfigRepos{repos}
}