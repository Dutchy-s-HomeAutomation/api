use crate::common::service::{Credentials, get_password_credentials};
use crate::types::honeywell::LoginResponse;
use crate::database::Database;

const HONEYWELL_LOGIN_ENDPOINT: &str = "https://international.mytotalconnectcomfort.com/api/accountApi/login";

pub struct HoneywellUser {
    pub access_token:   String,
    pub email:          String,
    pub username:       String
}

/**
Function to check the validity of Honeywell login credentials

## Parameters
    appdata: Instance of the AppData object
    username: The username/email to use when logging in
    password: The password to use when logging in

## Returns
    Err: If an error occurred
    None: If the login failed
    Some: If the login was successful
*/
pub fn do_test_login(username: String, password: String) -> Result<Option<()>, ()> {
    let credentials = Credentials {
        username,
        password
    };

    let login = login(credentials);

    if login.is_err() {
        eprintln!("An error occurred: {:?}", login.err());
        return Err(());
    }

    if login.unwrap().is_none() {
        return Ok(None);
    }

    Ok(Some(()))
}

pub fn do_login(db: Database, user_id: String) -> Result<Option<HoneywellUser>, ()> {
    let credentials = get_password_credentials(db, user_id.clone());

    if credentials.is_err() {
        eprintln!("An error occurred: {:?}", credentials.err());
        return Err(());
    }

    let credentials_unwrapped = credentials.unwrap();
    if credentials_unwrapped.is_none() {
        return Ok(None);
    }


    let login = login(credentials_unwrapped.unwrap());
    if login.is_err() {
        eprintln!("An error occurred: {:?}", login.err());
        return Err(());
    }

    return Ok(login.unwrap());
}

fn login(credentials: Credentials) -> Result<Option<HoneywellUser>, ()> {
    let login_payload = crate::types::honeywell::LoginRequest {
        redirect_uri: "".to_string(),
        form_errors: vec![],
        events: vec![],
        api_active: true,
        api_down: false,
        is_service_status_returned: true,
        email_address: credentials.username,
        password: credentials.password
    };

    let login_request = reqwest::blocking::Client::new().post(HONEYWELL_LOGIN_ENDPOINT).json(&login_payload).send();
    if login_request.is_err() {
        eprintln!("An error occured: {:?}", login_request.err());
        return Err(());
    }

    let unwrapped_request = login_request.unwrap();
    let cookies: Vec<reqwest::cookie::Cookie> = unwrapped_request.cookies().collect();
    let mut session: Option<String> = None;
    for cookie in cookies {
        match cookie.name() {
            "SessionCookie" => {
                session = Some(cookie.value().to_string());
            },
            _ => {}
        }
    }

    if session.is_none() {
        return Ok(None);
    }

    let response_deserialized: LoginResponse = unwrapped_request.json().unwrap();
    if response_deserialized.content.is_none() {
        return Ok(None);
    }

    let content_unwrapped = response_deserialized.content.unwrap();
    let user = HoneywellUser {
        access_token: session.unwrap().clone(),
        email: content_unwrapped.username.clone(),
        username: content_unwrapped.display_name.clone()
    };

    Ok(Some(user))
}