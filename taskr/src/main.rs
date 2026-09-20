use crate::state::AppState;

mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    print!("taskr running on http://localhost:4000");
    axum::serve(listener, app).await.unwrap();
}
