use std::env;

use futures_util::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use ingvar::contract::{ ServerMessage, ResponseToServer };

type WSSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let room_key = &args[1];
    println!("{}", room_key);
    let token = std::env::var("TETRIS_TOKEN").expect("missing token");
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
