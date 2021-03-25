use crate::appdata::AppData;
use crate::environment::Environment;
use crate::logic::user::User;

use actix_web::{web, post, HttpResponse, HttpRequest};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use serde::{Serialize, Deserialize};
use mysql::consts::SessionStateType::SESSION_TRACK_GTIDS;

#[derive(Deserialize)]
struct SessionForm {
    session_id: String
}

#[derive(Serialize)]
struct Session {
    status:     i16,
    user:       Option<User>
}

#[post("/auth/session")]
pub async fn post_session(data: web::Data<AppData>, req: HttpRequest, form: web::Form<SessionForm>) -> HttpResponse {
    let user = crate::logic::user::get_user(form.session_id.clone().as_str(), data.get_ref());
    if user.is_err() {
        eprintln!("Unable to verify session_id: {:?}", user.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user_unwrapped = user.unwrap();
    if user_unwrapped.is_none() {
        let session_response = Session { status: 401, user: None };
        return HttpResponse::Ok().json(session_response);
    }

    let session_response = Session { status: 200, user: Some(user_unwrapped.unwrap()) };
    return HttpResponse::Ok().json(session_response);
}