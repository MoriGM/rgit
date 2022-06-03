use crate::repo::GitRepo;

#[test]
fn test_repo_create() {
    let repo = GitRepo::new("./");
    
    assert_eq!(repo.is_ok(), true);
}

#[test]
fn test_repo_has_commits() {
    let repo = GitRepo::new("./");
    
    assert_eq!(repo.is_ok(), true);
    
    let repo = repo.unwrap();
    
    let commits = repo.logs();
    assert_eq!(commits.len() > 0, true);
}

#[test]
fn test_repo_refresh() {
    let repo = GitRepo::new("./");
    
    assert_eq!(repo.is_ok(), true);
    
    let repo = repo.unwrap().refresh();
    
    assert_eq!(repo.is_ok(), true);
}

#[test]
fn test_repo_refresh_has_commits() {
    let repo = GitRepo::new("./");
    
    assert_eq!(repo.is_ok(), true);
    
    let repo = repo.unwrap().refresh();
    
    assert_eq!(repo.is_ok(), true);
    
    let repo = repo.unwrap();
    
    let commits = repo.logs();
    assert_eq!(commits.len() > 0, true);
}