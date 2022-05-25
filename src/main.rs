pub mod repo;
mod config;

#[macro_use] extern crate rocket;

use rocket::State;
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
fn index(tera: &State<Tera>) -> (Status, (ContentType, String)) {
    let mut context = Context::new();
    context.insert("repos", &config::repo::get_repos().repos);
    
    (Status::Ok, (ContentType::HTML, tera.render("main.html", &context).unwrap()))
}

#[get("/<path>/<file>")]
fn web_static(_tera: &State<Tera>, path: &str, file: &str) -> (Status, (ContentType, &'static str)) {
    if path == "css" && file == "main.css" {
        return (Status::Ok, (ContentType::HTML, include_str!("../static/css/main.css")));
    }
    
    (Status::NotFound, (ContentType::HTML, ""))
}

#[get("/<repo>")]
fn web_repo(tera: &State<Tera>, repo: &str) -> (Status, (ContentType, String)) {
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

#[launch]
fn rocket() -> _ {
    let cli = Cli::parse();
    
    let mut config = rocket::Config::default();
    config.port = cli.port.parse::<u16>().unwrap();
    
    let mut tera = Tera::default();
    tera.add_raw_template("main.html", include_str!("../templates/main.html")).unwrap();
    tera.add_raw_template("repo.html", include_str!("../templates/repo.html")).unwrap();

    rocket::build()
        .manage(tera)
        .mount("/", routes![index])
        .mount("/repo", routes![web_repo])
        .mount("/static", routes![web_static])
}