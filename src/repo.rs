use git2::Repository;
use serde::{Serialize, Deserialize};

pub struct GitRepo {
    repo: Repository
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub oid :String,
    pub message: String
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
            let oid_raw = rev.unwrap();
            let oid = format!("{}", oid_raw);
            let commit = self.repo.find_commit(oid_raw);
            let message = match commit.unwrap().message() {
                Some(message) => message.to_string(),
                None => "".to_string()
            };
            commits.push(GitCommit{oid, message});
        });
        
        commits
    }
}