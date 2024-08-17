use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    #[serde(rename = "authenticated")]
    Authenticated(Authenticated),

    #[serde(rename = "action")]
    Action(Action),

    #[serde[rename = "request_move"]]
    RequestMove(RequestMove),
}

#[derive(Deserialize, Debug)]
pub struct Authenticated {
    #[serde[rename = "sessionId"]]
    session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    #[serde[rename = "commands"]]
    commands: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RequestMove {
    #[serde[rename = "gameState"]]
    game_state: serde_json::Value,
    #[serde[rename = "players"]]
    players: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct ResponseToServerPayload {
    commands: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ResponseToServer {
    r#type: String,
    payload: ResponseToServerPayload,
}

impl ResponseToServer {
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn new(commands: Vec<String>) -> Self {
        Self {
            r#type: String::from("action"),
            payload: ResponseToServerPayload { commands: commands },
        }
    }

    pub fn add_command(&mut self, command: &str) {
        self.payload.commands.push(String::from(command));
    }
}