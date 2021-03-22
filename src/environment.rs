use std::env;

/**
Object with the Environmental variables required by SmartHome
*/
pub struct Environment {
    /// The password pepper to be used when hashing passwords
    pub password_pepper:            String,

    /// The FQDN or IP of the MySQL database server
    pub mysql_host:                 String,
    /// The MySQL database to use
    pub mysql_database:             String,
    /// The username to use when authenticating with the MySQL server
    pub mysql_username:             String,
    /// the password to use when authenticating with the MySQL server
    pub mysql_password:             String,

    /// The host of this server. E.g 'https://api.thedutchmc.nl'
    /// This should NOT end with a trailing slash
    pub host:                       String,

    /// Consumer key provided by Honeywell
    pub honeywell_consumer_key:     String,

    /// Consumer secret provided by Honeywell
    pub honeywell_consumer_secret:  String,

    /// Google Client ID provided by Google
    pub google_client_id:           String,
}

impl Environment {

    /**
    Verify that all required environmental variables are set and create and return an Environment object
    */
    pub fn new() -> Environment {
        let password_pepper = env::var("PASSWORD_PEPPER");
        if password_pepper.is_err() {
            environmental_variable_not_set("PASSWORD_PEPPER");
        }

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
        if google_client_id.is_err() {
            environmental_variable_not_set("GOOGLE_CLIENT_ID");
        }

        Environment {
            password_pepper:            password_pepper.unwrap(),
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
