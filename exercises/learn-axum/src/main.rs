use axum::{
    routing::{get, post},
    Router, Server,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::{net::SocketAddr, sync::atomic::AtomicUsize};

mod extractors;
mod handlers;
mod responses;

const SECRET: &[u8] = b"deadbeef";
// 线程间共享变量的做法
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub user_id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Default, Clone)]
pub struct TodoStore {
    // 允许多线程读写
    pub items: Arc<RwLock<Vec<Todo>>>,
}

#[tokio::main]
async fn main() {
    let store = TodoStore::default();

    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/login", post(handlers::login_handler))
        .route(
            "/todos",
            get(handlers::todos_handler)
                .post(handlers::create_todo_handler)
                .with_state(store),
        )
        .fallback(handlers::static_handler);
    // 等价于:
    // use axum::handler::Handler;
    // .fallback(static_handler.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
