use std::collections::HashMap;
use std::vec;
use std::sync::Arc;
use std::fmt;

use futures::io::empty;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use reqwest::Client;
use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use super::super::Intents;

pub mod _events;

// -------------------------------
// User Structure (like discord.py)
// -------------------------------

pub struct User {
    pub id: String,
    pub username: Option<String>,
}

impl User {
    pub fn new(id: String) -> Self {
        Self {
            id,
            username: None,
        }
    }

    pub fn with_username(id: String, username: String) -> Self {
        Self {
            id,
            username: Some(username),
        }
    }

    /// Return the mention string for this user
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref username) = self.username {
            write!(f, "{}", username)
        } else {
            write!(f, "<@{}>", self.id)
        }
    }
}

// -------------------------------
// Context Structure (like discord.py)
// -------------------------------

pub struct Context {
    pub interaction_id: String,
    pub interaction_token: String,
    pub application_id: String,
    pub client: reqwest::Client,
    pub deferred: Arc<Mutex<bool>>,
    pub interaction_data: Value,
}

impl Context {
    pub fn new(
        interaction_id: String,
        interaction_token: String,
        application_id: String,
        interaction_data: Value,
    ) -> Self {
        Self {
            interaction_id,
            interaction_token,
            application_id,
            client: Client::new(),
            deferred: Arc::new(Mutex::new(false)),
            interaction_data,
        }
    }

    /// Defer the response (think time to process the command)
    pub async fn defer(&self) {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            self.interaction_id, self.interaction_token
        );

        let body = json!({
            "type": 5
        });

        let _ = self.client
            .post(&url)
            .json(&body)
            .send()
            .await;

        let mut deferred = self.deferred.lock().await;
        *deferred = true;
    }

    /// Send the initial response to the interaction
    pub async fn respond(&self, content: &str) {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            self.interaction_id, self.interaction_token
        );

        let response_type = if *self.deferred.lock().await { 4 } else { 4 };

        let body = json!({
            "type": response_type,
            "data": { "content": content }
        });

        let _ = self.client
            .post(&url)
            .json(&body)
            .send()
            .await;
    }

    /// Send a followup message
    pub async fn followup(&self, content: &str) {
        let url = format!(
            "https://discord.com/api/v10/webhooks/{}/{}",
            self.application_id, self.interaction_token
        );

        let body = json!({
            "content": content
        });

        let _ = self.client
            .post(&url)
            .json(&body)
            .send()
            .await;
    }

    /// Get the user ID from interaction data (first option value)
    pub fn get_user_id(&self, interaction_data: &Value) -> Option<String> {
        interaction_data
            .get("data")
            .and_then(|d| d.get("options"))
            .and_then(|opts| opts.get(0))
            .and_then(|opt| opt.get("value"))
            .and_then(|val| val.as_str())
            .map(|s| s.to_string())
    }

    /// Get a User object from interaction data
    pub fn get_user(&self, interaction_data: &Value) -> Option<User> {
        self.get_user_id(interaction_data)
            .map(|id| User::new(id))
    }

    /// Send a mention message for a user
    pub async fn mention_user(&self, user_id: &str, message: &str) {
        let mention = format!("<@{}>", user_id);
        let content = format!("{} {}", mention, message);
        self.respond(&content).await;
    }
}

struct SlashCommand<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub options: Vec<Value>,
    pub handler: Box<dyn Fn(&Value, Context) + Send + Sync + 'a>,
}

// -------------------------------
// Bot Structure
// -------------------------------

pub struct Bot<'a> {
    pub intents: Intents,
    token: String,
    client: Client,
    slash_commands: Vec<SlashCommand<'a>>,
    application_id: String,
}

impl<'a> Bot<'a> {
    pub fn new(intents: Intents) -> Self {
        Self {
            intents,
            token: String::new(),
            client: Client::new(),
            slash_commands: Vec::new(),
            application_id: String::new(),
        }
    }

    // Add a slash command with Context parameter
    pub fn add_slash_command<F>(&mut self, name: &'a str, description: &'a str, options: Vec<Value>, handler: F)
    where
        F: Fn(&Value, Context) + Send + Sync + 'a,
    {
        self.slash_commands.push(SlashCommand {
            name,
            description,
            options,
            handler: Box::new(handler),
        });
    }

    // Fetch Application ID
    pub async fn fetch_application_id(&mut self) {
        let res = self.client
            .get("https://discord.com/api/v10/users/@me")
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await
            .expect("Failed to contact Discord API");

        let data: Value = res.json().await.unwrap();
        self.application_id = data["id"].as_str().unwrap().to_string();
    }

