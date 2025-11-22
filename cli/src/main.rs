use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use rest::RestReq;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
    /// Send greeting message
    Greet { msg: String },
    /// Something error
    Error,
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
        Some(Commands::Error) => Some(RestReq::new("error", "".to_string())),
    };

    if let Some(req) = req {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .build()
            .new_agent();
        let mut res = agent.post(format!("http://127.0.0.1:{}", cli.port)).send_json(&req)?;
        let status = res.status();
        let body = res.body_mut().read_json::<serde_json::Value>()?;

        let json_str = serde_json::to_string_pretty(&body)?;
        if status != ureq::http::StatusCode::OK {
            println!("status code: {}", status);
        }
        println!("{}", json_str);
    }


    Ok(())
}
