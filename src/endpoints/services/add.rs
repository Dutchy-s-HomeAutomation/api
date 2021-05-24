use actix_web::{web, post, HttpResponse};
use crate::appdata::AppData;
use crate::types::service::{Service, ServiceType, PasswordProtectedService};
use serde::{Serialize, Deserialize};
use crate::common::service::Credentials;
use rand::Rng;

#[derive(Serialize)]
pub struct AddServiceResponse {
    status:         i16,
    service_id:     Option<String>
}

#[derive(Deserialize)]
pub struct AddServiceRequest<T> {
    session_id:     String,
    service:        T
}

#[post("/services/add")]
pub async fn post_add(data: web::Data<AppData>, bytes: web::Bytes) -> HttpResponse {
    //Get the Request's payload
    let body = String::from_utf8(bytes.to_vec());
    let body_unwrapped = body.unwrap();

    //First we deserialize the payload into a generic Service
    let basic_add_request = serde_json::from_slice::<AddServiceRequest<Service>>(&body_unwrapped.as_bytes());
    if basic_add_request.is_err() {
        eprintln!("Unable to deserialize request payload: {:?}", basic_add_request.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Unwrap the request
    let req_unwrapped = basic_add_request.unwrap();

    //Get the user connected to the provided session_id
    let user_wrapped = crate::common::user::get_user(&req_unwrapped.session_id, &data);
    if user_wrapped.is_err() {
        eprintln!("Unable to verify session_id: {:?}", user_wrapped.err());
        return HttpResponse::InternalServerError().finish();
    }

    //Unwrap the Result<> into an Option<>
    let user_option = user_wrapped.unwrap();

    //If user_option is None, the user doesn't exist, so we return a status 401
    if user_option.is_none() {
        let response = AddServiceResponse { status: 401, service_id: None };
        return HttpResponse::Ok().json(response);
    }

    //Unwrap the Option<> into a User
    let user = user_option.unwrap();

    //Get the Service object from the request payload
    let service = req_unwrapped.service.clone();

    //Every service type requires different actions to be taken
    //Check which service_type was requested, and do whatever needs to be done
    match service.service_type {
        ServiceType::HONEYWELL => {

            //Deserialize the payload again, this time into a PasswordProtectedService,
            //since Honeywell takes a Username/Password combination
            let add_honeywell_service = serde_json::from_slice::<AddServiceRequest<PasswordProtectedService>>(body_unwrapped.as_bytes());
            if add_honeywell_service.is_err() {
                eprintln!("Unable to deserialize request payload as HONEYWELL");
                return HttpResponse::InternalServerError().finish();
            }

            //Unwrap the request, get the service
            let service = add_honeywell_service.unwrap().service.clone();

            //Validate the credentials
            let login_response = crate::services::honeywell::do_test_login(service.username.clone(), service.password.clone());

            //Check if any errors occurred
            if login_response.is_err() {
                let response = AddServiceResponse { status: 600, service_id: None };
                return HttpResponse::Ok().json(response);
            }

            //Check if the login was successful
            //If the value is None, it wasn't
            if login_response.unwrap().is_none() {
                let response = AddServiceResponse { status: 700, service_id: None };
                return HttpResponse::Ok().json(response);
            }

            //Create a service ID
            let service_id: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(64).map(char::from).collect();

            //Create an entry in the services table
            let add_service_result = crate::common::service::create_service(data.database.clone(), user.user_id.clone(), service_id.clone(), ServiceType::HONEYWELL);
            if add_service_result.is_err() {
                eprintln!("An error occurred: {:?}", add_service_result.err());
                return HttpResponse::InternalServerError().finish();
            }

            //Insert the credentials into the Database
            let set_credentials_response = crate::common::service::set_password_credentials(data.database.clone(), service_id.clone(), Credentials { password: service.password.clone(), username: service.username.clone() });
            if set_credentials_response.is_err() {
                eprintln!("An error occurred: {:?}", set_credentials_response.err());
                return HttpResponse::InternalServerError().finish();
            }

            //Finally, formulate a response
            let response = AddServiceResponse { status: 200, service_id: Some(service_id) };
            return HttpResponse::Ok().json(response);
        }
    }
}