use axum::{
    async_trait,
    body::{boxed, Full},
    extract::Extension,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router, Server,
};
use jsonwebtoken as jwt;
use jwt::Validation;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    exp: usize, // Required. Expiration time (as UTC timestamp)
    name: String,
}

#[derive(Debug, Default, Clone)]
struct TotdoStore {
    // 允许多线程读写
    items: Arc<RwLock<Vec<Todo>>>,
}

#[derive(RustEmbed)]
#[folder = "my-app/build/"]
struct Assets;

// 实现IntoResponse
struct StaticFile<T>(pub T);
impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Assets::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref()) // mime type 如html/utf8之类的type
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from(format!("File not found: {}", path))))
                .unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    let store = TotdoStore::default();

    let app = Router::new()
        .route("/", get(index_handler))
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todo_handler)
                .layer(Extension(store)),
        )
        .route("/login", post(login_handler))
        .fallback(get(static_handler));
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

async fn index_handler() -> impl IntoResponse {
    // StaticFile("./index.html") // 等价于：
    static_handler("/index.html".parse().unwrap()).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    // 去掉uri的首use axum::handler::Handler;/
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

async fn todos_handler(
    claims: Claims,
    Extension(store): Extension<TotdoStore>,
) -> Result<Json<Vec<Todo>>, HttpError> {
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
        Err(_) => Err(HttpError::Internal),
    }
}

async fn create_todo_handler(
    claims: Claims,
    Json(todo): Json<CreateTodo>,
    Extension(store): Extension<TotdoStore>,
) -> Result<StatusCode, HttpError> {
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
        Err(_) => Err(HttpError::Internal),
    }
}

async fn login_handler(Json(_login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login validation: 如果用户正确传入email & passed，将为该用户生成一个Token
    let claims = Claims {
        id: 1,
        exp: get_epoch() + 14 * 24 * 60 * 60, // 14天后过期
        name: "Kaige Dong".to_string(),
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = HttpError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // 要求Axum使用features = ["headers"]
        // 拿到bear token
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| HttpError::Auth)?;
        let key = jwt::DecodingKey::from_secret(SECRET);
        // Decode bear token
        let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_e| HttpError::Auth)?;
        Ok(token.claims)
    }
}

#[derive(Debug)]
enum HttpError {
    Auth,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        // Axum已经实现的如果tupe每个字段都实现了into_response，那这个tupe也会实现into_response
        (code, msg).into_response()
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
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
