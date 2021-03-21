use actix_web::{web, get, HttpResponse, HttpRequest};
use crate::appdata::AppData;

pub async fn get_install(data: web::Data<AppData>, req: HttpRequest) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

}