use rocket::State;
use rocket::http::{Status, ContentType};

use tera::{Context, Tera};

use crate::config;
use crate::repo::GitRepo;

#[get("/<repo>/<_branch>")]
pub fn index(tera: &State<Tera>, repo: &str, _branch: &str) -> (Status, (ContentType, String)) {
    let repos = config::repo::get_repos().repos;
    
    let repo = String::from(repo);
    
    let repo_path = repos.iter().find(|config_repo| config_repo.info.name == repo);
    
    if repo_path.is_none() {
        return (Status::NotFound, (ContentType::HTML, String::new()));
    } 
    
    let repo = GitRepo::new(repo_path.unwrap().info.path.as_str());
    
    let mut context = Context::new();
    context.insert("commits", &repo.unwrap().logs());
    
    (Status::Ok, (ContentType::HTML, tera.render("repo.html", &context).unwrap()))
}