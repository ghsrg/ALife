use crate::viewer_server::broadcaster::WsMessage;
use crate::viewer_server::state::{AppState, status_ws_text_from_state};
use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use tokio::sync::broadcast::error::RecvError;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/stream", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws_client(socket, state))
}

async fn handle_ws_client(mut socket: WebSocket, state: AppState) {
    let mut receiver = {
        let locked = state.lock().unwrap();
        locked.broadcaster.subscribe()
    };

    let initial_status = {
        let locked = state.lock().unwrap();
        status_ws_text_from_state(&locked)
    };
    if socket
        .send(Message::Text(initial_status.into()))
        .await
        .is_err()
    {
        return;
    }

    let latest_frame = {
        let locked = state.lock().unwrap();
        locked.latest_frame.clone()
    };
    if let Some(bytes) = latest_frame {
        if socket.send(Message::Binary(bytes.into())).await.is_err() {
            return;
        }
    }

    loop {
        tokio::select! {
            received = receiver.recv() => {
                match received {
                    Ok(WsMessage::Frame(bytes)) => {
                        if socket.send(Message::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                    Ok(WsMessage::Status(text)) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => break,
                }
            }
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
        }
    }
}
