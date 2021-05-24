use crate::appdata::AppData;

use mysql::prelude::Queryable;
use mysql::{Row, Params, params, Error};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub user_id:        String,
}

/**
Get a User object from a session_id

## Parameters
session_id: The session_id to use in the lookup
appdata: AppData instance

## Returns
Some: Returned if everything went according to plan
None: No user was found for the provided session_id
*/
pub fn get_user(session_id: &str, appdata: &AppData) -> Result<Option<User>, Error> {
    let conn_wrapped = appdata.database.pool.get_conn();
    if conn_wrapped.is_err() {
        let err = conn_wrapped.err().unwrap();
        eprintln!("An error occurred while creating a connection to the Database: {:?}", err.to_string());
        return Err(err);

    }
    let mut conn = conn_wrapped.unwrap();
    let sql_fetch_result = conn.exec::<Row, &str, Params>("SELECT user_id FROM users WHERE session_id = :session_id", params! {
        "session_id" => session_id.clone()
    });

    if sql_fetch_result.is_err() {
        let err = sql_fetch_result.err().unwrap();
        eprintln!("An Error occurred while fetching the user_id from the Database: {:?}", err.to_string());
        return Err(err);
    }

    let result = sql_fetch_result.unwrap();
    if result.len() == 0 {
        return Ok(None);
    }

    let row = result.get(0).unwrap();
    let user_id = row.get::<String, &str>("user_id").unwrap();

    Ok(Some(User {
        user_id
    }))
}