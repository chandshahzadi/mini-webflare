use axum::{
    extract::{OriginalUri, Request},
    middleware::Next,
    response::Response,
};

use crate::{enums::Role, error::AppError, jwt::verify_jwt};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: i32,
    pub role: Role,
}

pub async fn verify_token(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = match verify_jwt(token) {
        Ok(c) => c,
        Err(_) => return Err(AppError::Unauthorized),
    };
    let user = AuthUser {
        id: claims.sub,
        role: claims.role,
    };

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

pub async fn verify_role(
    OriginalUri(uri): OriginalUri,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = uri.path();

    let user = req
        .extensions()
        .get::<AuthUser>()
        .ok_or(AppError::Unauthorized)?;

    if path.starts_with("/admin") && user.role != Role::Admin {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(req).await)
}
