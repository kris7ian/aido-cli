use home::home_dir;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use spinners_rs::{Spinner, Spinners};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    auth_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RegistrationError {
    email: Option<Vec<String>>,
    password: Option<Vec<String>>,
}

fn create_token_file(token: &str){
    let home_path = home_dir();
    let file_path = home_path.unwrap().join(".aido/.token");
    let mut file = fs::File::create(&file_path).expect("Token file creation failed.");
    file.write_all(token.as_bytes()).expect("Error while writing to file");
}


pub async fn register(email: &str, password: &str) {
    let host: &str;
    if cfg!(debug_assertions) {
        host = "http://127.0.0.1:8000";
    } else {
        host = "https://getaido.app";
    }

    let mut sp = Spinner::new(Spinners::Dots, "Registering...");
    sp.start();

    let mut data = Map::new();
    data.insert(
        "email".to_string(),
        Value::String(email.to_string()),
    );
    data.insert("password".to_string(), Value::String(password.to_string()));
    data.insert("password_re".to_string(), Value::String(password.to_string()));

    let client = reqwest::Client::new();
    let result = client
        .post(host.to_owned() + "/auth/users/")
        .json(&data)
        .send()
        .await
        .unwrap();
    
    match result.status() {
        reqwest::StatusCode::CREATED => {
            sp.stop_with_message("Done ✓                        \n");
            println!("Successfully registered, please check your inbox to confirm your email!\nAfter confirming your email address you can login to your account by typing `aido login` and you'll get 20 free requests per day.");
        }
        reqwest::StatusCode::BAD_REQUEST => {
            sp.stop_with_message("Error ✗                        \n");
            match result.json::<RegistrationError>().await {
                Ok(result) => {
                    println!("\nThere were some problems with the registration:");
                    if let Some(msgs) = &result.email{
                        for message in msgs.iter() {
                            println!("- {}\n", message);
                        }
                    }
                    if let Some(msgs) = &result.password{
                        for message in msgs.iter() {
                            println!("- {}\n", message);
                        }
                    }
                }
                Err(_) => println!("There was an unexpected error, maybe you need to update aido."),
            };
        }
        _other => {
            // sp.stop_with_message("Error ✗                        \n");
            println!("Unexpected error.");
        }
    };
}


pub async fn login(email: &str, password: &str) {
    let host: &str;
    if cfg!(debug_assertions) {
        host = "http://127.0.0.1:8000";
    } else {
        host = "https://getaido.app";
    }

    let mut sp = Spinner::new(Spinners::Dots, "Logging in...");
    sp.start();

    let mut data = Map::new();
    data.insert(
        "email".to_string(),
        Value::String(email.to_string()),
    );
    data.insert("password".to_string(), Value::String(password.to_string()));

    let client = reqwest::Client::new();
    let result = client
        .post(host.to_owned() + "/auth/token/login/")
        .json(&data)
        .send()
        .await
        .unwrap();

    match result.status() {
        reqwest::StatusCode::OK => {
            match result.json::<Token>().await {
                Ok(token) => {
                    sp.stop_with_message("Done ✓                        \n");
                    create_token_file(&token.auth_token);
                    println!("Successfully authenticated!");
                }
                Err(_) => println!("There was an unexpected error, maybe you need to update aido."),
            };
        }
        reqwest::StatusCode::BAD_REQUEST => {
            println!("Email and/or password incorrect.");
        }
        _other => {
            println!("Unexpected error.");
        }
    };
}

pub fn logout(){
    let home_path = home_dir();
    let file_path = home_path.unwrap().join(".aido/.token");
    if file_path.exists() {
        fs::remove_file(file_path).expect("Logout failed.")
    }
}