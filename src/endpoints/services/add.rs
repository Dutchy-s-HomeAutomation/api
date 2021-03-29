use actix_web::{web, post, HttpResponse};
use mysql::prelude::Queryable;
use mysql::{Params, params};
use crate::appdata::AppData;
use crate::types::service::{Service, ServiceType, PasswordProtectedService};
use crate::types::honeywell::LoginResponse;
use ring::{digest, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use serde::{Serialize, Deserialize};

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
        return HttpResponse::InternalServerError().finish();
    }

    let req_unwr = basic_add_request.unwrap();

    let user_wrapped = crate::logic::user::get_user(&req_unwr.session_id, &data);
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
                return HttpResponse::InternalServerError().finish();
            }

            let service = add_honeywell_service.unwrap().service.clone();

            //Log in
            let login_request = crate::types::honeywell::LoginRequest {
                redirect_uri: "".to_string(),
                form_errors: vec![],
                events: vec![],
                api_active: true,
                api_down: false,
                is_service_status_returned: true,
                email_address: service.username.clone(),
                password: service.password.clone()
            };

            let login_req = reqwest::blocking::Client::new().post("https://international.mytotalconnectcomfort.com/api/accountApi/login").json(&login_request).send();
            if login_req.is_err() {
                let response = AddServiceResponse { status: 600 };
                return HttpResponse::Ok().json(response);
            }

            let unwrapped_req = login_req.unwrap();


            /*
            let cookies: Vec<reqwest::cookie::Cookie> = unwrapped_req.cookies().collect();
            let mut session: String = String::new();
            let mut expires: Option<SystemTime> = None;
            for cookie in cookies {
                match cookie.name() {
                    "SessionCookie" => {
                        session = cookie.value().to_string();
                        expires = cookie.expires();
                    },
                    _ => {}
                }
            }
            */

            let login_response: LoginResponse = unwrapped_req.json().unwrap();
            if login_response.content.is_some() {
                //let dt: DateTime<Utc> = chrono::DateTime::from(expires.unwrap());
                //let expires_epoch = dt.timestamp();

                //Encrypt the password
                const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
                let n_iter = NonZeroU32::new(100_100).unwrap();
                let rng = rand::SystemRandom::new();

                let mut salt = [0u8, CREDENTIAL_LEN as u8];
                let salt_generation = rng.fill(&mut salt);
                if salt_generation.is_err() {
                    return HttpResponse::InternalServerError().finish();
                }

                let mut pbkdf2_password_hash = [0u8, CREDENTIAL_LEN as u8];
                pbkdf2::derive(
                    pbkdf2::PBKDF2_HMAC_SHA512,
                    n_iter,
                    &salt,
                    &service.password.clone().as_bytes(),
                    &mut pbkdf2_password_hash
                );

                let salt_base64 = base64::encode(&salt);
                let password_base64 = base64::encode(&pbkdf2_password_hash);

                //Insert into the database
                let conn = data.database.pool.get_conn();
                if conn.is_err() {
                    eprintln!("Unable to open a connection to the Database: {:?}", conn.err());
                    return HttpResponse::InternalServerError().finish();
                }

                let sql_insert = conn.unwrap().exec::<usize, &str, Params>("INSERT INTO api_passwords (user_id, username, password, salt) VALUES (:user_id, :username, :password, :salt)", params! {
                    "user_id" => user_unwrapped.user_id.clone(),
                    "username" => service.username.clone(),
                    "password" => password_base64,
                    "salt" => salt_base64
                });

                if sql_insert.is_err() {
                    eprintln!("An error occurred while inserting data into the Database: {:?}", sql_insert.err());
                    return HttpResponse::InternalServerError().finish();
                }

                let response = AddServiceResponse { status: 200 };
                return HttpResponse::Ok().json(response);
            } else {
                let response = AddServiceResponse { status: 700 };
                return HttpResponse::Ok().json(response);
            }
        }
    }
}