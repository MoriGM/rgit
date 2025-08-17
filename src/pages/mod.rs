pub mod repo;
pub mod statics;

use rocket::http::{ContentType, Status};
use rocket::State;

use tera::{Context, Tera};

use crate::config;

#[get("/")]
pub fn index(repos_file: &State<crate::Config>, tera: &State<Tera>) -> (Status, (ContentType, String)) {
    let mut context = Context::new();
    context.insert("repos", &config::repo::get_repos(repos_file.path.as_deref()).repos);

    (
        Status::Ok,
        (
            ContentType::HTML,
            tera.render("main.html", &context).unwrap(),
        ),
    )
}
