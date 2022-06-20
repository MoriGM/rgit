use rocket::http::{ContentType, Status};
use rocket::State;

use tera::{Context, Tera};

use crate::config;
use crate::repo::GitRepo;

#[get("/<repo>/<branch>")]
pub fn index(tera: &State<Tera>, repo: &str, branch: &str) -> (Status, (ContentType, String)) {
    let repos = config::repo::get_repos().repos;

    let repo = String::from(repo);

    let repo_path = repos
        .iter()
        .find(|config_repo| config_repo.info.name == repo);

    if repo_path.is_none() {
        return (Status::NotFound, (ContentType::HTML, String::new()));
    }

    let mut repo = GitRepo::new(repo_path.unwrap().info.path.as_str()).unwrap();

    repo.change_branch(branch);

    let mut context = Context::new();
    context.insert("commits", &repo.logs());
    context.insert("branches", &repo.branches());

    (
        Status::Ok,
        (
            ContentType::HTML,
            tera.render("repo.html", &context).unwrap(),
        ),
    )
}
