use anyhow::Result;

use crate::{RestReq, RestRes};

pub fn handle(req: &RestReq) -> Result<RestRes> {
    println!("anyhow error!");
    anyhow::bail!("anyhow error happen");
}
