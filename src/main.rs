pub mod repo;

#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::Config;
use clap::Parser;

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
    let repo = GitRepo::new("/home/max/Code/rust/book/hello_world/");
    
    let mut body = String::from("");
    
    repo.unwrap().logs().for_each(|text| {
        body.push_str(text.as_str());
        body.push_str("<br/>");
    });
    
    (Status::Ok, (ContentType::HTML, body))
}

#[launch]
fn rocket() -> _ {
    let cli = Cli::parse();
    
    let mut config = Config::default();
    config.port = cli.port.parse::<u16>().unwrap();
    
    rocket::build()
        .mount("/", routes![index])
        .mount("/repo", routes![web_repo])
}