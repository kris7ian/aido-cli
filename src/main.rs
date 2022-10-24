use clap::CommandFactory;
use clap::Parser;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use home::home_dir;
use reqwest;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use spinners_rs::{Spinner, Spinners};
use std::fs;
use tokio;


#[derive(Parser, Debug)]
#[clap(author, version, about, trailing_var_arg = true)]
struct Cli {
    /// Explain a shell command
    #[clap(short, long)]
    explain: bool,
    /// Copy the command to the clipboard
    #[clap(short, long)]
    clipboard: bool,
    /// Description of what you want to do
    #[clap(multiple_values = true, allow_hyphen_values = false)]
    description: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    result: String,
    message: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn too_many_requests(){
    println!("\n\nNo free requests left! Please visit http://getaido.app for more info.\n");
}

fn unauthorized(){
    println!("Unauthorized request, please visit http://getaido.app for more info.");
}

fn other_error(other: &StatusCode){
    println!(
        "Uh oh! Looks like we have problems with our server: {:?}",
        other
    );
}

async fn get_command(args: Cli, host: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Looking up your command...");
    sp.start();

    let description = args.description.join(" ");
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
                    if args.clipboard {
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

async fn explain_command(args: Cli, host: &str) {
    let mut sp = Spinner::new(Spinners::Dots, "Looking up explanation...");
    sp.start();

    let description = args.description.join(" ");
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

fn first_usage(){
    let home_path = home_dir();
    let aido_path = home_path.unwrap().join(".aido");
    let _result = fs::create_dir_all(aido_path);
    let home_path = home_dir();
    let file_path = home_path.unwrap().join(".aido/.intro");
    std::fs::File::create(file_path).expect("File creation failed.");

    println!("\n---------------------------------\n");
    println!("✨✨✨ Welcome to aido! ✨✨✨\n");
    println!("IMPORTANT:");
    println!("Aido uses a deeplearning model to automatically generate the command that you are looking for. Auto-generated commands can be dangerous because they can easily include syntax errors that can cause problems when the commands are executed. In addition, auto-generated commands can sometimes generate unexpected results that can be difficult to troubleshoot. Please always check the command before executing it.");
    println!("\nBy using this service, you agree that getaido.app is not to be held liable for any decisions you make or commands executed based on any of our services.\n");
    println!("This welcome message will only be shown once on the first usage. If you encounter any issues please file a github issue (https://github.com/kris7ian/aido-cli/issues).\n");
    println!("You can start using aido now.\n");
    println!("---------------------------------\n\n");
}

fn is_first_usage() -> bool{
    let home_path = home_dir();
    return !home_path.unwrap().join(".aido/.intro").exists();
}

#[tokio::main]
async fn main() {
    let host: &str;
    if cfg!(debug_assertions) {
        host = "http://127.0.0.1:8000";
    } else {
        host = "https://getaido.app";
    }

    if is_first_usage(){
        first_usage();
        return;
    }

    let args = Cli::parse();

    if args.description.len() == 0 {
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
    } else {
        if args.explain {
            explain_command(args, host).await;
        } else {
            get_command(args, host).await;
        }
    }
}
