use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    print!("taskr running on http://localhost:4000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "taskr is alive"
}

async fn health() -> &'static str {
    "ok"
}

async fn list_tasks() -> &'static str {
    " a list of tasks will go here"
}

async fn create_task() -> &'static str {
    "create task here"
}
