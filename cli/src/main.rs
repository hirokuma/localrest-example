use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use rest::{RestReq, RestRes};

const SERVER: &str = "http://127.0.0.1:8000";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Send greeting message
    Greet { msg: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let req: Option<RestReq> = match cli.command {
        None => {
            // clap will show help if user asks, but when no subcommand provided, print help
            Cli::command().print_help()?;
            println!();
            None
        }
        Some(Commands::Greet { msg }) => Some(RestReq::new("greet", msg)),
    };

    if let Some(req) = req {
        let res = ureq::post(SERVER)
            .header("X-My-Header", "Secret")
            .send_json(&req)?
            .body_mut()
            .read_json::<RestRes>()?;

        let json_str = serde_json::to_string_pretty(&res)?;
        println!("{}", json_str);
    }

    Ok(())
}
