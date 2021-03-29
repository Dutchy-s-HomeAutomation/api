use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub service_type:       ServiceType,
    pub has_password_auth:  bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordProtectedService {
    pub service_type:       ServiceType,
    pub username:           String,
    pub password:           String
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServiceType {
    HONEYWELL
}