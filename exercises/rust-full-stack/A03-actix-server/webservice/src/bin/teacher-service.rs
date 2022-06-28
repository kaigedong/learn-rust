#[path = "../db_access.rs"]
mod db_access;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::*;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;
use std::io;
use std::sync::Mutex;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 从.env文件中加载环境变量
    dotenv().ok();
    // 从环境变量中读取
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在.env中配置");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
