#[macro_use] extern crate magic_crypt;

mod environment;
mod database;
mod appdata;
mod endpoints;
mod threads;
mod types;
mod services;
mod common;
mod config;

use crate::database::Database;
use crate::appdata::AppData;

use tera::Tera;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Welcome to ConnectedHome");

    let database = Database::new();

    let oauth_credentials = config::get_oauth_credentials(&database);
    let services_configs = config::read_services_conf();

    let mut tera = Tera::new("templates/**/*").expect("Unable to initialize Tera!");
    tera.autoescape_on(vec![]);

    let appdata = AppData { database, tera, oauth_credentials, services_configs};

    HttpServer::new(move || {

        App::new()
            .data(appdata.clone())
            //Static pages
            .service(actix_files::Files::new("/static", "./static")
                .show_files_listing()//TODO This should only be enabled during development
                .index_file("index.html")
            )

            //Authentication endpoints
            .service(endpoints::auth::login::post_login)
            .service(endpoints::auth::register::post_register)
            .service(endpoints::auth::session::post_session)

            //OAuth endpoints
            .service(endpoints::oauth::login::get_login)
            .service(endpoints::oauth::finish::post_finish)
            .service(endpoints::oauth::token::post_token)

            //Service endpoints
            .service(endpoints::services::add::post_add)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}