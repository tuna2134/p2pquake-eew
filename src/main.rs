use native_tls::TlsConnector;
use serde_json::Value;

use std::net::TcpStream;

mod webhook;
use crate::webhook::Webhook;


fn main() {
    let webhook = Webhook::new(String::from("webhook_url"));
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("api.p2pquake.net:443").unwrap();
    let stream = connector.connect("api.p2pquake.net", stream).unwrap();
    let wsurl = String::from("wss://api.p2pquake.net/v2/ws");
    let (mut ws, res) = tungstenite::client::client(&wsurl, stream).unwrap();
    println!("Response HTTP code: {}", res.status());
    webhook.send("接続しました。");
    loop {
        let msg = ws.read_message().unwrap();
        if msg.is_text() {
            let data: Value = serde_json::from_str(&msg.to_text().unwrap()).unwrap();
            if data["code"] == 554 {
                webhook.send("地震がきました。");
            }
        }
    }
}