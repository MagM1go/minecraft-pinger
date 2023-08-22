mod utils;

use utils::get_server_info;

// An example
#[tokio::main]
async fn main() {
    let server_info = get_server_info("mc.hypixel.net", 25565).await.unwrap();

    println!(
        "Online: {}\nProtocol: {}\nBrand: {}",
        server_info.players.online, server_info.version.protocol, server_info.version.name
    );
}
