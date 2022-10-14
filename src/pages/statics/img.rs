use rocket::http::{ContentType, Status};
use rocket::State;

use tera::Tera;

#[get("/favicon.ico")]
pub fn favicon<'a>(_tera: &State<Tera>) -> (Status, (ContentType, &'a [u8])) {
    (
        Status::Ok,
        (
            ContentType::Icon,
            include_bytes!("../../../statics/img/favicon.ico"),
        ),
    )
}