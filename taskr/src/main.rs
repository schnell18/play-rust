use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

fn seed_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 1,
            title: "Learn Axum".to_string(),
            done: false,
        },
        Task {
            id: 2,
            title: "Write a script".to_string(),
            done: false,
        },
        Task {
            id: 3,
            title: "Record a video".to_string(),
            done: false,
        },
    ]
}

async fn root() -> &'static str {
    "taskr is alive"
}

async fn list_tasks() -> Json<Vec<Task>> {
    Json(seed_tasks())
}

async fn health() -> &'static str {
    "ok"
}

async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let task = Task {
        id: 99,
        title: payload.title,
        done: false,
    };
    (StatusCode::CREATED, Json(task))
}

async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let tasks = seed_tasks();
    tasks
        .into_iter()
        .find(|t| t.id == id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

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
