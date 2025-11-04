use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;
use std::time::Duration; 
use crate::indexer::order_mapper::{OrderbookState, get_orderbook_snapshot};

pub type AppState = (Arc<Mutex<OrderbookState>>, PgPool);

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State((orderbook, _pool)): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, orderbook))
}

// on upgrade is here
pub async fn handle_socket(
    socket: WebSocket,
    State((orderbook, pool)): State<AppState>,
) {

    let (mut sender, mut receiver) = socket.split();

    // now we send initial snapshot

    {
        let ob = orderbook.lock().await;
        let snapshot = get_orderbook_snapshot(&ob);

        if let Ok(json) = serde_json::to_string(&snapshot){
            if sender.send(Message::Text(json)).await.is_err(){
                println!("Failed to send intiial snapshot. Get Fucked");
                return;
            }
            println!(" Sent initial snapshot");
        }
    }
    let interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        tokio::select!{
            _ = interval.tick().await => {
                let ob = orderbook.lock().await;
                let snapshot = get_orderbook_snapshot(&ob);
        
                if let Ok(json) = serde_json::to_string(&snapshot) {
                    if sender.send(Message::Text(json)).await.is_err() {
                        println!("Failed to send initial snapshot");
                        break;
                    }
                    println!(" Sent initial snapshot");
                }
            }

            msg = receiver.next() {
                match msg {
                    Some(Ok(Message::Close(_))) => {
                        println!(" Client closed connection");
                        break;
                    }
                    Some(Ok(Message::Ping(data))) => {
                        sender.send(Message::Pong(data)).await;
                    }
                    None => {
                        println!("Connection Lost");
                        break;
                    }
                }
            } 
        }
    }
    println!("WebSocket connection closed");  
}