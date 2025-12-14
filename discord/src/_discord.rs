use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use reqwest::Client;

pub struct DiscordBot {
    pub token: String,
}

impl DiscordBot {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn run(&self) {
        let url = "wss://gateway.discord.gg/?v=10&encoding=json";
        let (ws_stream, _) = connect_async(url).await.expect("WebSocket failed");
        println!("Connected to Discord Gateway");

        let (mut write, mut read) = ws_stream.split();

        // Identify Event
        let identify = json!({
            "op": 2,
            "d": {
                "token": self.token,
                "intents": 513,
                "properties": {
                    "$os": "Windows 11 Pro",
                    "$browser": "my_bot",
                    "$device": "my_bot"
                }
            }
        });
        write.send(Message::Text(identify.to_string())).await.unwrap();

        // Event-Loop
        while let Some(msg) = read.next().await {
            let msg = msg.unwrap();
            if let Message::Text(txt) = msg {
                let json_msg: serde_json::Value = serde_json::from_str(&txt).unwrap();
                if let Some(event_type) = json_msg.get("t") {
                    if event_type == "MESSAGE_CREATE" {
                        let content = &json_msg["d"]["content"];
                        let channel_id = &json_msg["d"]["channel_id"];
                        println!("Message received: {} in channel {}", content, channel_id);

                        // Hier könnte man Message-Antwort senden (REST)
                    }
                }
            }
        }
    }

    pub async fn send_message(&self, channel_id: &str, content: &str) {
        let client = Client::new();
        let url = format!("https://discord.com/api/v10/channels/{}/messages", channel_id);
        let body = json!({ "content": content });

        let res = client.post(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(_) => println!("Message sent!"),
            Err(e) => eprintln!("Error sending message {}", e)
        }
    }
}
