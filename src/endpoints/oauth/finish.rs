use actix_web::{web, post, HttpResponse};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use crate::appdata::AppData;
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Deserialize)]
pub struct FinishFlowForm {
    session_id:     String,

    #[serde(rename(deserialize = "state"))]
    internal_state: String
}

#[derive(Serialize)]
struct FinishResponse {
    status:         i16,
    redirect_uri:   Option<String>
}

#[post("/oauth/finish")]
pub async fn post_finish(data: web::Data<AppData>, form: web::Form<FinishFlowForm>) -> HttpResponse {
    //Open a connection to the database
    let conn = data.database.pool.get_conn();
    if conn.is_err() {
        eprintln!("Unable to open a connection to the Database: {:?}", conn.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Query the database for the provided internal_state
    let mut conn_unwrapped = conn.unwrap();
    let oauth_states_query_response = conn_unwrapped.exec::<Row, &str, Params>("SELECT oauth_state, oauth_redirect_uri FROM oauth_states WHERE internal_state = :internal_state", params! {
        "internal_state" => form.internal_state.clone()
    });

    if oauth_states_query_response.is_err() {
        eprintln!("An error occurred while fetching oauth_state, oauth_redirect_uri from the Database: {:?}", oauth_states_query_response.err());
        return HttpResponse::InternalServerError().finish();
    }

    let oauth_state_query_response_unwrapped = oauth_states_query_response.unwrap();
    if oauth_state_query_response_unwrapped.len() == 0 {
        let finish_response = FinishResponse { status: 404, redirect_uri: None };
        return HttpResponse::Ok().json(finish_response);
    }

    let first_row = oauth_state_query_response_unwrapped.get(0).unwrap();

    let oauth_state = first_row.get::<String, &str>("oauth_state").unwrap();
    let oauth_redirect_uri = first_row.get::<String, &str>("oauth_redirect_uri").unwrap();

    //Get the user_id from the Database
    let user_id_query_response = conn_unwrapped.exec::<Row, &str, Params>("SELECT user_id FROM users WHERE session_id = :session_id", params! {
        "session_id" => form.session_id.clone()
    });

    if user_id_query_response.is_err() {
        eprintln!("An error occurred while fetching user_id from the Database: {:?}", user_id_query_response.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user_id_query_response_unwrapped = user_id_query_response.unwrap();
    if user_id_query_response_unwrapped.len() == 0 {
        let finish_response = FinishResponse { status: 401, redirect_uri: None };
        return HttpResponse::Ok().json(finish_response);
    }

    let first_row = user_id_query_response_unwrapped.get(0).unwrap();
    let user_id = first_row.get::<String, &str>("user_id").unwrap();

    //Generate an authorization state
    let authorization_code: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    //Insert the authorization code into the Database
    let authorization_put_response = conn_unwrapped.exec::<usize, &str, Params>("INSERT INTO oauth_authorization_codes (user_id, authorization_code) VALUES (:user_id, :authorization_code)", params! {
        "user_id" => user_id,
        "authorization_id" => authorization_code.clone()
    });

    if authorization_put_response.is_err() {
        eprintln!("An error occurred while isnerting the authorization code into the database");
        return HttpResponse::InternalServerError().finish();
    }

    //We can now remove the internal state from the Database
    let _ = conn_unwrapped.exec::<usize, &str, Params>("DELETE FROM oauth_states WHERE internal_state = :internal_state", params! {
        "internal_state" => form.internal_state.clone()
    });

    let redirect_uri = format!("{oauth_redirect_uri}?code={authorization_code}&state={oauth_state}",
        oauth_redirect_uri = oauth_redirect_uri,
        authorization_code = authorization_code,
        oauth_state        = oauth_state
    );

    let finish_response = FinishResponse { status: 200, redirect_uri: Some(redirect_uri)};
    return HttpResponse::Ok().json(finish_response);
}