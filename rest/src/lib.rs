use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MySendBody {
    pub thing: String,
}

#[derive(Serialize, Deserialize)]
pub struct MyRecvBody {
    pub other: String,
}
