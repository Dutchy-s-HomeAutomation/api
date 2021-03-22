pub mod register;
pub mod login;

use serde::Serialize;

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
