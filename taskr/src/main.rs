use axum::{
    Router,
    extract::{Path, Query},
    routing::get,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    print!("taskr running on http://localhost:4000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "taskr is alive"
}

async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String {
    let filter = params
        .get("done")
        .map_or("everything".to_string(), |v| format!("done={}", v));
    format!("listing tasks, filter: {}", filter)
}

async fn health() -> &'static str {
    "ok"
}

async fn create_task() -> &'static str {
    "create task here"
}

async fn get_task(Path(id): Path<u32>) -> String {
    format!("you asked for task {}", id)
}
