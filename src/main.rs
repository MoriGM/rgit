pub mod repo;

#[macro_use] extern crate rocket;

use rocket::http::{Status, ContentType};

use crate::repo::GitRepo;

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
    rocket::build()
        .mount("/", routes![index])
        .mount("/repo", routes![web_repo])
}