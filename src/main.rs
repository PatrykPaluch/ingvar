use futures_util::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum ServerMessage {
    #[serde(rename = "authenticated")]
    Authenticated(Authenticated),

    #[serde(rename = "action")]
    Action(Action),

    #[serde[rename = "request_move"]]
    RequestMove(RequestMove),
}

#[derive(Deserialize, Debug)]
struct Authenticated {
    #[serde[rename = "sessionId"]]
    session_id: String,
}

#[derive(Deserialize, Debug)]
struct Action {
    #[serde[rename = "commands"]]
    commands: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct RequestMove {
    #[serde[rename = "gameState"]]
    game_state: serde_json::Value,
    #[serde[rename = "players"]]
    players: serde_json::Value,
}

type WSSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let token = std::env::var("TETRIS_TOKEN").expect("missing token");
    let room_key = std::env::var("TETRIS_ROOM_KEY").expect("missing room key");
    let url = format!("wss://botrisbattle.com/ws?token={token}&roomKey={room_key}");

    let (mut ws_stream, _) = connect_async(url).await.expect("cannot connect");

    // Receive messages from the server
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                parse_server_message(&mut ws_stream, &text).await;
            }
            Ok(Message::Binary(data)) => {
                println!("Got binary");
            }
            Ok(Message::Close(m)) => {
                println!("Closed by server {:?}", m);
            }
            Ok(_) => {
                println!("????");
            }
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
    println!("end");
}

async fn parse_server_message(ws_stream: &mut WSSocket, text: &str) {
    match serde_json::from_str::<ServerMessage>(text) {
        Ok(parsed_msg) => match parsed_msg {
            ServerMessage::Authenticated(auth) => {
                println!("Authenticated: {:?}", auth);
            }
            ServerMessage::Action(action) => {
                println!("Action: {:?}", action);
            }
            ServerMessage::RequestMove(_) => {
                println!("RequestMove");
                send_response(ws_stream).await;
            }
            _ => {
                println!("unsupported message");
            }
        },
        Err(e) => eprintln!("Failed to parse message: {}", e),
    }
}

#[derive(Serialize, Debug)]
struct ResponseToServerPayload {
    commands: Vec<String>,
}

#[derive(Serialize, Debug)]
struct ResponseToServer {
    r#type: String,
    payload: ResponseToServerPayload,
}

impl ResponseToServer {
    fn empty() -> Self {
        Self::new(vec![])
    }

    fn new(commands: Vec<String>) -> Self {
        Self {
            r#type: String::from("action"),
            payload: ResponseToServerPayload { commands: commands },
        }
    }

    fn add_command(&mut self, command: &str) {
        self.payload.commands.push(String::from(command));
    }
}

async fn send_response(ws_stream: &mut WSSocket) {
    let mut response = ResponseToServer::empty();
    response.add_command(if rand::random() {
        "move_left"
    } else {
        "move_right"
    });

    let json = serde_json::to_string(&response).expect("cannot create response json");

    //what have i done
    ws_stream
        .send(Message::Text(json))
        .await
        .and_then(|o| {
            println!("send response");
            Ok(())
        })
        .or_else(|e| {
            println!("cannot send response {e}");
            Ok::<(), ()>(())
        })
        .expect("fuck");
}
