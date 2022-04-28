mod repo;
mod config;

#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType};
use clap::Parser;
use tera::{Context, Tera};

use crate::repo::GitRepo;

#[derive(Parser)]
struct Cli {
    #[clap(short, default_value = "8000")]
    port: String
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<repo>")]
fn web_repo(repo: &str) -> (Status, (ContentType, String)) {
    let repos = match config::repo::get_repos().repos {
        Some(repos) => repos,
        None => panic!("Missing no repo is registered in the rgit_repos.toml")
    };
    
    let repo = String::from(repo);
    
    let repo_path = repos.iter().find(|config_repo| config_repo.name.as_ref().expect("[[repos]] in rgit_repos.toml is missing name") == &repo);
    
    if repo_path.is_none() {
        return (Status::NotFound, (ContentType::HTML, String::new()));
    } 
    
    let repo_path = repo_path.unwrap().path.as_ref().expect("[[repos]] in rgit_repos.toml is missing its path");
    
    let repo = GitRepo::new(repo_path.as_str());
    
    let mut tera = Tera::default();
    tera.add_raw_template("repo.html", include_str!("../templates/repo.html")).unwrap();
    
    let mut context = Context::new();
    context.insert("commits", &repo.unwrap().logs());
    
    (Status::Ok, (ContentType::HTML, tera.render("repo.html", &context).unwrap()))
}

#[launch]
fn rocket() -> _ {
    let cli = Cli::parse();
    
    let mut config = rocket::Config::default();
    config.port = cli.port.parse::<u16>().unwrap();

    rocket::build()
        .mount("/", routes![index])
        .mount("/repo", routes![web_repo])
}