mod environment;
mod database;
mod appdata;
mod endpoints;
mod threads;
mod logic;
mod types;

use crate::database::Database;
use crate::appdata::{AppData, OAuthCredentials};

use tera::Tera;
use actix_web::{HttpServer, App};
use mysql::{Row, Params, params};
use mysql::prelude::Queryable;
use rand::Rng;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting application.");

    let database = Database::new();

    //Initial configuration
    //Check if the oauth_credentials table contains the required data
    //For now this is only for:
    //- Google
    let conn_wrapped = database.pool.get_conn();
    if conn_wrapped.is_err() {
        eprintln!("Unable to establish connection to the Database: {:?}", conn_wrapped.err());
        std::process::exit(1);
    }

    let mut conn = conn_wrapped.unwrap();
    let google_credentials_query_result = conn.exec::<Row, &str, Params>("SELECT client_id,client_secret FROM oauth_credentials WHERE identifier = :identifier", params!{
        "identifier" => "GOOGLE"
    });

    if google_credentials_query_result.is_err() {
        eprintln!("Unable to query oauth_credentials table: {:?}", google_credentials_query_result.err());
        std::process::exit(1);
    }

    let result_unwrapped = google_credentials_query_result.unwrap();
    let (client_id, client_secret) = if result_unwrapped.len() == 0 {
        println!("No OAuth credentials found for 'GOOGLE'. Preparing (You should note these, you need them!)");
        let client_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect();
        let client_secret: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect();

        //Insert the newly generated credentials into the database
        let sql_insert = conn.exec::<usize, &str, Params>("INSERT INTO oauth_credentials (client_id, client_secret, identifier) VALUES (:client_id, :client_secret, 'GOOGLE')", params! {
            "client_id" => client_id.clone(),
            "client_secret" => client_secret.clone()
        });

        if sql_insert.is_err() {
            eprintln!("Unable to insert OAuth credentials into the Database: {:?}", sql_insert.err());
            std::process::exit(1);
        }

        println!("Your OAuth credentials for 'GOOGLE': \nclient_id: {client_id} \nclient_secret: {client_secret}",
            client_id = client_id.clone(),
            client_secret = client_secret.clone()
        );

        (client_id, client_secret)
    } else {
        let row = result_unwrapped.get(0).unwrap();
        let client_id = row.get::<String, &str>("client_id").unwrap();
        let client_secret = row.get::<String, &str>("client_secret").unwrap();

        (client_id, client_secret)
    };

    let google_oauth_credentials = OAuthCredentials { client_id, client_secret, identifier: appdata::OAuthIdentifier::GOOGLE };
    let oauth_credentials_vec = vec![google_oauth_credentials];

    let mut tera = Tera::new("templates/**/*").expect("Unable to initialize Tera!");
    tera.autoescape_on(vec![]);

    let appdata = AppData { database, tera, oauth_credentials: oauth_credentials_vec };

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
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}