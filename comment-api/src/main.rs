mod db;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};
use db::{DbPool, init_db};
use handlers::{AppState, add_comment, delete_comment, get_comments};
use r2d2_sqlite::SqliteConnectionManager;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let manager = SqliteConnectionManager::file("comments.db");
    let pool = DbPool::new(manager).expect("DB 풀 생성 실패");

    init_db(&pool).expect("DB 초기화 실패");

    let state = AppState { pool };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/add", post(add_comment))
        .route("/list/{post}", get(get_comments))
        .route("/delete/{id}", post(delete_comment))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7777));
    println!("서버 실행중: http://{}", addr);
    let listener = TcpListener::bind(addr).await.expect("리스너 바인딩 실패");

    axum::serve(listener, app).await.unwrap();
}
