use std::env;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::backend::jwt_authentication::verify_token;

pub async fn extract_bearer_token(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set in .env");

    if let Some(auth_header) = req.headers().get(axum::http::header::AUTHORIZATION) {
        if let Ok(auth_header_value) = auth_header.to_str() {
            if let Some(token) = auth_header_value.strip_prefix("Bearer ") {
                match verify_token(secret, token.to_string()) {
                    Ok(claim) => {
                        // You can store the claim in the request's extensions if needed
                        req.extensions_mut().insert(claim);

                        // Continue to the next middleware/handler
                        return Ok(next.run(req).await);
                    }
                    Err(_) => return Err(StatusCode::UNAUTHORIZED), // Token verification failed
                }
            }
        }
    }

    // Respond with a 401 Unauthorized if the token is missing or invalid
    Err(StatusCode::UNAUTHORIZED)
}
