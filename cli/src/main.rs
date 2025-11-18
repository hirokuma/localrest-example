use anyhow::Result;

use rest::{MySendBody, MyRecvBody};

const SERVER: &str = "http://127.0.0.1:8000";

fn main() -> Result<()> {
    let send_body = MySendBody {
        thing: "good morning".to_string(),
    };

    // Requires the `json` feature enabled.
    let recv_body = ureq::post(SERVER)
        .header("X-My-Header", "Secret")
        .send_json(&send_body)?
        .body_mut()
        .read_json::<MyRecvBody>()?;
    println!("response: {}", recv_body.other);
    Ok(())
}
