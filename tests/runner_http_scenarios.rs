use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use std::path::PathBuf;
use tower::ServiceExt;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        10,
        30,
    )
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn get_scenarios_returns_200() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn get_scenarios_returns_sorted_scenario_ids() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let json = body_json(response).await;
    let ids: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap())
        .collect();

    assert!(ids.contains(&"bootstrap_minimal_viable_world"));
    assert!(ids.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[tokio::test]
async fn get_scenario_by_id_returns_config_toml() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios/bootstrap_minimal_viable_world")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let json = body_json(response).await;
    assert_eq!(
        json["id"].as_str().unwrap(),
        "bootstrap_minimal_viable_world"
    );
    assert!(
        json["config_toml"]
            .as_str()
            .unwrap()
            .contains("scenario_id = \"bootstrap_minimal_viable_world\"")
    );
}

#[tokio::test]
async fn get_scenario_by_unknown_id_returns_404() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios/not_a_real_scenario")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn get_scenarios_includes_relative_path_metadata() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let json = body_json(response).await;
    let scenario = json
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == "bootstrap_minimal_viable_world")
        .unwrap();

    assert!(
        scenario["path"]
            .as_str()
            .unwrap()
            .ends_with("bootstrap/minimal_viable_world.toml")
    );
}
