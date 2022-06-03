pub mod repo;
pub mod config;
mod pages;

#[macro_use] extern crate rocket;

use clap::Parser;
use tera::Tera;

#[derive(Parser)]
struct Cli {
    #[clap(short, default_value = "8000")]
    port: String
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
        .mount("/", routes![pages::index])
        .mount("/repo", routes![pages::repo::index])
        .mount("/statics", routes![pages::statics::index])
}