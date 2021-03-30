use crate::environment::Environment;

use mysql::{Error, Params, params, Row};
use mysql::prelude::Queryable;
use magic_crypt::MagicCryptTrait;
use magic_crypt::MagicCrypt256;
use crate::database::Database;

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String
}

pub fn set_password_credentials(db: Database, user_id: String, credentials: Credentials, service_type: crate::types::service::ServiceType) -> Result<(), Error>{
    let mut conn = db.pool.get_conn()?;

    let env = Environment::new();
    let mc: MagicCrypt256 = new_magic_crypt!(env.password_pepper, 256);

    let password_encrypted_base64 = mc.encrypt_str_to_base64(credentials.password);
    let username_encrypted_base64 = mc.encrypt_str_to_base64(credentials.username);

    let _ = conn.exec::<usize, &str, Params>("INSERT INTO api_passwords (user_id, username, password, service) VALUES (:user_id, :username, :password, :service)", params! {
        "user_id" => user_id,
        "username" => username_encrypted_base64,
        "password" => password_encrypted_base64,
        "service" => service_type.to_string()
    })?;

    Ok(())
}

pub fn get_password_credentials(db: Database, user_id: String, service_type: crate::types::service::ServiceType) -> Result<Option<Credentials>, Error> {
    let mut conn = db.pool.get_conn()?;

    let env = Environment::new();
    let mc: MagicCrypt256 = new_magic_crypt!(env.password_pepper, 256);

    let fetch_result = conn.exec::<Row, &str, Params>("SELECT username, password FROM api_passwords WHERE user_id = :user_id AND service = :service_type", params! {
        "user_id" => user_id,
        "service_type" => service_type.to_string()
    })?;

    if fetch_result.len() == 0 {
        return Ok(None);
    }

    let first_row = fetch_result.get(0).unwrap();

    let password_encrypted = first_row.get::<String, &str>("password").unwrap();
    let username_encrypted = first_row.get::<String, &str>("username").unwrap();

    let password_decrypted = mc.decrypt_base64_to_string(&password_encrypted).unwrap();
    let username_decrypted = mc.decrypt_base64_to_string(&username_encrypted).unwrap();

    let credentials = Credentials {
        username: username_decrypted,
        password: password_decrypted
    };

    Ok(Some(credentials))
}