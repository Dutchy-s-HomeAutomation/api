use crate::database::Database;
use tera::Tera;

#[derive(Clone)]
pub struct AppData {
    pub database:           Database,
    pub tera:               Tera,

    /**
    OAuth Client credentials
    0: GOOGLE
    */
    pub oauth_credentials:  Vec<OAuthCredentials>
}

#[derive(Clone)]
pub struct OAuthCredentials {
    pub client_id:      String,
    pub client_secret:  String,
    pub identifier:     OAuthIdentifier
}

#[derive(Clone)]
pub enum OAuthIdentifier {
    GOOGLE
}