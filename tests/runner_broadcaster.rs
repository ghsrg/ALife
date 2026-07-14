use alife::viewer_server::broadcaster::{Broadcaster, WsMessage};
use tokio::sync::broadcast::error::RecvError;

#[tokio::test]
async fn broadcaster_sends_frame_to_single_subscriber() {
    let broadcaster = Broadcaster::new(16);
    let mut rx = broadcaster.subscribe();

    broadcaster.send_frame(vec![0x41, 0x4C, 0x49, 0x46]);

    assert!(matches!(rx.recv().await.unwrap(), WsMessage::Frame(_)));
}

#[tokio::test]
async fn broadcaster_sends_status_to_single_subscriber() {
    let broadcaster = Broadcaster::new(16);
    let mut rx = broadcaster.subscribe();

    broadcaster.send_status(r#"{"type":"status"}"#.to_string());

    assert!(matches!(rx.recv().await.unwrap(), WsMessage::Status(_)));
}

#[tokio::test]
async fn broadcaster_delivers_to_multiple_subscribers_independently() {
    let broadcaster = Broadcaster::new(16);
    let mut rx1 = broadcaster.subscribe();
    let mut rx2 = broadcaster.subscribe();

    broadcaster.send_frame(vec![1, 2, 3]);

    assert!(matches!(rx1.recv().await.unwrap(), WsMessage::Frame(_)));
    assert!(matches!(rx2.recv().await.unwrap(), WsMessage::Frame(_)));
}

#[tokio::test]
async fn slow_subscriber_gets_lagged_and_can_recover() {
    let broadcaster = Broadcaster::new(2);
    let mut rx = broadcaster.subscribe();

    broadcaster.send_frame(vec![1]);
    broadcaster.send_frame(vec![2]);
    broadcaster.send_frame(vec![3]);

    loop {
        match rx.recv().await {
            Ok(WsMessage::Frame(_)) => break,
            Ok(WsMessage::Status(_)) => panic!("unexpected status message"),
            Err(RecvError::Lagged(_)) => continue,
            Err(error) => panic!("unexpected receive error: {error:?}"),
        }
    }
}

#[test]
fn broadcaster_sender_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<tokio::sync::broadcast::Sender<WsMessage>>();
}
