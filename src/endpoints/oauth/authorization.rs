use actix_web::{get, HttpResponse, HttpRequest, web, HttpMessage};
use crate::appdata::AppData;
use qstring::QString;
use crate::environment::Environment;
use rand::Rng;
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};

pub async fn get_authorization(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = QString::from(req.query_string());

    let client_id_param = qstring.get("client_id");
    if client_id_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'client_id'");
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

    let response_type_param = qstring.get("response_type");
    if response_type_param.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'response_type'");
    }

    if !response_type_param.unwrap().eq("code") {
        return HttpResponse::BadRequest().body("Value of parameter 'response_type' is invalid");
    }

    let user_locale_param = qstring.get("locale");
    if user_locale.is_none() {
        return HttpResponse::BadRequest().body("Missing parameter 'locale'");
    }

    let env = Environment::new();
    if !client_id_param.unwrap().eq(env.google_client_id.as_str()) {
        return HttpResponse::BadRequest().body("Value of parameter 'client_id' is invalid.");
    }

    let authorization_code: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

    if !(redirect_uri_param.unwrap().contains(" https://oauth-redirect.googleusercontent.com/r/") || redirect_uri_param.contains("https://oauth-redirect-sandbox.googleusercontent.com/r/")) {
        return HttpResponse::BadRequest().body("Value of parameter 'redirect_uri' is invalid.");
    }

    let mut is_signed_in = false;
    let mut user_id: Option<String>;

    //Get the session_id cookie, if it is set
    let session_cookie: Option<String> = req.cookie("session_id");
    if session_cookie.is_some() {
        let conn = data.database.pool.get_conn();
        if conn.is_err() {
            return HttpResponse::InternalServerError().body("Something went wrong. Please try again later!");
        }


        let mut conn_unwrapped = conn.unwrap();
        let query_result = conn_unwrapped.exec::<Row, &str, Params>("SELECT session_id,user_id FROM sessions WHERE session_id = :session_id", params! {
            "session_id" => session_cookie.unwrap()
        });

        if query_result.is_err() {
            return HttpResponse::InternalServerError().body("Something went wrong. Please try again later!");
        }

        let query_unwrapped = query_result.unwrap();
        for row in query_unwrapped {
            is_signed_in = true;

            let user_id_from_db = row.get::<String, &str>("user_id");
            if user_id_from_db.is_none() {
                continue;
            }

            user_id = Some(user_id_from_db.unwrap());
        }
    }

    if !is_signed_in {

    }

    //Insert authorization code into the database


    return HttpResponse::Ok().finish();


}