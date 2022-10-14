pub mod img;

use rocket::http::{ContentType, Status};
use rocket::State;

use tera::Tera;

#[get("/<path>/<file>")]
pub fn index<'a>(_tera: &State<Tera>, path: &str, file: &str) -> (Status, (ContentType, &'a str)) {
    if path == "css" && file == "main.css" {
        return (
            Status::Ok,
            (
                ContentType::CSS,
                include_str!("../../../statics/css/main.css"),
            ),
        );
    }
    
    if path == "js" && file == "repo.js" {
        return (
            Status::Ok,
            (
                ContentType::JavaScript,
                include_str!("../../../statics/js/repo.js"),
            ),
        );
    }

    (Status::NotFound, (ContentType::HTML, ""))
}
