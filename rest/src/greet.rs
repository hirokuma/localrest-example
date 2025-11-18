use anyhow::Result;

use rest::{RestReq, RestRes};

pub const COMMAND: &str = "greet";

pub fn handle(req: RestReq) -> Result<RestRes> {
    println!("greeting message: {}", req.params);
    Ok(RestRes{response: "good-bye".to_string()})
}
