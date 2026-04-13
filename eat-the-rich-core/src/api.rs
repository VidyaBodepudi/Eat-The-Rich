use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::sync::broadcast;

/// Starts the Axum Server for the UI Dashboard (WebSockets) and internal REST triggers
pub async fn start_server() {
    // Scaffold broadcast channel for pushing trailing anomaly data to Dashboards
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/health", get(|| async { "Eat The Rich NLP Engine -> Online 🚀" }))
        .route("/ws", get(ws_handler))
        .with_state(tx);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌐 Dashboard API Server listening on {}", addr);
    
    // Unbind lifetime to run server in background loop
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}

/// The WebSocket handler for the Dashboard frontend
async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    println!("📱 UI Dashboard Connected via WebSocket");
    
    // Simulate pushing mock market anomaly data to the UI
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        let mock_payload = serde_json::json!({
            "type": "T1_ANOMALY",
            "ticker": "GME",
            "sentiment_shift": "+400%",
            "price_action": "flat",
            "source_volume": 1205
        });

        if socket.send(Message::Text(mock_payload.to_string().into())).await.is_err() {
            println!("📱 UI Dashboard Disconnected");
            break;
        }
    }
}
