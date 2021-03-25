pub mod register;
pub mod login;
pub mod session;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginForm {
    pub email:      String,
    pub password:   String
}

#[derive(Serialize)]
pub struct LoginResponse {
    status:         i16,
    status_message: Option<String>,
    session_id:     Option<String>
}

impl LoginResponse {
    pub fn new(status: i16, status_message: Option<String>, session_id: Option<String>) -> LoginResponse {
        LoginResponse {
            status,
            status_message,
            session_id
        }
    }
}
