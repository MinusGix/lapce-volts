use async_session::{MemoryStore, SessionStore};
use axum::{extract::State, response::IntoResponse, TypedHeader};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

use crate::{db::models::User, state::SESSION_COOKIE_NAME};

pub async fn list(
    State(store): State<MemoryStore>,
    State(db_pool): State<Pool<AsyncPgConnection>>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(SESSION_COOKIE_NAME).unwrap();
    let session = store
        .load_session(cookie.to_string())
        .await
        .unwrap()
        .unwrap();
    let user_id: i32 = session.get("user_id").unwrap();

    let mut conn = db_pool.get().await.unwrap();
    let user = User::find(&mut conn, user_id).await.unwrap();
}