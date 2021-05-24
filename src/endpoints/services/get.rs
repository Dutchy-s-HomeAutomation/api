use crate::appdata::AppData;
use crate::config::ServicesConfig;

use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GetServicesResponse {
    status:         i16,
    services:       Option<Vec<crate::config::ServicesConfig>>
}

#[derive(Serialize)]
pub struct GetUserServiceResponse {
    status:         i16,
    services:       Vec<UserService>
}

#[derive(Serialize)]
pub struct UserService {
    service_id:     String,
    config:         ServicesConfig,
}

#[derive(Deserialize)]
pub struct GetServicesRequest {
    session_id:     String,
    only_owned:     bool
}

#[post("/services/get")]
pub async fn post_get(data: web::Data<AppData>, bytes: web::Bytes) -> HttpResponse {
    // payload is a stream of Bytes objects
    let body = String::from_utf8(bytes.to_vec());
    let body_unwrapped = body.unwrap();

    let request: Result<GetServicesRequest, serde_json::error::Error> = serde_json::from_str::<GetServicesRequest>(&body_unwrapped);
    if request.is_err() {
        return HttpResponse::BadRequest().body(request.err().unwrap().to_string());
    }

    let request_unwrapped = request.unwrap();

    let user_result = crate::common::user::get_user(&request_unwrapped.session_id, &data);
    if user_result.is_err() {
        eprintln!("An error occurred: {:?}", user_result.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user_option = user_result.unwrap();
    if user_option.is_none() {
        let response = GetServicesResponse {
            status: 401,
            services: None
        };

        return HttpResponse::Ok().json(response);
    };

    if request_unwrapped.only_owned {
        let user = user_option.unwrap();

        let all_user_services_result = crate::common::service::get_services(data.database.clone(), user.user_id);
        if all_user_services_result.is_err() {
            eprintln!("An error occurred: {:?}", all_user_services_result.err());
            return HttpResponse::InternalServerError().finish();
        }

        let services = all_user_services_result.unwrap();

        let mut result: Vec<UserService> = vec![];
        for item in services {
            let service_id = item.0.as_str();
            let service_type = item.1;

            let service_configs = data.services_configs.clone();
            for config in service_configs {
                if config.identifier == service_type {
                    let service_id_clone = service_id.to_string().clone();

                    result.push(UserService {
                        service_id: service_id_clone,
                        config
                    })
                }
            }
        }

        let response = GetUserServiceResponse {
            status: 200,
            services: result
        };

        return HttpResponse::Ok().json(response);
    } else {
        let response = GetServicesResponse {
            status: 200,
            services: Some(data.services_configs.clone())
        };

        return HttpResponse::Ok().json(response);
    }
}
