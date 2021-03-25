mod environment;
mod database;
mod appdata;
mod endpoints;
mod threads;
mod logic;

use crate::database::Database;
use crate::appdata::AppData;

use tera::Tera;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting application.");

    HttpServer::new(move || {
        let database = Database::new();
        let mut tera = Tera::new("templates/**/*").expect("Unable to initialize Tera!");
        tera.autoescape_on(vec![]);

        let appdata = AppData { database, tera };

        App::new()
            .data(appdata)
            //Static pages
            .service(actix_files::Files::new("/static", "./static")
                .show_files_listing()//TODO This should only be enabled during development
                .index_file("index.html")
            )

            //POST endpoints
            .service(endpoints::auth::login::post_login)
            .service(endpoints::auth::register::post_register)
            .service(endpoints::auth::session::post_session)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}