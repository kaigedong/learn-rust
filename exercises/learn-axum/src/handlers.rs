use crate::{Todo, TodoStore, SECRET};
use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::Json;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

// 首页 `/` 路由
pub async fn index_handler() -> impl IntoResponse {
    // StaticFile("./index.html") // 等价于：
    static_handler("/index.html".parse().unwrap()).await
}

// 处理fallback
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    // 去掉uri的首use axum::handler::Handler;/
    let path = uri.path().trim_start_matches('/').to_string();
    crate::responses::StaticFile(path)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

// `/login` 路由
pub async fn login_handler(Json(_login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login validation: 如果用户正确传入email & passed，将为该用户生成一个Token
    let claims = crate::extractors::Claims {
        id: 1,
        exp: get_epoch() + 14 * 24 * 60 * 60, // 14天后过期
        name: "Kaige Dong".to_string(),
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

// get `/todos` 路由
pub async fn todos_handler(
    claims: crate::extractors::Claims,
    State(store): State<TodoStore>,
) -> Result<Json<Vec<Todo>>, crate::responses::HttpError> {
    let user_id = claims.id;
    match store.items.read() {
        Ok(items) => Ok(Json(
            items
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .cloned()
                // 使用cloned 代替 .map(|todo| todo.clone())
                .collect(),
        )),
        // 低效的做法:
        // Ok(items) => Ok(Json(
        //     items
        //         .clone()
        //         .into_iter()
        //         .filter(|todo| todo.user_id == user_id)
        //         .collect(),
        // )),
        Err(_) => Err(crate::responses::HttpError::Internal),
    }
}

fn get_epoch() -> usize {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn get_next_id() -> usize {
    crate::NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

// post `/todos` 路由
// NOTE: 0.6开始只允许最后一个extractor consume body
pub async fn create_todo_handler(
    claims: crate::extractors::Claims,
    State(store): State<TodoStore>,
    Json(todo): Json<CreateTodo>,
) -> Result<StatusCode, crate::responses::HttpError> {
    // println!("{:?}", claims);
    match store.items.write() {
        Ok(mut guard) => {
            let todo = Todo {
                id: get_next_id(),
                user_id: claims.id,
                title: todo.title,
                completed: false,
            };
            guard.push(todo);
            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(crate::responses::HttpError::Internal),
    }
}
