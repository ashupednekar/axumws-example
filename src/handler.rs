use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
        Path
    }, response::IntoResponse
};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use crate::state::AppState;
use log;

pub async fn handle_ws(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(service): Path<String>,
) -> impl IntoResponse{
    ws.on_upgrade(|socket| handle(socket, state, service))
}

async fn handle(stream: WebSocket, state: Arc<AppState>, service: String){
    let (mut sender, mut receiver) = stream.split();

    let mut uid = String::new();

    //Receive Loop
    while let Some(Ok(message)) = receiver.next().await {
        log::info!("received: {:?} for {}", message, service);
    }
}
