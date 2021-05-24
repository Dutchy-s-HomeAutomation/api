use crate::environment::Environment;
use crate::database::Database;
use crate::types::service::ServiceType;

use std::str::FromStr;
use mysql::{Error, Params, params, Row};
use mysql::prelude::Queryable;
use magic_crypt::MagicCryptTrait;
use magic_crypt::MagicCrypt256;

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String
}

/**
Set a username/password for a Service

## Parameters
    db: An instance of Database
    service_id: The ID of the Service to set the credentials for
    credentials: A Credentials object containing the username and password

## Returns
    Err: If an error occurred
    Ok: If everything went OK
*/
pub fn set_password_credentials(db: Database, service_id: String, credentials: Credentials) -> Result<(), Error>{
    let mut conn = db.pool.get_conn()?;

    //Get an instance of Environment and create an instance of MagicCrypt256 with our password pepper
    let env = Environment::new();
    let mc: MagicCrypt256 = new_magic_crypt!(env.password_pepper, 256);

    //Encrypt the username and password
    let username_encrypted_base64 = mc.encrypt_str_to_base64(credentials.username);
    let password_encrypted_base64 = mc.encrypt_str_to_base64(credentials.password);

    //Add a row to the service_password_credentials table
    let _ = conn.exec::<usize, &str, Params>("INSERT INTO services_password_credentials (service_id, username, password) VALUES (:service_id, :username, :password)", params! {
        "service_id" => service_id,
        "username" => username_encrypted_base64,
        "password" => password_encrypted_base64
    })?;

    Ok(())
}

/**
Get the Username/Password credentials for a Service

## Parameters
    db: An instance of Database
    service_Id: The ID of the Service to get the credentials for

## Returns
    Err: If an error occurred
    Ok:
        Some: An instance of Credentials containing the requested credentials
        None: The Service with the provided ID does not have username/Password credentials
*/
pub fn get_password_credentials(db: Database, service_id: String) -> Result<Option<Credentials>, Error> {
    let mut conn = db.pool.get_conn()?;

    //Get an instance of Environment and create a MagicCrypt256 instance of the password pepper
    let env = Environment::new();
    let mc: MagicCrypt256 = new_magic_crypt!(env.password_pepper, 256);

    //Fetch the username and password from the Database
    let fetch_result = conn.exec::<Row, &str, Params>("SELECT username, password FROM services_password_credentials WHERE service_id = :service_id", params! {
        "service_id" => service_id,
    })?;

    //If no results were returned, that must mean that the Service doesn't exist
    if fetch_result.len() == 0 {
        return Ok(None);
    }

    //Get the first row returned (it should only ever be 1 anyways)
    //and get the encrypted username and password
    let first_row = fetch_result.get(0).unwrap();
    let username_encrypted = first_row.get::<String, &str>("username").unwrap();
    let password_encrypted = first_row.get::<String, &str>("password").unwrap();

    //Decrypt the username and password
    let username_decrypted = mc.decrypt_base64_to_string(&username_encrypted).unwrap();
    let password_decrypted = mc.decrypt_base64_to_string(&password_encrypted).unwrap();

    Ok(Some(Credentials {
        username: username_decrypted,
        password: password_decrypted
    }))
}

/**
Create a Service

## Parameters
    db: An instance of Database
    user_id: The ID of the user who owns this service
    service_id: The ID of the Service to be created
    service_type: The type of Service being created

## Returns
    Err: If an error occurred
    Ok: If everything went OK
*/
pub fn create_service(db: Database, user_id: String, service_id: String, service_type: ServiceType) -> Result<(), Error> {
    let mut conn = db.pool.get_conn()?;
    let _ = conn.exec::<usize, &str, Params>("INSERT INTO services (user_id, service_id, identifier) VALUES (:user_id, :service_id, :identifier)", params! {
        "user_id" => user_id,
        "service_id" => service_id,
        "identifier" => service_type.to_string()
    })?;

    Ok(())
}

/**
Get all Services owned by a specific User

## Parameters
    db: An instance of Database
    suer_id: The ID of the User to fetch all Services for

## Returns
    Err: If an error occurred
    Ok: Returns a Vector containing all service_id's with their respective ServiceType found for the requested user
*/
pub fn get_services(db: Database, user_id: String) -> Result<Vec<(String, ServiceType)>, Error> {

    let mut conn = db.pool.get_conn()?;
    let get_services_query = conn.exec::<Row, &str, Params>("SELECT service_id, identifier FROM services WHERE user_id = :user_id", params! {
        "user_id" => user_id
    })?;

    let mut result: Vec<(String, ServiceType)> = vec![];
    for row in get_services_query {
        let service_id = row.get::<String, &str>("service_id").unwrap();
        let identifier = row.get::<String, &str>("identifier").unwrap();

        let service_type = ServiceType::from_str(&identifier).unwrap();

        result.push((service_id, service_type));
    }

    Ok(result)
}