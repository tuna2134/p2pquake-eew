use native_tls::TlsConnector;
use serde_json::Value;

use std::net::TcpStream;


fn main() {
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("api.p2pquake.net:443").unwrap();
    let stream = connector.connect("api.p2pquake.net", stream).unwrap();
    let url = String::from("wss://api.p2pquake.net/v2/ws");
    let (mut ws, res) = tungstenite::client::client(&url, stream).unwrap();
    println!("Response HTTP code: {}", res.status());
    loop {
        let msg = ws.read_message().unwrap();
        if msg.is_text() {
            println!("{}", msg);
            let data: Value = serde_json::from_str(&msg.to_text().unwrap()).unwrap();
            if data["type"] == 554 {
                println!("{}", data["data"]);
            }
        }
    }
}