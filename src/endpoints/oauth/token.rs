use actix_web::{post, web, HttpResponse, HttpRequest};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use rand::Rng;
use serde::Serialize;
use crate::appdata::AppData;
use crate::environment::Environment;

#[derive(Serialize)]
struct TokenResponse {
    token_type:     String,
    access_token:   String,
    refresh_token:  String,
    expires_in:     i16
}

#[derive(Serialize)]
struct RefreshTokenResponse {
    token_type:     String,
    access_token:   String,
    expires_in:     i16
}

#[post("/oauth/token")]
pub async fn post_token(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

    let client_id_param = qstring.get("client_id");
    if client_id_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'client_id'");
    }

    let client_id_unwrapped = client_id_param.unwrap();
    if !client_id_unwrapped.eq(&data.oauth_credentials.get(0).unwrap().client_id) {
        return HttpResponse::BadRequest().json("{\"error\": \"invalid_grant\"}");
    }

    let client_secret_param = qstring.get("client_secret");
    if client_secret_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'client_secret'");
    }

    let client_secret_unwrapped = client_secret_param.unwrap();
    if !client_secret_unwrapped.eq(&data.oauth_credentials.get(0).unwrap().client_secret) {
        return HttpResponse::BadRequest().json("{\"error\": \"invalid_grant\"}");
    }

    let grant_type_param = qstring.get("grant_type");
    if grant_type_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'grant_type'");
    }

    //Create a database connection
    let conn = data.database.pool.get_conn();
    if conn.is_err() {
        eprintln!("Unable to open a connection to the Database: {:?}", conn.err());
        return HttpResponse::InternalServerError().finish();
    }

    let mut conn_unwrapped = conn.unwrap();

    let grant_type_unwrapped = grant_type_param.unwrap();
    if grant_type_unwrapped == "authorization_code" {
        let code_param = qstring.get("code");
        if code_param.is_none() {
            return HttpResponse::BadRequest().body("Missing parameter 'code'");
        }

        let redirect_uri_param = qstring.get("redirect_uri");
        if redirect_uri_param.is_none() {
            return HttpResponse::BadRequest().body("Missing parameter 'redirect_uri'");
        }

        let env = Environment::new();
        if !redirect_uri_param.unwrap().eq(&format!("{}/oauth/login", env.host)) {
            return HttpResponse::BadRequest().json("{\"error\": \"invalid_grant\"}");
        }

        //Verify the code param
        let sql_code_query = conn_unwrapped.exec::<Row, &str, Params>("SELECT user_id FROM oauth_authorization_codes WHERE authorization_code = :authorization_code", params! {
            "authorization-code" => code_param.unwrap()
        });

        if sql_code_query.is_err() {
            eprintln!("An error occurred while fetching data from the Database: {:?}", sql_code_query.err());
            return HttpResponse::InternalServerError().finish();
        }

        let fetch_result = sql_code_query.unwrap();
        if fetch_result.len() == 0 {
            return HttpResponse::BadRequest().json("{\"error\": \"invalid_grant\"}");
        }

        let row = fetch_result.get(0).unwrap();
        let user_id = row.get::<String, &str>("user_id").unwrap();

        //Generate an access and refresh token
        let access_token: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect();
        let refresh_token: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

        //Calculate the new expiry time
        let datetime = chrono::Utc::now() + chrono::Duration::seconds(3600);
        let expiry = datetime.timestamp();

        //Insert the two tokens into the database
        let sql_insert_query = conn_unwrapped.exec::<usize, &str, Params>("INSERT INTO oauth_grants (user_id, access_token, refresh_token, expiry) VALUES (:user_id, :access_token, :refresh_token, :expiry)", params! {
            "user_id" => user_id.clone(),
            "access_token" => access_token.clone(),
            "refresh_token" => refresh_token.clone(),
            "expiry" => expiry
        });

        if sql_insert_query.is_err() {
            eprintln!("An error occurred while inserting data into the Database: {:?}", sql_insert_query.err());
            return HttpResponse::InternalServerError().finish();
        }

        //Remove the authorization code from the Database
        let remove_sql_query = conn_unwrapped.exec::<usize, &str, Params>("DELETE FROM oauth_authorization_codes WHERE user_id = :user_id", params! {
            "user_id" => user_id.clone()
        });

        if remove_sql_query.is_err() {
            eprintln!("An error occurred while inserting data into the Database: {:?}", remove_sql_query.err());
            return HttpResponse::InternalServerError().finish();
        }

        let token_response = TokenResponse {
            token_type: "Bearer".to_string(),
            access_token,
            refresh_token,
            expires_in: 3600
        };

        return HttpResponse::Ok().json(token_response);
    } else if grant_type_unwrapped == "refresh_token" {
        let refresh_token_param = qstring.get("refresh_token");
        if refresh_token_param.is_none() {
            return HttpResponse::BadRequest().body("Missing parameter 'refresh_token'");
        }

        //Verify the refresh_token
        let sql_fetch_result = conn_unwrapped.exec::<Row, &str, Params>("SELECT user_id FROM oauth_grants WHERE refresh_token = :refresh_token", params! {
            "refresh_token" => refresh_token_param.unwrap()
        });

        if sql_fetch_result.is_err() {
            eprintln!("An error occurred while fetching data from the Database: {:?}", sql_fetch_result.err());
            return HttpResponse::InternalServerError().finish();
        }

        let fetch_result_unwrapped = sql_fetch_result.unwrap();
        if fetch_result_unwrapped.len() == 0 {
            return HttpResponse::BadRequest().json("{\"error\": \"invalid_grant\"}");
        }

        let row = fetch_result_unwrapped.get(0).unwrap();
        let user_id = row.get::<String, &str>("user_id").unwrap();

        //Generate a new access_token
        let access_token: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect();

        //Calculate the new expiry time
        let datetime = chrono::Utc::now() + chrono::Duration::seconds(3600);
        let expiry = datetime.timestamp();

        //Update the database with the new token
        let sql_insert_result = conn_unwrapped.exec::<usize, &str, Params>("UPDATE oauth_grants SET access_token = :access_token expiry = :expiry WHERE user_id = :user_id", params! {
            "access_token" => access_token.clone(),
            "user_id" => user_id,
            "expiry" => expiry
        });

        if sql_insert_result.is_err() {
            eprintln!("An error occurred while inserting data into the Database: {:?}", sql_insert_result.err());
            return HttpResponse::InternalServerError().finish();
        }

        let refresh_token_response = RefreshTokenResponse {
            token_type: "Bearer".to_string(),
            access_token,
            expires_in: 3600
        };

        return HttpResponse::Ok().json(refresh_token_response);
    } else {
        HttpResponse::BadRequest().body("Invalid value for parameter 'grant_type''");
    }

    HttpResponse::Ok().finish()
}