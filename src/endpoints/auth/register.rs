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
Endpoint allowing a user to register with their Email and Password

## Endpoint
Path:   /auth/register
Method: POST

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
#[post("/auth/register")]
pub async fn post_register(data: web::Data<AppData>, form: web::Form<LoginForm>) -> HttpResponse {

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
        eprintln!("An error occurred while creating a connection to the Database: {:?}", conn_wrapped.err());
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn_wrapped.unwrap();

    //Check if the user is already registered with
    let sql_fetch_result = conn.exec::<Row, &str, Params>("SELECT 1 FROM users WHERE email = :email", params! {
        "email" => email.clone()
    });

    if sql_fetch_result.is_err() {
        eprintln!("An error occurred while fetching rows from the Database: {:?}", sql_fetch_result.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Get the row count
    //If it's > 0, an account with that E-mail address already exists
    let row_count = sql_fetch_result.unwrap().len();
    if row_count > 0 {
        let login_response = LoginResponse::new(409, Some("Account already exists".to_string()), None);
        return HttpResponse::Ok().json(login_response);
    }

    //Generate a 64 character long salt
    let salt: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(16).map(char::from).collect();

    //Create a Sha512 hasher instance
    let mut hasher = Sha512Trunc256::new();

    //Hash the password with sha512 truncated to 256 bits
    //Include the provided password, the generated salt, and the application's pepper
    hasher.update(&password);
    hasher.update(&salt.clone());
    hasher.update(&env.password_pepper);

    //Encode the hashed password as Base64
    let password_hashed = base64::encode(hasher.finalize());

    //Run the Bcrypt algorithm on the hashed password
    //Cost of 10 is a nice balance between strength and computation time
    let password_bcrypt = bcrypt::hash_with_salt(&password_hashed, 10, salt.clone().as_bytes());
    if password_bcrypt.is_err() {
        eprintln!("An error occurred while bcrypt-ing the hashed password: {:?}", password_bcrypt.err());
        return HttpResponse::InternalServerError().finish();
    }

    let hash_parts = password_bcrypt.unwrap();
    let password_final = hash_parts.format_for_version(Version::TwoY);

    //Generate a 64 character long session ID and user ID
    let session_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();
    let user_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    //Write the user id, email, hashed and bcrypt-ed password, salt and session id to the database
    let sql_write_response = conn.exec::<usize, &str, Params>("INSERT INTO users (user_id, email, password, salt, session_id) VALUES (:user_id, :email, :password, :salt, :session_id)", params! {
        "user_id" => user_id,
        "email" => email,
        "password" => password_final,
        "salt" => salt.clone(),
        "session_id" => session_id.clone()
    });

    //Check if the insert operation succeeded
    if sql_write_response.is_err() {
        eprintln!("An error occurred while inserting a new account into the Database: {:?}", sql_write_response.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Create a response indicating everything went OK
    //The user is now registered
    let login_response = LoginResponse::new(200, None, Some(session_id));
    return HttpResponse::Ok().json(login_response);
}