pub mod config;
mod pages;
pub mod repo;
mod tests;

#[macro_use]
extern crate rocket;

use clap::Parser;
use tera::Tera;

#[derive(Parser)]
struct Cli {
    #[clap(short, default_value = "8000")]
    port: String,
    #[clap(short, long)]
    repos_config: Option<std::path::PathBuf>,
}

pub struct Config {
    path: Option<std::path::PathBuf>,
}

#[launch]
fn rocket() -> _ {
    let cli = Cli::parse();

    let mut tera = Tera::default();
    tera.add_raw_template("main.html", include_str!("../templates/main.html"))
        .unwrap();
    tera.add_raw_template("repo.html", include_str!("../templates/repo.html"))
        .unwrap();

    let config = rocket::Config::figment().merge(("port", cli.port.parse::<i16>().unwrap()));

    rocket::custom(config)
        .manage(tera)
        .manage(Config{path: cli.repos_config})
        .mount("/", routes![pages::index])
        .mount("/repo", routes![pages::repo::index])
        .mount("/statics", routes![pages::statics::index])
        .mount("/", routes![pages::statics::img::favicon])

}
