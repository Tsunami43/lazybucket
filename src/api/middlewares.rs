use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::AppState;

pub async fn auth(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request.headers().get("AUTHORIZATION");

    // 401
    let Some(value) = auth_header else {
        tracing::warn!("Request without Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    };

    // login:password
    let parts: Vec<&str> = value.to_str().unwrap().splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (login, password) = (parts[0], parts[1]);

    if login != state.config.login || password != state.config.password {
        tracing::warn!("Invalid credentials for login: {}", login);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // всё ок — передаём запрос дальше
    Ok(next.run(request).await)
}
