use rocket::State;
use rocket::http::{Status, ContentType};

use tera::Tera;

#[get("/<path>/<file>")]
pub fn index<'a>(_tera: &State<Tera>, path: &str, file: &str) -> (Status, (ContentType, &'a str)) {
    if path == "css" && file == "main.css" {
        return (Status::Ok, (ContentType::HTML, include_str!("../../../statics/css/main.css")));
    }
    
    (Status::NotFound, (ContentType::HTML, ""))
}