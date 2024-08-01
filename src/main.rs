use axum::{
    routing::get, Router, serve
};
use std::env;

mod state;
mod handler;
mod backends;

#[tokio::main]
async fn main(){
    env_logger::init();  // TODO: add tracing
    let state = state::AppState::new();
    let app: Router = Router::new()
        .route("/ws/:service/", get(handler::handle_ws))
        .with_state(state);

    let port = env::var("WEBSOCKET_PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", &port)).await.unwrap();
    log::info!("listening at {}", &port);
    serve(listener, app).await.unwrap() 
}
