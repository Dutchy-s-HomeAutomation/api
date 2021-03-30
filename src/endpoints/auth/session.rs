use crate::appdata::AppData;
use crate::common::user::User;

use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct SessionForm {
    session_id: String
}

#[derive(Serialize)]
struct Session {
    status:     i16,
    user:       Option<User>
}

#[post("/auth/session")]
pub async fn post_session(data: web::Data<AppData>, form: web::Form<SessionForm>) -> HttpResponse {
    let user = crate::common::user::get_user(form.session_id.clone().as_str(), data.get_ref());
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