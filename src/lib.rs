use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use spinners_rs::{Spinner, Spinners};


#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    result: String,
    message: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn too_many_requests() {
    println!("\n\nNo free requests left! Please visit http://getaido.app for more info.\n");
}

fn unauthorized() {
    println!("Unauthorized request, please visit http://getaido.app for more info.");
}

fn other_error(other: &StatusCode) {
    println!(
        "Uh oh! Looks like we have problems with our server: {:?}",
        other
    );
}

pub async fn get_command(description: &str, host: &str, clipboard: bool) {
    let mut sp = Spinner::new(Spinners::Dots, "Looking up your command...");
    sp.start();

    let mut data = Map::new();
    data.insert(
        "description".to_string(),
        Value::String(description.to_string()),
    );
    data.insert("version".to_string(), Value::String(VERSION.to_string()));

    let client = reqwest::Client::new();
    let result = client
        .post(host.to_owned() + "/api/1/command/")
        .json(&data)
        .send()
        .await
        .unwrap();

    match result.status() {
        reqwest::StatusCode::OK => {
            match result.json::<APIResponse>().await {
                Ok(parsed) => {
                    sp.stop_with_message("Done ✓                        ");
                    println!("\n\n{:^5}\n", parsed.result);
                    if clipboard {
                        let mut ctx = ClipboardContext::new().unwrap();
                        ctx.set_contents(parsed.result.to_owned()).unwrap();
                        println!("Command copied to clipboard!");
                    }

                    if !parsed.message.is_empty() {
                        println!("{:}", parsed.message)
                    }
                }
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            unauthorized();
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            too_many_requests();
        }
        other => {
            other_error(&other);
        }
    };
}

pub async fn explain_command(description: &str, host: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Looking up explanation...");
    sp.start();

    let mut data = Map::new();
    data.insert(
        "command".to_string(),
        Value::String(description.to_string()),
    );
    data.insert("version".to_string(), Value::String(VERSION.to_string()));

    let client = reqwest::Client::new();
    let result = client
        .post(host.to_owned() + "/api/1/explain/")
        .json(&data)
        .send()
        .await
        .unwrap();

    match result.status() {
        reqwest::StatusCode::OK => {
            match result.json::<APIResponse>().await {
                Ok(parsed) => {
                    sp.stop_with_message("Done ✓                        ");
                    println!("\n\n{:^5}\n", parsed.result);

                    if !parsed.message.is_empty() {
                        println!("{:}", parsed.message)
                    }
                }
                Err(_) => println!("There was an unexpected error, maybe you need to update aido."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            unauthorized();
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            too_many_requests();
        }
        other => {
            other_error(&other);
        }
    };
}
