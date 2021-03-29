use actix_web::{get, HttpResponse, HttpRequest, web};
use crate::appdata::AppData;
use qstring::QString;
use crate::environment::Environment;
use rand::Rng;
use mysql::prelude::Queryable;
use mysql::{Params, params};
use tera::Context;

#[get("/oauth/login")]
pub async fn get_login(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = QString::from(req.query_string());

    let client_id_param = qstring.get("client_id");
    if client_id_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'client_id'");
    }

    if !client_id_param.unwrap().eq(&data.oauth_credentials.get(0).unwrap().client_id) {
        return HttpResponse::BadRequest().body("Value of parameter 'client_id' is invalid.");
    }

    let redirect_uri_param = qstring.get("redirect_uri");
    if redirect_uri_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'redirect_uri'");
    }

    let state_param = qstring.get("state");
    if state_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'state'");
    }

    let scope_param = qstring.get("scope");
    if scope_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'scope'");
    }

    let response_type_param = qstring.get("response_type");
    if response_type_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'response_type'");
    }

    if !response_type_param.unwrap().eq("code") {
        return HttpResponse::BadRequest().body("Value of parameter 'response_type' is invalid. Only the value of 'code' is supported.");
    }

    let user_locale_param = qstring.get("locale");
    if user_locale_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'locale'");
    }

    if !(redirect_uri_param.unwrap().contains(" https://oauth-redirect.googleusercontent.com/r/") || redirect_uri_param.unwrap().contains("https://oauth-redirect-sandbox.googleusercontent.com/r/")) {
        return HttpResponse::BadRequest().body("Value of parameter 'redirect_uri' is invalid.");
    }

    //Create an internal state string
    let internal_state: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    //Insert the internal_state and Oauth state into the database
    let conn = data.database.pool.get_conn();
    if conn.is_err() {
        eprintln!("Unable to open a connection to the Database: {:?}", conn.err());
        return HttpResponse::InternalServerError().finish();
    }

    let mut conn_unwrapped = conn.unwrap();

    let db_insert_response = conn_unwrapped.exec::<usize, &str, Params>("INSERT INTO oauth_states (internal_state, oauth_state, oauth_redirect_uri) VALUES (:internal_state, :oauth_state, :oauth_redirect_uri)", params! {
        "internal_state" => internal_state.clone(),
        "oauth_state" => state_param.unwrap().to_string(),
        "redirect_uri_param" => redirect_uri_param.unwrap()
    });

    if db_insert_response.is_err() {
        eprintln!("An error occurred while inserting data into the Database: {:?}", db_insert_response.err());
        return HttpResponse::InternalServerError().finish();
    }

    let env = Environment::new();
    
    //Format the redirect_uri
    let redirect_uri = format!("{host}/static/login/login.html?is_oauth=true&state={internal_state}",
        host            = env.host,
        internal_state  = internal_state.clone()
    );

    let mut ctx = Context::new();
    ctx.insert("redirect_uri", &redirect_uri);

    let rendered = data.tera.render("redirect.html", &ctx).unwrap();
    return HttpResponse::Ok().body(rendered);
}