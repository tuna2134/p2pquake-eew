use reqwest::blocking::Client;

use std::collections::HashMap;


pub struct Webhook {
    client: Client,
    url: String,
}

impl Webhook {
    pub fn new(url: String) -> Webhook {
        Webhook {
            client: Client::new(),
            url: url
        }
    }

    pub fn send(&self, message: &str) {
        let mut map = HashMap::new();
        map.insert("content", message);
        match self.client.post(&self.url).json(&map).send() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            },
        }
    }
}