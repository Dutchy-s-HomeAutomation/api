use crate::appdata::AppData;
use crate::environment::Environment;
use crate::endpoints::auth::{LoginResponse, LoginForm};

use actix_web::{web, post, HttpResponse};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use sha2::{Sha512Trunc256, Digest};
use rand::Rng;
use bcrypt::Version;

/**
Endpoint allowing a user to log in with their Email and Password

## Endpoint
Path:   /auth/login
Method: Post

## Parameters
| Name     | Type                        | Description                       |
|----------|-----------------------------|-----------------------------------|
| email    | Base64 encoded UTF-8 String | The email provided by the user    |
| password | Base64 encoded UTF-8 String | The password provided by the user |

## Body
No body should be provided

## Returns
| Name           | Type            | Description                                                                 |
|----------------|-----------------|-----------------------------------------------------------------------------|
| status         | i16             | Refer to the status code documentation                                      |
| status_message | Optional String | If the status has a message, this will hold a message describing the status |
| session_id     | Optional String | If the login succeeded, this will hold the session_id                       |
*/
#[post("/auth/login")]
pub async fn post_login(data: web::Data<AppData>, form: web::Form<LoginForm>) -> HttpResponse {
    //Decode the Email address from Base64 to a vector of bytes and check if it succeeded
    //This could fail due to the client providing an invalid Base64 String
    let email_decoded_result = base64::decode(form.email.clone().as_bytes());
    if email_decoded_result.is_err() {
        return HttpResponse::BadRequest().body(email_decoded_result.err().unwrap().to_string());
    }

    //Decode the password from Base64 to a vector of bytes and check if it succeeded
    //This could fail due to the client providing an invalid Base64 String
    let password_decoded_result = base64::decode(form.password.clone().as_bytes());
    if password_decoded_result.is_err() {
        return HttpResponse::BadRequest().body(password_decoded_result.err().unwrap().to_string());
    }

    //Convert both the vectors into a UTF-8 String
    let email = String::from_utf8(email_decoded_result.unwrap()).unwrap();
    let password = String::from_utf8(password_decoded_result.unwrap()).unwrap();

    let env = Environment::new();

    //Create a connection to the database
    let conn_wrapped = data.database.pool.get_conn();
    if conn_wrapped.is_err() {
        eprintln!("Unable to unwrap Database connection: {:?}", conn_wrapped.err());
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn_wrapped.unwrap();

    //Fetch the password and salt from the database
    let sql_fetch_result = conn.exec::<Row, &str, Params>("SELECT password, salt FROM users WHERE email = :email", params! {
        "email" => email.clone()
    });

    //Check if the fetch succeeded
    if sql_fetch_result.is_err() {
        eprintln!("Unable to execute a SQL Fetch query: {:?}", sql_fetch_result.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Check if we got any results at all
    let sql_fetch_result_unwrapped = sql_fetch_result.unwrap();
    let row_count = sql_fetch_result_unwrapped.len();
    if row_count == 0 {
        //We got no results, this means the user account does not exist. We return a 401 status for security reasons.
        //If we'd return a 404 an attacker could figure out what emails are valid
        let login_response = LoginResponse::new(401, Some("E-mail address and password combination is invalid, or the account does not exist.".to_string()), None);
        return HttpResponse::Ok().json(login_response);
    }

    //Check if we got more than 1 result, this should never happen!
    if row_count > 1 {
        eprintln!("Database returned more than 1 row from the database. This should never happen! (login.rs)");
        return HttpResponse::InternalServerError().finish();
    }

    //Get the password and salt columns from the first row
    let row = sql_fetch_result_unwrapped.get(0).unwrap();
    let password_from_db = row.get::<String, &str>("password");
    let salt_from_db = row.get::<String, &str>("salt");

    //Check if the password or the salt from the database are None
    //If this is the case, something went horribly wrong
    if password_from_db.is_none() || salt_from_db.is_none() {
        //We got a result, but the password or salt are empty, This is an error.
        eprintln!("Received a result from the Database, but the password or the salt are empty (login.rs)");
        return HttpResponse::InternalServerError().finish();
    }

    //Create a Sha512 hasher instance
    let mut hasher = Sha512Trunc256::new();

    //Insert the password, salt fetched from the database, and application pepper into the Hasher
    hasher.update(&password);
    hasher.update(&salt_from_db.clone().unwrap());
    hasher.update(&env.password_pepper);

    //Encode the hashed password as Base64
    let password_hashed = base64::encode(hasher.finalize());

    //Run the Bcrypt algorithm on the hashed password
    //Cost of 10 is a nice balance between strength and computation time
    let password_bcrypt = bcrypt::hash_with_salt(&password_hashed, 10, salt_from_db.clone().unwrap().as_bytes());
    if password_bcrypt.is_err() {
        eprintln!("An error occurred while bcrypt-ing the password hash: {:?}", password_bcrypt.err());
        return HttpResponse::InternalServerError().finish();
    }

    let hash_parts = password_bcrypt.unwrap();
    let password_final = hash_parts.format_for_version(Version::TwoY);

    //Check if the password provided by client and the password in the database match
    if password_final != password_from_db.unwrap() {
        //Passwords do not match.
        let login_response = LoginResponse::new(401, Some("E-mail address and password combination is invalid, or the account does not exist.".to_string()), None);
        return HttpResponse::Ok().json(login_response);
    }

    //Generate a session ID for the user
    let session_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    //Write the session ID to the database
    let session_id_write_response = conn.exec::<usize, &str, Params>("UPDATE users SET session_id = :session_id", params! {
        "session_id" => session_id.clone()
    });

    //Check if this operation succeeded.
    //This is important, otherwise the user will end up in
    //an infinite login loop
    if session_id_write_response.is_err() {
        eprintln!("An error occurred while writing the session_id to the Database: {:?}", session_id_write_response.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Create a response indicating that the user has logged in, and return it that to the user
    let login_response = LoginResponse::new(200, None, Some(session_id));
    return HttpResponse::Ok().json(login_response);
}