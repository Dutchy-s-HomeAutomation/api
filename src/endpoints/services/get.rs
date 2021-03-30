use actix_web::{get, web, HttpResponse};
use crate::appdata::AppData;
use actix_web::dev::ServiceRequest;

pub struct GetServicesResponse {
    status:         i16,
    services:       Option<Vec<crate::config::ServicesConfig>>
}

pub struct GetServicesRequest {
    session_id:     String,
    only_owned:     bool
}

#[get("/services/get")]
pub async fn get_get(data: web::Data<AppData>, bytes: web::Bytes) -> HttpResponse {
    // payload is a stream of Bytes objects
    let body = String::from_utf8(bytes.to_vec());
    let body_unwrapped = body.unwrap();

    let request: Result<GetServicesRequest, serde_json::error::Error> = serde_json::from_str::<GetServicesRequest>(&body_unwrapped);
    if request.is_err() {
        return HttpResponse::BadRequest().finish();
    }

    let request_unwrapped = request.unwrap();

    let user_result = crate::common::user::get_user(&request_unwrapped.session_id, &data);
    if user_result.is_err() {
        eprintln!("An error occurred: {:?}", user_result.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user_optn = user_result.unwrap();
    if user_optn.is_none() {
        let response = GetServicesResponse {
            status: 401,
            services: None
        };

        return HttpResponse::Ok().json(response);
    }

    let user = user_optn.unwrap();

    let response = if request_unwrapped.only_owned {
        


    } else {
        response = GetServicesResponse {
            status: 200,
            services: Some(data.services_configs.clone())
        }
    };
}
