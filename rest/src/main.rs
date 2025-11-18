mod greet;

use std::collections::HashMap;

use anyhow::Result;
use serde_json;
use tiny_http::{Response, Server};

use rest::{RestReq, RestRes};

const SERVER: &str = "0.0.0.0:8000";

type CommandHandler = fn(req: RestReq) -> Result<RestRes>;

fn main() -> Result<()> {
    // use tiny_http::{Response, Server};
    let mut handlers: HashMap<&str, CommandHandler> = HashMap::new();
    handlers.insert(greet::COMMAND, greet::handle);

    let server = Server::http(SERVER).unwrap();

    for mut request in server.incoming_requests() {
        println!(
            "----------------\nreceived request!\n  method: {:?}\n  url: {:?}\n----------------",
            request.method(),
            request.url(),
        );
        let mut body = String::new();
        let res = match request.as_reader().read_to_string(&mut body) {
            Ok(_) => {
                let req: RestReq = serde_json::from_str(body.as_str())?;
                let res = if let Some(func) = handlers.get(req.command.as_str()) {
                    func(req)?
                } else {
                    anyhow::bail!("unknown: {}", req.command)
                };
                serde_json::to_string(&res)?
            },
            Err(e) => { anyhow::bail!("Error {e}") },
        };

        let response = Response::from_string(res);
        request.respond(response).expect("Responded");
    }

    Ok(())
}

