use anyhow::Result;
use serde_json;
use tiny_http;

use rest::{MySendBody, MyRecvBody};

const SERVER: &str = "0.0.0.0:8000";

fn main() -> Result<()> {
    use tiny_http::{Response, Server};

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
                let res = handler(body)?;
                serde_json::to_string(&res)?
            },
            Err(e) => { anyhow::bail!("Error {e}") },
        };

        let response = Response::from_string(res);
        request.respond(response).expect("Responded");
    }

    Ok(())
}

fn handler(body: String) -> Result<MyRecvBody> {
    let data: MySendBody = serde_json::from_str(&body)?;
    println!("request: {}", data.thing);
    Ok(MyRecvBody{other: "good-bye".to_string()})
}
