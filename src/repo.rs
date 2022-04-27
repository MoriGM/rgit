use git2::Repository;

pub struct GitRepo {
    repo: Repository
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
    
    pub fn logs(&self) -> std::vec::IntoIter<String> {
        let mut revs = match self.repo.revwalk() {
            Ok(rev) => rev,
            Err(e) => panic!("failed to init: {}", e)
        };
        
        revs.push_head().unwrap();
        
        let mut log_strings = vec![];
        
        revs.for_each(|rev| {
            log_strings.push(format!("{}", rev.unwrap()))
        });
        
        log_strings.into_iter()
    } 
}