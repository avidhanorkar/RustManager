use crate::models::user_model::User;
use axum::{
    Json,
    extract::{State},
    http::{StatusCode},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use bson::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::{env};
use chrono::{Utc};


#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub user_id: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Serialize)]
pub struct AuthResponse {
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    msg: String,
    user_id: String,
    token: String,
}

pub async fn register(
    State(db): State<Database>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<AuthResponse>)> {
    let collection: Collection<User> = db.collection("user");

    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                msg: "The fields can't be empty".to_string(),
                id: None,
            }),
        ));
    };

    let filter = doc! {
        "email": &payload.email
    };

    match collection.find_one(filter).await {
        Ok(user_found) => {
            if user_found.is_some() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(AuthResponse {
                        msg: "The Email already exists".to_string(),
                        id: None,
                    }),
                ));
            }
        }
        Err(_) => {
            println!("Error while checking the existence of the email");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    msg: "Internal Server Error".to_string(),
                    id: None,
                }),
            ));
        }
    }

    let hashed = match hash(&payload.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            println!("Error in hashing the password, {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    msg: "Internal Server Error".to_string(),
                    id: None,
                }),
            ));
        }
    };

    let new_user = User {
        user_id: None,
        username: payload.username,
        email: payload.email,
        password: hashed,
        tasks: vec![],
    };

    match collection.insert_one(new_user).await {
        Ok(user_created) => {
            return Ok(Json(AuthResponse {
                msg: "User created Successfully".to_string(),
                id: Some(user_created.inserted_id.to_string()),
            }));
        }
        Err(e) => {
            println!("There is some error in inserting the new user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    msg: "Internal Server Error".to_string(),
                    id: None,
                }),
            ));
        }
    }
}

pub async fn login(
    State(db): State<Database>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<AuthResponse>)> {
    let jwt_secret: String = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    const TOKEN_EXPIRY: i64 = 24; //24hrs

    let collection: Collection<User> = db.collection("user");
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                msg: "All fields are required".to_string(),
                id: None,
            }),
        ));
    };

    let filter = doc! {
        "email": &payload.email
    };

    match collection.find_one(filter).await {
        Ok(Some(user_found)) => match verify(&payload.password, &user_found.password) {
            Ok(true) => {
                let now = Utc::now();
                let exp = now + chrono::Duration::hours(TOKEN_EXPIRY);

                // jwt claims 
                let claims = JWTClaims{
                    user_id: user_found.user_id.map(|id| id.to_string()).unwrap_or_default(),
                    username: user_found.username.clone(),
                    exp: exp.timestamp() as usize,
                    iat: now.timestamp() as usize,
                };

                match encode(
                    &Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()) ){
                        Ok(token) => {
                            return Ok(Json(LoginResponse{
                                msg: "Login Successful".to_string(),
                                user_id: user_found.user_id.map(|id| id.to_string()).unwrap_or_default(),
                                token,
                            }));
                        } Err(e) => {
                            println!("Error while encoding JWT: {}", e);
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(AuthResponse {
                                    msg: "Internal Server Error".to_string(),
                                    id: None,
                                }),
                            ));
                        }
                    }

            }
            Ok(false) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(AuthResponse {
                        msg: "Wrong Password".to_string(),
                        id: None,
                    }),
                ));
            }
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(AuthResponse {
                        msg: "Password Verification Failed".to_string(),
                        id: None,
                    }),
                ));
            }
        },
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthResponse {
                    msg: "User Not Found".to_string(),
                    id: None,
                }),
            ));
        }
        Err(e) => {
            println!("Error while searching for email: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    msg: "Password Verification Failed".to_string(),
                    id: None,
                }),
            ));
        }
    }
}
