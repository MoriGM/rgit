use rocket::http::{ContentType, Status};
use rocket::State;

use tera::{Context, Tera};

use crate::config;
use crate::repo::GitRepo;

#[get("/<repo_name>/<branch>")]
pub fn index(
    repo_file: &State<crate::Config>,
    tera: &State<Tera>,
    repo_name: &str,
    branch: &str,
) -> (Status, (ContentType, String)) {
    let repos = config::repo::get_repos(repo_file.path.as_deref()).repos;

    let repo_path = repos
        .iter()
        .find(|config_repo| config_repo.info.name == repo_name);

    if repo_path.is_none() {
        return (Status::NotFound, (ContentType::HTML, String::new()));
    }

    let mut repo = GitRepo::new(repo_path.unwrap().info.path.as_str()).unwrap();

    repo.change_branch(branch);

    let mut context = Context::new();
    context.insert("path", &format!("/repo/{}/", repo_name));
    context.insert("commits", &repo.logs());
    context.insert("branches", &repo.branches());
    context.insert("branchName", &branch);

    (
        Status::Ok,
        (
            ContentType::HTML,
            tera.render("repo.html", &context).unwrap(),
        ),
    )
}