    // Register all slash commands
    async fn register_commands(&self) {
        let url = format!(
            "https://discord.com/api/v10/applications/{}/commands",
            self.application_id
        );

        for cmd in &self.slash_commands {
            let body = json!({
                "name": cmd.name,
                "description": cmd.description,
                "options": cmd.options,
                "type": 1
            });

            let _ = self.client
                .post(&url)
                .header("Authorization", format!("Bot {}", self.token))
                .json(&body)
                .send()
                .await;
        }
    }

    // Send a response to an interaction
    pub async fn respond_interaction(&self, id: &str, token: &str, content: &str) {
        let url = format!("https://discord.com/api/v10/interactions/{}/{}/callback", id, token);

        let body = json!({
            "type": 4,
            "data": { "content": content }
        });

        let _ = self.client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await;
    }

    // Main bot entry
    pub async fn run(&mut self, token: String) {
        self.token = token;

        // 1. Fetch Application ID
        self.fetch_application_id().await;

        // 2. Register Slash Commands
        self.register_commands().await;

        // 3. Connect to Gateway
        let url = "wss://gateway.discord.gg/?v=10&encoding=json";
        let (ws_stream, _) = connect_async(url).await.unwrap();
        let (write_raw, mut read) = ws_stream.split();

        // Writer thread-safe machen
        let write = Arc::new(Mutex::new(write_raw));
        // Identify payload
        let identify = json!({
            "op": 2,
            "d": {
                "token": self.token,
                "intents": self.intents.to_bitmask(),
                "properties": {
                    "$os": "linux",
                    "$browser": "rust-bot",
                    "$device": "rust-bot"
                }
            }
        });

        let mut guard = write.lock().await;

        guard.send(Message::Text(identify.to_string())).await.unwrap();

        // Beispiel: Heartbeat-Task (benutzt Arc clone)
        let write_clone = Arc::clone(&write);
        // last_seq könnte z.B. Arc<Mutex<Option<i64>>> sein; hier nur demo
        let last_sequence: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));

        tokio::spawn({
            let write = write_clone;
            let seq = Arc::clone(&last_sequence);
            async move {
                // Beispielintervall; normalerweise vom OP 10 (hello) kommen
                let interval_ms = 41250u64;
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;

                    // hole seq (nur für demo)
                    let current_seq = *seq.lock().await;

                    let hb = json!({
                        "op": 1,
                        "d": current_seq
                    });

                    // lock holen, senden, lock freigeben
                    let mut guard = write.lock().await;
                    if let Err(e) = guard.send(Message::Text(hb.to_string())).await {
                        eprintln!("Heartbeat failed: {}", e);
                        break;
                    }
                    // guard fällt raus, lock freigegeben
                }
            }
        });

        // Main event loop
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(txt)) = msg {
                let json_msg: Value = serde_json::from_str(&txt).unwrap();
                // Beispiel: Hello Event abwarten
                if json_msg["op"] == 10 {
                    let interval = json_msg["d"]["heartbeat_interval"].as_u64().unwrap(); // u64
                    let write_clone = Arc::clone(&write);
                    let last_sequence_clone = Arc::clone(&last_sequence);

                    tokio::spawn(async move {
                        // Beispielintervall; normalerweise vom OP 10 (hello) kommen
                        loop {
                            tokio::time::sleep(std::time::Duration::from_millis(interval)).await;

                            // hole seq (nur für demo)
                            let current_seq = *last_sequence_clone.lock().await;

                            let hb = json!({
                                "op": 1,
                                "d": current_seq
                            });

                            // lock holen, senden, lock freigeben
                            let mut guard = write_clone.lock().await;
                            if let Err(e) = guard.send(Message::Text(hb.to_string())).await {
                                eprintln!("Heartbeat failed: {}", e);
                                break;
                            }
                            // guard fällt raus, lock freigegeben
                        }
                    });
                }


                // Interaction Create (Slash Commands)
                if json_msg["t"] == "INTERACTION_CREATE" {
                    let data = &json_msg["d"];
                    let name = data["data"]["name"].as_str().unwrap();
                    let interaction_id = data["id"].as_str().unwrap().to_string();
                    let interaction_token = data["token"].as_str().unwrap().to_string();

                    if let Some(cmd) = self.slash_commands.iter().find(|c| c.name == name) {
                        let ctx = Context::new(
                            interaction_id,
                            interaction_token,
                            self.application_id.clone(),
                            data.clone(),
                        );
                        (cmd.handler)(data, ctx);
                    }
                }
            }
        }
    }
}