use serde::{Serialize, Deserialize};
use std::fmt;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LoginMethod {
    PASSWORD
}

impl fmt::Display for LoginMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ServiceType {
    HONEYWELL
}

impl std::str::FromStr for ServiceType {
    type Err = ();

    fn from_str(input: &str) -> Result<ServiceType, Self::Err> {
        match input {
            "HONEYWELL" => Ok(ServiceType::HONEYWELL),
            _           => Err(())
        }
    }
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}