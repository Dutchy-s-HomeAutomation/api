use actix_web::{post, web, HttpResponse, HttpRequest};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use crate::appdata::AppData;
use crate::types::assistant_incoming::{FulfillmentRequest, GenericFulfillmentInput, FulfillmentIntent};

#[post("/assistant/webhook")]
pub async fn post_webhook(data: web::Data<AppData>, req: HttpRequest, bytes: web::Bytes) -> HttpResponse {

    // payload is a stream of Bytes objects
    let body = String::from_utf8(bytes.to_vec());
    let body_unwrapped = body.unwrap();

    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_header_value = auth_header.unwrap().to_str().unwrap().to_string().clone();
    let auth_header_parts: Vec<&str> = auth_header_value.split(" ").collect();
    if !auth_header_parts.get(0).unwrap().to_string().eq("Bearer") {
        return HttpResponse::BadRequest().finish();
    }

    if auth_header_parts.len() != 2 {
        return HttpResponse::BadRequest().finish();
    }
    let access_token = auth_header_parts.get(1).unwrap().to_string();

    let conn = data.database.pool.get_conn();
    if conn.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let user_id_fetch_result = conn.unwrap().exec::<Row, &str, Params>("SELECT user_id,expiry FROM oauth_grants WHERE access_token = :access_token", params! {
        "access_token" => access_token
    });

    if user_id_fetch_result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let user_id_fetch_result_unwrapped = user_id_fetch_result.unwrap();
    if user_id_fetch_result_unwrapped.len() == 0 {
        return HttpResponse::Unauthorized().finish();
    }

    let first_row = user_id_fetch_result_unwrapped.get(0).unwrap();
    let user_id = first_row.get::<String, &str>("user_id").unwrap();
    let expiry = first_row.get::<i64, &str>("expiry").unwrap();

    let now = chrono::Utc::now().timestamp();
    if now >= expiry {
        return HttpResponse::Unauthorized().finish();
    }

    //Access token validated.
    let basic_fulfillment_request: FulfillmentRequest<GenericFulfillmentInput> = serde_json::from_slice(&body_unwrapped.clone().as_bytes()).unwrap();
    for input in basic_fulfillment_request.inputs {
        match input.intent {
            FulfillmentIntent::SYNC => {

            },
            FulfillmentIntent::QUERY => {

            }
        }
    }

    HttpResponse::Ok().finish()
}