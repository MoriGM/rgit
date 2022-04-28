use git2::Repository;
use serde::{Serialize, Deserialize};

pub struct GitRepo {
    repo: Repository
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub oid :String
}

impl GitRepo {
    pub fn new(path: &str) -> Result<GitRepo, String> {
        let repo = Repository::open(path);
        
        if repo.is_err() {
           return Err("Canno't find repo in path".to_string());
        }
        
        let repo = repo.unwrap();
        
        Ok(GitRepo{repo})
    }
    
    pub fn refresh(&self) -> Result<GitRepo, String> {
        let repo = Repository::open(self.repo.path());
        
        if repo.is_err() {
           return Err("Canno't find repo in path".to_string());
        }
        
        let repo = repo.unwrap();
        
        Ok(GitRepo{repo})
    }
    
    pub fn logs(&self) -> std::vec::Vec<GitCommit> {
        let mut revs = match self.repo.revwalk() {
            Ok(rev) => rev,
            Err(e) => panic!("failed to init: {}", e)
        };
        
        revs.push_head().unwrap();
        
        let mut commits = Vec::new();
        
        revs.for_each(|rev| {
            commits.push(GitCommit{oid: format!("{}", rev.unwrap())});
        });
        
        commits
    }
}