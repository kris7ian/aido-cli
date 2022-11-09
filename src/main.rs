use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;
use dialoguer::Input;
use dialoguer::Password;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio;

mod authentication;
mod lib;

#[derive(Parser, Debug)]
#[clap(author, version, about, trailing_var_arg = true)]
struct Cli {
    /// Explain a shell command
    #[clap(short, long)]
    explain: bool,
    /// Copy the command to the clipboard
    #[clap(short, long)]
    clipboard: bool,
    /// Description of the command you want to generate
    #[clap(multiple_values = true, allow_hyphen_values = false)]
    description: Vec<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Register
    Register,
    /// Login with your email
    Login,
    /// Logout
    Logout,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    result: String,
    message: String,
}

fn first_usage() {
    let home_path = home_dir();
    let aido_path = home_path.unwrap().join(".aido");
    let _result = fs::create_dir_all(aido_path);
    let home_path = home_dir();
    let file_path = home_path.unwrap().join(".aido/.intro");
    std::fs::File::create(file_path).expect("File creation failed.");

    let text = "
    ---------------------------------
    ✨✨✨ Welcome to aido! ✨✨✨

    IMPORTANT:

    Aido uses a deeplearning model to automatically generate the command that you are looking for. Auto-generated commands can be dangerous because they can easily include syntax errors that can cause problems when the commands are executed. In addition, auto-generated commands can sometimes generate unexpected results that can be difficult to troubleshoot. Please always check the command before executing it.
    Please never input any sensitive data!

    By using this service, you agree that getaido.app is not to be held liable for any decisions you make or commands executed based on any of our services.

    This welcome message will only be shown once on the first usage. If you encounter any issues please send us an email to info@getaido.app.
    To see how to use aido type `aido` or `aido --help` into your console.
    ---------------------------------
    ";
    let text = textwrap::dedent(text);
    println!("{}", textwrap::fill(&text, 75));

}

fn is_first_usage() -> bool {
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

    if is_first_usage() {
        first_usage();
        return;
    }

    let args = Cli::parse();

    match &args.command {
        Some(Commands::Register) => {
            let register_text_1 = "\nRegistration is 100% free and we will never send you spam or sell your data!\nBy registering you get 20 free API calls per day.\n";
            let register_text_2 = "Please fill in your email address and your desired password you will receive a confirmation mail, after confirming you're email address you can login with `aido login` in the terminal.\n";
            println!("{}", textwrap::fill(register_text_1, 75));
            println!("{}", textwrap::fill(register_text_2, 75));

            let email: String = Input::new().with_prompt("Email").interact_text().unwrap();
            let password = &Password::new()
                .with_prompt("Password")
                .with_confirmation("Confirm Password", "Passwords don't match")
                .interact()
                .unwrap();
            authentication::register(&email, password).await;
            return;
        }
        Some(Commands::Login) => {
            let email: String = Input::new().with_prompt("Email").interact_text().unwrap();
            let password = &Password::new().with_prompt("Password").interact().unwrap();
            authentication::login(&email, password).await;
            return;
        }
        Some(Commands::Logout) => {
            authentication::logout();
            return;
        }
        None => {}
    }

    if args.description.len() == 0 {
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
    } else {
        let description = args.description.join(" ");
        if args.explain {
            lib::explain_command(&description, host).await;
        } else {
            lib::get_command(&description, host, args.clipboard).await;
        }
    }
}
