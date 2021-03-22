use crate::appdata::AppData;
use crate::environment::Environment;
use crate::endpoints::auth::LoginResponse;

use actix_web::{web, post, HttpResponse, HttpRequest};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use sha2::{Sha512Trunc256, Digest};
use rand::Rng;

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
pub async fn post_login(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

    //Get and validate that the 'email' query parameter is given
    let email_param = qstring.get("email");
    if email_param.is_none() {
        return HttpResponse::BadRequest().body("Missing required parameter 'email'");
    }

    //Get and validate that the 'password' query parameter is given
    let password_param = qstring.get("password");
    if password_param.is_none() {
        return HttpResponse::BadRequest().body("Missing required parameter 'password'");
    }

    //Decode the Email address from Base64 to a vector of bytes and check if it succeeded
    //This could fail due to the client providing an invalid Base64 String
    let email_decoded_result = base64::decode(email_param.unwrap().as_bytes());
    if email_decoded_result.is_err() {
        return HttpResponse::BadRequest().body(email_decoded_result.err().unwrap().to_string());
    }

    //Decode the password from Base64 to a vector of bytes and check if it succeeded
    //This could fail due to the client providing an invalid Base64 String
    let password_decoded_result = base64::decode(password_param.unwrap());
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
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn_wrapped.unwrap();

    //Fetch the password and salt from the database
    let sql_fetch_result = conn.exec::<Row, &str, Params>("SELECT password, salt FROM users WHERE email = :email", params! {
        "email" => email.clone()
    });

    //Check if the fetch succeeded
    if sql_fetch_result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    //Iterate over the returned rows
    //Take the first row and get the password and salt
    let mut password_from_db: Option<String> = None;
    let mut salt_from_db: Option<String> = None;
    for row in sql_fetch_result.unwrap() {
        //Get the password, and check that it is not None
        let password = row.get::<String, &str>("password");
        if password.is_none() {
            return HttpResponse::InternalServerError().finish();
        }

        //Get the salt, and check that it is not None
        let salt = row.get::<String, &str>("salt");
        if salt.is_none() {
            return HttpResponse::InternalServerError().finish();
        }

        password_from_db = Some(password.unwrap());
        salt_from_db = Some(salt.unwrap());

        //Only doing 1 iteration
        break;
    }

    //Check if the password or the salt from the database are None
    //If this is the case the user does not have an account
    if password_from_db.is_none() || salt_from_db.is_none() {
        let login_response = LoginResponse::new(404, Some("No user exists with that E-mail address".to_string()), None);
        return HttpResponse::Ok().json(login_response);
    }

    //Create a Sha512 hasher instance
    let mut hasher = Sha512Trunc256::new();

    //Insert the password, salt fetched from the database, and application pepper into the Hasher
    hasher.update(&password);
    hasher.update(&salt_from_db.unwrap());
    hasher.update(&env.password_pepper);

    //Encode the hashed password as Base64
    let password_hashed = base64::encode(hasher.finalize());

    //Run the Bcrypt algorithm on the hashed password
    //Cost of 10 is a nice balance between strength and computation time
    let password_bcrypt = bcrypt::hash(&password_hashed, 10);
    if password_bcrypt.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    //Check if the password provided by client and the password in the database match
    if password_bcrypt.unwrap() != password_from_db.unwrap() {
        //Passwords do not match.
        let login_response = LoginResponse::new(401, Some("E-mail address and password combination is invalid".to_string()), None);
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
        return HttpResponse::InternalServerError().finish();
    }

    //Create a response indicating that the user has logged in, and return it that to the user
    let login_response = LoginResponse::new(200, None, Some(session_id));
    return HttpResponse::Ok().json(login_response);
}