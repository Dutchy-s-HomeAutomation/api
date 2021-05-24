use std::fs;
use serde::{Deserialize, Serialize};
use crate::appdata::OAuthCredentials;
use mysql::{Row, Params, params};
use mysql::prelude::Queryable;
use rand::Rng;
use crate::database::Database;

#[derive(Deserialize, Serialize, Clone)]
pub struct ServicesConfig {
    pub name:           String,
    pub identifier:     crate::types::service::ServiceType,
    pub icon:           String,
    pub requires_login: bool,
    pub login_method:   crate::types::service::LoginMethod
}

pub fn read_services_conf() -> Vec<ServicesConfig> {
    let conf_content = fs::read_to_string("config/services.jsonc");
    if conf_content.is_err() {
        eprintln!("Unable to read services.jsonc: {:?}", conf_content.err());
        std::process::exit(1);
    }

    //Remove the comments
    let conf_str = conf_content.unwrap();
    let regex = regex::Regex::new(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*/+").unwrap();
    let conf_str_filtered = regex.replace_all(&conf_str, "").as_ref().to_string().clone();

    let config: Result<Vec<ServicesConfig>, serde_json::error::Error> = serde_json::from_str::<Vec<ServicesConfig>>(&conf_str_filtered);
    if config.is_err() {
        eprintln!("Invalid config 'services.jsonc': {:?}", config.err());
        std::process::exit(1);
    }

    config.unwrap()
}

pub fn get_oauth_credentials(database: &Database) -> Vec<OAuthCredentials> {

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

    let google_oauth_credentials = OAuthCredentials { client_id, client_secret, identifier: crate::appdata::OAuthIdentifier::GOOGLE };
    let oauth_credentials_vec = vec![google_oauth_credentials];

    oauth_credentials_vec
}