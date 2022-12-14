use axum::async_trait;
use axum::extract::{FromRequestParts, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use axum::http::request::Parts;
use jsonwebtoken as jwt;
use jwt::Validation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    // Required. Expiration time (as UTC timestamp)
    pub exp: usize,
    pub name: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = crate::responses::HttpError;
    async fn from_request_parts(
        parts: &mut Parts,
        // req: &mut RequestParts<B>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 要求Axum使用features = ["headers"]
        // 拿到bear token
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| crate::responses::HttpError::Auth)?;
        let key = jwt::DecodingKey::from_secret(crate::SECRET);
        // Decode bear token
        let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_e| crate::responses::HttpError::Auth)?;
        Ok(token.claims)
    }
}
