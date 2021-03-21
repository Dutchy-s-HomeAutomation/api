use std::env;

pub struct Environment {
    pub mysql_host:                 String,
    pub mysql_database:             String,
    pub mysql_username:             String,
    pub mysql_password:             String,

    pub host:                       String,

    pub honeywell_consumer_key:     String,
    pub honeywell_consumer_secret:  String,

    pub google_client_id:           String,
}

impl Environment {
    pub fn new() -> Environment {
        let mysql_host = env::var("MYSQL_HOST");
        if mysql_host.is_err() {
            environmental_variable_not_set("MYSQL_HOST");
        }

        let mysql_database = env::var("MYSQL_DATABASE");
        if mysql_database.is_err() {
            environmental_variable_not_set("MYSQL_DATABASE");
        }

        let mysql_username = env::var("MYSQL_USERNAME");
        if mysql_username.is_err() {
            environmental_variable_not_set("MYSQL_USERNAME");
        }

        let mysql_password = env::var("MYSQL_PASSWORD");
        if mysql_password.is_err() {
            environmental_variable_not_set("MYSQL_PASSWORD");
        }

        let host = env::var("HOST");
        if host.is_err() {
            environmental_variable_not_set("HOST");
        }

        let honeywell_consumer_key = env::var("HONEYWELL_CONSUMER_KEY");
        if honeywell_consumer_key.is_err() {
            environmental_variable_not_set("HONEYWELL_CONSUMER_KEY");
        }

        let honeywell_consumer_secret = env::var("HONEYWELL_CONSUMER_SECRET");
        if honeywell_consumer_secret.is_err() {
            environmental_variable_not_set("HONEYWELL_CONSUMER_SECRET");
        }

        let google_client_id = env::var("GOOGLE_CLIENT_ID");
        if google_client_id {
            environmental_variable_not_set("GOOGLE_CLIENT_ID");
        }

        Environment {
            mysql_host:                 mysql_host.unwrap(),
            mysql_database:             mysql_database.unwrap(),
            mysql_username:             mysql_username.unwrap(),
            mysql_password:             mysql_password.unwrap(),

            host:                       host.unwrap(),

            honeywell_consumer_key:     honeywell_consumer_key.unwrap(),
            honeywell_consumer_secret:  honeywell_consumer_secret.unwrap(),

            google_client_id:           google_client_id.unwrap()
        }
    }
}

fn environmental_variable_not_set(variable_name: &str) {
    eprintln!("Environmental variable {} is not set. Exiting!", variable_name);
    std::process::exit(1);
}
