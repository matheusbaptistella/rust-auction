use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use axum::{http::StatusCode, extract::{Json, State}};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    #[serde(flatten)]
    pub user: User,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDB {
    #[serde(flatten)]
    pub user: User,
    pub hashed_password: String,
    pub salt: u8,
}

pub async fn sign_up(State(pool): State<PgPool>, Json(user): Json<CreateUser>) -> Result<(), StatusCode> {
    match crate::db::read_user(&pool, &user.user.email).await {
        Ok(result) => match result {
            Some(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            None => match crate::db::create_user(&pool, user).await {
                Ok(_) => Ok(()),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn sign_in(State(pool): State<PgPool>, Json(user): Json<User>) -> Result<Json<User>, StatusCode> {
    match crate::db::read_user(&pool, &user.email).await {
        Ok(result) => match result {
            Some(user) => Ok(Json(user)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn hash_password(password: &str) -> Result<>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
}