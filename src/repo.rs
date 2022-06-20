use std::cmp;

use git2::{BranchType, Repository};
use serde::{Deserialize, Serialize};

pub struct GitRepo {
    repo: Repository,
    branch: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub oid: String,
    pub message: String,
    pub author: String,
}

impl GitRepo {
    pub fn new(path: &str) -> Result<GitRepo, String> {
        let repo = Repository::open(path);

        if repo.is_err() {
            return Err("Canno't find repo in path".to_string());
        }

        let repo = repo.unwrap();

        Ok(GitRepo {
            repo,
            branch: String::from("master"),
        })
    }

    pub fn change_branch(&mut self, name: &str) -> bool {
        let branch = self.repo.find_branch(name, BranchType::Local);

        if branch.is_err() {
            return false;
        }

        self.branch = String::from(name);

        true
    }

    pub fn refresh(&self) -> Result<GitRepo, String> {
        let repo = Repository::open(self.repo.path());

        if repo.is_err() {
            return Err("Canno't find repo in path".to_string());
        }

        let repo = repo.unwrap();

        Ok(GitRepo {
            repo,
            branch: self.branch.clone(),
        })
    }

    pub fn logs(&self) -> std::vec::Vec<GitCommit> {
        let mut revs = match self.repo.revwalk() {
            Ok(rev) => rev,
            Err(e) => panic!("failed to init: {}", e),
        };

        let mut commits = Vec::new();

        let oid = self
            .repo
            .find_branch(self.branch.as_ref(), BranchType::Local)
            .unwrap()
            .into_reference()
            .target()
            .unwrap();
            
        revs.push(oid).unwrap();

        revs.for_each(|rev| {
            if rev.is_err() {
                return;
            }

            let oid_raw = rev.unwrap();
            let oid = format!("{}", oid_raw);
            let commit = self.repo.find_commit(oid_raw).unwrap();
            let message = match commit.message() {
                Some(message) => message.to_string(),
                None => "".to_string(),
            };
            let author = commit.author().to_string();
            commits.push(GitCommit {
                oid,
                message,
                author,
            });
        });

        commits
    }

    pub fn last_update(&self) -> i64 {
        let mut revs = match self.repo.revwalk() {
            Ok(rev) => rev,
            Err(e) => panic!("failed to init: {}", e),
        };

        revs.push_head().unwrap();

        let mut latest = 0;

        revs.for_each(|rev| {
            let commit = self.repo.find_commit(rev.unwrap()).unwrap();

            latest = cmp::max(latest, commit.time().seconds());
        });

        latest
    }
    
    pub fn branches(&self) -> Vec<String> {
        let mut vec = vec![];
        
        self.repo.branches(None).unwrap().for_each(|branch| {
            let branch = branch.unwrap();
            
            if branch.1 != BranchType::Local {
                return;
            }
            
            vec.push(String::from(branch.0.name().unwrap().unwrap()));
        });
        
        return vec;
    }
}
