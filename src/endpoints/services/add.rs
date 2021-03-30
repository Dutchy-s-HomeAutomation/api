use actix_web::{web, post, HttpResponse};
use crate::appdata::AppData;
use crate::types::service::{Service, ServiceType, PasswordProtectedService};
use serde::{Serialize, Deserialize};
use crate::common::service::Credentials;

#[derive(Serialize)]
pub struct AddServiceResponse {
    status:         i16
}

#[derive(Deserialize)]
pub struct AddServiceRequest<T> {
    session_id:     String,
    service:        T
}

#[post("/services/add")]
pub async fn post_add(data: web::Data<AppData>, bytes: web::Bytes) -> HttpResponse {
    // payload is a stream of Bytes objects
    let body = String::from_utf8(bytes.to_vec());
    let body_unwrapped = body.unwrap();

    let basic_add_request = serde_json::from_slice::<AddServiceRequest<Service>>(&body_unwrapped.as_bytes());
    if basic_add_request.is_err() {
        eprintln!("Unable to deserialize request payload: {:?}", basic_add_request.err());
        return HttpResponse::InternalServerError().finish();
    }

    let req_unwr = basic_add_request.unwrap();

    let user_wrapped = crate::common::user::get_user(&req_unwr.session_id, &data);
    if user_wrapped.is_err() {
        eprintln!("Unable to verify session_id: {:?}", user_wrapped.err());
        return HttpResponse::InternalServerError().finish();
    }

    let user = user_wrapped.unwrap();
    if user.is_none() {
        let response = AddServiceResponse { status: 401 };
        return HttpResponse::Ok().json(response);
    }

    let user_unwrapped = user.unwrap();

    let service = req_unwr.service.clone();
    match service.service_type {
        ServiceType::HONEYWELL => {
            let add_honeywell_service = serde_json::from_slice::<AddServiceRequest<PasswordProtectedService>>(body_unwrapped.as_bytes());
            if add_honeywell_service.is_err() {
                eprintln!("Unable to deserialize request payload as HONEYWELL");
                return HttpResponse::InternalServerError().finish();
            }

            let service = add_honeywell_service.unwrap().service.clone();

            //Do a test login
            let login_response = crate::services::honeywell::do_test_login(service.username.clone(), service.password.clone());

            //Check if any errors occurred
            if login_response.is_err() {
                let response = AddServiceResponse { status: 600 };
                return HttpResponse::Ok().json(response);
            }

            //Check if the login was successful
            if login_response.unwrap().is_none() {
                let response = AddServiceResponse { status: 700 };
                return HttpResponse::Ok().json(response);
            }

            //Test login was successful. Add the credentials to the database
            let put_response = crate::common::service::set_password_credentials(data.database.clone(), user_unwrapped.user_id.clone(), Credentials { password: service.password.clone(), username: service.username.clone() }, ServiceType::HONEYWELL);
            if put_response.is_err() {
                return HttpResponse::InternalServerError().finish();
            }

            let response = AddServiceResponse { status: 200 };
            return HttpResponse::Ok().json(response);
        }
    }
}