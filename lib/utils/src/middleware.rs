use axum::{
    http::Request,
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};

use crate::jwt::{verify_token, Claims};

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h,
        None => {
            println!("No Authorization header found");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            println!("Authorization header invalid string");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    if !auth_str.to_lowercase().starts_with("bearer ") {
        println!("Authorization header does not start with Bearer");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let token = auth_str[7..].trim();
        println!("Token: {}", token);
    let claims: Claims = match verify_token(token) {
        Ok(c) => c,
        Err(_) => {
            println!("Token verification failed");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    req.extensions_mut().insert(claims);

    next.run(req).await
}

pub async fn admin_only<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {

    let claims = match req.extensions().get::<Claims>() {
        Some(c) => c,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if claims.role != "admin" {
        return StatusCode::FORBIDDEN.into_response();
    }

    next.run(req).await
}