use anyhow::Result;
use serde_json;
use tiny_http::{Response, Server};

use crate::{cmd, RestReq};

pub async fn rest_main(port: u16) -> Result<()> {
    let handlers = cmd::register_handle();

    let server = Server::http(format!("0.0.0.0:{port}")).unwrap();

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
