use std::io::{Read, stdin};
use std::{str, thread};
use serde_json::{json, Value};
use tokio;

use crate::utils::get_server_info;

pub mod utils;

// An example
#[tokio::main]
async fn main() {
    let server_info = get_server_info("mc.hypixel.net", 25565)
        .await
        .unwrap();
    let players = &server_info["players"];

    println!("Online: {}\nProtocol: {}\nBrand: {}", players["online"], server_info["version"]["protocol"], server_info["version"]["name"]);
}