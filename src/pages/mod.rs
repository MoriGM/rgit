pub mod repo;
pub mod statics;

use rocket::State;
use rocket::http::{Status, ContentType};

use tera::{Context, Tera};

use crate::config;

#[get("/")]
pub fn index(tera: &State<Tera>) -> (Status, (ContentType, String)) {
    let mut context = Context::new();
    context.insert("repos", &config::repo::get_repos().repos);
    
    (Status::Ok, (ContentType::HTML, tera.render("main.html", &context).unwrap()))
}