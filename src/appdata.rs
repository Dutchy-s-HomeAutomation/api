use crate::database::Database;
use tera::Tera;

pub struct AppData {
    pub database:   Database,
    pub tera:       Tera
}