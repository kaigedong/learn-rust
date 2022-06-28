// use super::models::Course;
use sqlx::postgres::PgPool;
use std::sync::Mutex;

// 共享于所有线程
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    // 数据库的连接池
    pub db: PgPool,
}
