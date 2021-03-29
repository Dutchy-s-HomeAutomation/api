use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginResponse {
    pub content:        Option<Content>,
    pub errors:         Option<Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    pub username:       String,
    pub display_name:   String
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequest {
    pub email_address:              String,
    pub password:                   String,
    pub is_service_status_returned: bool,
    pub api_active:                 bool,
    pub api_down:                   bool,
    pub redirect_uri:               String,
    pub events:                     Vec<String>,
    pub form_errors:                Vec<String>
}