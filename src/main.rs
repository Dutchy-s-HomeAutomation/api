use crate::environment::Environment;
use crate::database::Database;
use tera::Tera;
use actix_web::{HttpServer, App};
use crate::appdata::AppData;

mod environment;
mod database;
mod appdata;
mod endpoints;
mod authentication;
mod threads;

#[actix_web::main]
async fn main() {
    println!("Starting application.");

    let database = Database::new();

    let mut tera = Tera::new("templates/**/*").expect("Unable to initialize Tera!");
    tera.autoescape_on(vec![]);

    let appdata = AppData { database, tera };

    HttpServer::new(move || {
        App::new()
            .data(appdata)
            .service(actix_files::Files::new("/static", "/var/www/static/")
                .show_files_listing()
                .index_file("index.html")
                .disable_content_disposition()
            )
    });
}